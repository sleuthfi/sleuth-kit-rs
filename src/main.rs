mod api;
mod cli;
mod config;
mod helpers;

use config::Config;
use sqlx::sqlite::SqlitePool;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    
    fs::create_dir_all("data/sqlite")?;
    
    let database_path = Path::new("data/sqlite/blocks.db");
    let database_url = format!("sqlite:{}", database_path.display());

    if !database_path.exists() {
        fs::File::create(database_path)?;
        println!("Created new database file.");
    }

    let pool = SqlitePool::connect(&database_url).await.map_err(|e| {
        eprintln!("Failed to connect to the database: {}", e);
        e
    })?;

    helpers::setup_schema::setup_database_schema(&pool).await?;

    cli::run_cli(&mut config, &pool).await?;

    Ok(())
}