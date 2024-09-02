mod api;
mod cli;
mod config;
mod helpers;
mod db;
mod models;
mod utils;
mod ui;

use config::Config;
use sqlx::sqlite::SqlitePool;
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    let sqlite_pool = if config.save_as_sqlite {
        match SqlitePool::connect("sqlite:data/sqlite/sleuth.db").await {
            Ok(pool) => {
                println!("Successfully connected to SQLite database.");
                Some(pool)
            },
            Err(e) => {
                eprintln!("Error connecting to SQLite: {}", e);
                None
            }
        }
    } else {
        None
    };

    let pg_pool = if config.save_as_postgres {
        if let Some(postgres_url) = config.postgres_url() {
            match PgPool::connect(&postgres_url).await {
                Ok(pool) => {
                    println!("Successfully connected to PostgreSQL database.");
                    Some(pool)
                },
                Err(e) => {
                    eprintln!("Error connecting to PostgreSQL: {}", e);
                    None
                }
            }
        } else {
            eprintln!("PostgreSQL URL not set");
            None
        }
    } else {
        None
    };

    cli::run_cli(&mut config, sqlite_pool.as_ref(), pg_pool.as_ref()).await?;

    Ok(())
}