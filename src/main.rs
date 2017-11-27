extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate app_dirs;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate time;

use clap::{App, Arg};

use std::io::{Read, Write, stdout};
use std::path::PathBuf;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

use app_dirs::*;
const APP_INFO: AppInfo = AppInfo {
    name: "cryptoticker",
    author: "Josh Leverette",
};

#[derive(Debug)]
struct StrError(String);

impl std::convert::From<reqwest::Error> for StrError {
    fn from(error: reqwest::Error) -> Self {
        StrError(format!("{:#?}", error))
    }
}

impl std::convert::From<serde_json::Error> for StrError {
    fn from(error: serde_json::Error) -> Self {
        StrError(format!("{:#?}", error))
    }
}

impl std::convert::From<std::io::Error> for StrError {
    fn from(error: std::io::Error) -> Self {
        StrError(format!("{:#?}", error))
    }
}

impl std::convert::From<String> for StrError {
    fn from(error: String) -> Self {
        StrError(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Currency {
    id: String,
    name: String,
    symbol: String,
    rank: String,

    price_usd: Option<String>,
    price_btc: Option<String>,

    #[serde(rename = "24h_volume_usd")]
    volume_usd_24h: Option<String>,

    market_cap_usd: Option<String>,
    available_supply: Option<String>,
    total_supply: Option<String>,
    percent_change_1: Option<String>,
    percent_change_24: Option<String>,
    percent_change_7: Option<String>,
    last_updated: Option<String>,
}

fn fetch_ticker(
    name: &str,
    cache_file: Option<PathBuf>,
    debug: bool,
) -> Result<Currency, StrError> {
    if debug {
        println!("retrieving latest for {}", name);
    }
    let url = "https://api.coinmarketcap.com/v1/ticker/".to_string() + &name;
    let mut resp = reqwest::get(url.as_str())?;
    if !resp.status().is_success() {
        return Err(format!("Ticker ID {} not valid.", name))?;
    }

    let mut content = String::new();
    resp.read_to_string(&mut content)?;

    let mut tickers: Vec<Currency> = serde_json::from_str(&content)?;

    let ticker = tickers.remove(0);

    if let Some(cache_file) = cache_file {
        if debug {
            println!("{} stored in cache", cache_file.display());
        }
        let file = fs::File::create(cache_file)?;
        serde_json::to_writer(file, &ticker)?;
    }

    Ok(ticker)
}

fn print_ticker(name: String, cache: bool, debug: bool) -> Result<(), StrError> {
    let cache_dir = app_root(AppDataType::UserCache, &APP_INFO).expect(
        "Could not find or create the cache directory",
    );

    let ticker: Currency = if !cache {
        fetch_ticker(&name, None, debug)?
    } else {
        let cache_file = cache_dir.join(format!("{}{}", name, ".json"));
        let metadata = fs::metadata(&cache_file);
        match metadata {
            Ok(metadata) => {
                match metadata.modified().unwrap().elapsed() {
                    Ok(elapsed) if elapsed < Duration::from_secs(1800) => {
                        if debug {
                            println!(
                                "{} pulled from cache, {} seconds left until cache goes cold.",
                                cache_file.display(),
                                (Duration::from_secs(1800) - elapsed).as_secs()
                            );
                        }
                        let file = fs::File::open(cache_file)?;
                        serde_json::from_reader(file)?
                    }
                    _ => fetch_ticker(&name, Some(cache_file), debug)?,
                }
            }
            _ => fetch_ticker(&name, Some(cache_file), debug)?,
        }
    };

    let price = ticker.price_usd.unwrap_or("null".to_string());

    let short_name;
    if name == "ethereum" {
        short_name = "eth".to_string();
    } else if name == "bitcoin" {
        short_name = "btc".to_string();
    } else {
        short_name = name;
    }

    print!("{}:{} ", short_name, price);

    return Ok(());
}

fn main() {
    let matches = App::new("cryptoticker")
        .version(crate_version!())
        .about("Shows cryptoprices in a convenient ticker format for tmux")
        .author("Josh Leverette")
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("Sets the ticker to repeat on a time interval"),
        )
        .arg(
            Arg::with_name("interval-time")
                .short("t")
                .long("interval-time")
                .help("Sets the time interval for the ticker.")
                .default_value("90"),
        )
        .arg(Arg::with_name("debug").short("d").long("debug").help(
            "Shows verbose error messages",
        ))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Shows verbose error messages")
                .hidden(true),
        )
        .args_from_usage(
            "<TICKER>...  'The name of the currency, like bitcoin or ethereum'",
        )
        .get_matches();

    let debug = matches.is_present("debug") || matches.is_present("verbose");
    let interval = matches.is_present("interval");

    let time = value_t!(matches, "interval-time", u64).unwrap_or_else(|err| {
        println!("{}", err.description());
        std::process::exit(1)
    });

    let tickers: Vec<_> = matches.values_of("TICKER").unwrap().collect();

    loop {
        for arg in &tickers {
            let _ = print_ticker(arg.to_string(), !interval, debug).map_err(|err| if debug {
                println!("{}", err.0)
            } else {
                print!("{}:error ", arg)
            });
        }
        print!("\x08");
        stdout().flush().unwrap();
        if !interval {
            break;
        } else {
            print!("\r");
        }
        sleep(Duration::from_secs(time));
    }
}
