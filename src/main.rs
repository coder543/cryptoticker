extern crate app_dirs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate time;

use clap::{App, Arg};

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

use app_dirs::*;
const APP_INFO: AppInfo = AppInfo {
    name: "cryptoticker",
    author: "Josh Leverette",
};

mod errors;

mod ticker;
use ticker::print_ticker;

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
                .default_value("300"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Shows verbose error messages"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Shows verbose error messages")
                .hidden(true),
        )
        .arg(
            Arg::with_name("clear-cache")
                .long("clear-cache")
                .help("clears cryptoticker's cache directory"),
        )
        .args_from_usage("[TICKER]...  'The name of the currency, like bitcoin or ethereum'")
        .get_matches();

    let debug = matches.is_present("debug") || matches.is_present("verbose");
    let interval = matches.is_present("interval");
    let clear_cache = matches.is_present("clear-cache");

    if clear_cache {
        let _ = app_root(AppDataType::UserCache, &APP_INFO).map(|dir| {
            println!("removing cache directory {}", dir.display());
            ::std::fs::remove_dir_all(dir).unwrap()
        });
    }

    let time = value_t!(matches, "interval-time", u64).unwrap_or_else(|err| {
        println!("{}", err.description());
        std::process::exit(1)
    });

    let tickers = matches
        .values_of("TICKER")
        .map(|iter| iter.collect())
        .unwrap_or(vec![]);

    loop {
        for arg in &tickers {
            let _ = print_ticker(arg.to_string(), !interval, debug).map_err(|err| {
                if debug {
                    println!("{}", err.0)
                } else {
                    print!("{}:error ", arg)
                }
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
