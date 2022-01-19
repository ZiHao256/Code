use minigrep::Config;
use std::{env, process};

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|error| {
        eprintln!("Parsing error: {}", error);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
    };
}
