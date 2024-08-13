//! # Minigrep
//! `Minigrep` is a command line utility that searches for a string in files.

pub mod functions;
pub mod structs;

use std::{error::Error, fs};

pub use functions::search::{search, search_case_insensitive};
pub use structs::config::Config;

/// search the text in the given file and return the lines that contain the query string
///
/// # Example
///
/// ```
/// let config = minigrep::Config::new("rust".to_string() , "./poem.txt".to_string());
/// let result = minigrep::run(config).unwrap();
///
/// assert_eq!(result, ());
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

