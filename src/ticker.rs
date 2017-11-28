use std::io::Read;
use std::path::PathBuf;
use std::fs;
use std::time::Duration;
use errors::StrError;
use reqwest;
use serde_json;
use app_dirs::*;

use super::APP_INFO;

#[derive(Debug, Serialize, Deserialize)]
struct Currency {
    id: String,
    name: String,
    symbol: String,
    rank: String,

    price_usd: Option<String>,
    price_btc: Option<String>,

    #[serde(rename = "24h_volume_usd")] volume_usd_24h: Option<String>,

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

pub fn print_ticker(name: String, cache: bool, debug: bool) -> Result<(), StrError> {
    let cache_dir = app_root(AppDataType::UserCache, &APP_INFO)
        .expect("Could not find or create the cache directory");

    let ticker: Currency = if !cache {
        fetch_ticker(&name, None, debug)?
    } else {
        let cache_file = cache_dir.join(format!("{}{}", name, ".json"));
        let metadata = fs::metadata(&cache_file);
        match metadata {
            Ok(metadata) => match metadata.modified().unwrap().elapsed() {
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
            },
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
