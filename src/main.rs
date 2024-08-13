use minigrep_david20019::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep_david20019::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
