use dotenv::dotenv;
use std::env;

pub struct Config {
    pub transpose_api_key: Option<String>,
    pub save_as_csv: bool,
    pub save_as_sqlite: bool,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Config {
            transpose_api_key: env::var("TRANSPOSE_API_KEY").ok(),
            save_as_csv: env::var("SAVE_AS_CSV").unwrap_or_else(|_| "true".to_string()).to_lowercase() == "true",
            save_as_sqlite: env::var("SAVE_AS_SQLITE").unwrap_or_else(|_| "true".to_string()).to_lowercase() == "true",
        }
    }
}