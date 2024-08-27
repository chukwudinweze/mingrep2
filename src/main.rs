use std::{env, process};

use mingrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = mingrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = mingrep::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
