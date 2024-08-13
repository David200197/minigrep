use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Self {
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let config = Self::new(query, file_path);

        Ok(config)
    }
}
