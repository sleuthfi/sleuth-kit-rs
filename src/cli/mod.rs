use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use crate::config::Config;
use crate::api::transpose;
use crate::helpers::storage;
use crate::helpers::setup_schema;
use sqlx::SqlitePool;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

const SLEUTH_LOGO: &str = r#"
███████╗██╗     ███████╗██╗   ██╗████████╗██╗  ██╗    ██╗  ██╗██╗████████╗
██╔════╝██║     ██╔════╝██║   ██║╚══██╔══╝██║  ██║    ██║ ██╔╝██║╚══██╔══╝
███████╗██║     █████╗  ██║   ██║   ██║   ███████║    █████╔╝ ██║   ██║   
╚════██║██║     ██╔══╝  ██║   ██║   ██║   ██╔══██║    ██╔═██╗ ██║   ██║   
███████║███████╗███████╗╚██████╔╝   ██║   ██║  ██║    ██║  ██╗██║   ██║   
╚══════╝╚══════╝╚══════╝ ╚═════╝    ╚═╝   ╚═╝  ╚═╝    ╚═╝  ╚═╝╚═╝   ╚═╝   
"#;

pub async fn run_cli(config: &mut Config, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", SLEUTH_LOGO.cyan());
    println!("{}", "Sleuth Kit is a flexible and extensible toolkit for blockchain investigation and intelligence gathering.".green());

    if config.transpose_api_key.is_none() {
        println!("\n{}", "Warning: Transpose API key is not set. Please run 'setup' to set it.".red());
    }

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&["Setup", "Query Ethereum Account", "Query Ethereum Transactions", "Set Transpose API Key", "Exit"])
            .interact()?;

        match selection {
            0 => setup(config, pool).await?,
            1 => query_ethereum_account(config, pool).await?,
            2 => query_ethereum_transactions(config, pool).await?,
            3 => set_transpose_api_key(config).await?,
            4 => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

async fn setup(config: &mut Config, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "[Step 1] Setting up database schema".yellow());
    setup_schema::setup_database_schema(pool).await?;
    println!("{}", "Database schema set up successfully.".green());

    println!("{}", "[Step 2] Configuring API keys".yellow());
    set_transpose_api_key(config).await?;

    Ok(())
}

async fn set_transpose_api_key(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let key: String = Input::new()
        .with_prompt("Please enter your Transpose API key")
        .interact_text()?;

    // Save the API key to the .env file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(".env")?;

    writeln!(file, "TRANSPOSE_API_KEY={}", key)?;

    env::set_var("TRANSPOSE_API_KEY", &key);
    config.transpose_api_key = Some(key);
    println!("{}", "Transpose API key saved successfully.".green());

    Ok(())
}

async fn query_ethereum_account(config: &Config, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key.is_none() {
        println!("{}", "Transpose API key is not set. Please run 'setup' to set it.".red());
        return Ok(());
    }

    let address: String = Input::new()
        .with_prompt("Enter Ethereum address")
        .interact_text()?;

    println!("{}", "[Step 1] Querying Ethereum account details".yellow());
    let account_data = transpose::query_ethereum_account(config, &address).await?;

    if config.save_as_csv {
        println!("{}", "[Step 2] Saving data to CSV".yellow());
        storage::save_to_csv(&account_data, "data/csv/ethereum-accounts.csv", &["address", "created_timestamp", "creator_address", "last_active_timestamp", "type"]).await?;
    }

    if config.save_as_sqlite {
        println!("{}", "[Step 3] Saving data to SQLite".yellow());
        storage::save_to_sqlite(pool, &account_data, "ethereum_accounts").await?;
    }

    println!("{}", format!("\nRetrieved account data for address {}", address).green());
    Ok(())
}

async fn query_ethereum_transactions(config: &Config, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key.is_none() {
        println!("{}", "Transpose API key is not set. Please run 'setup' to set it.".red());
        return Ok(());
    }

    let address: String = Input::new()
        .with_prompt("Enter Ethereum address")
        .interact_text()?;

    println!("{}", "[Step 1] Querying Ethereum transactions".yellow());
    let transactions = transpose::query_ethereum_transactions(config, &[address.clone()]).await?;

    if transactions.is_empty() {
        println!("{}", "No transactions found for the provided address".yellow());
        return Ok(());
    }

    let total_transactions = transactions.len();

    if config.save_as_csv {
        println!("{}", "[Step 2] Saving data to CSV".yellow());
        storage::save_to_csv(&transactions, "data/csv/ethereum-transactions.csv", &["transaction_hash", "base_fee_per_gas", "block_number", "contract_address", "fees_burned", "fees_rewarded", "fees_saved", "from_address", "gas_limit", "gas_price", "gas_used", "input", "internal_failed_transaction_count", "internal_transaction_count", "log_count", "max_fee_per_gas", "max_priority_fee_per_gas", "nonce", "output", "position", "timestamp", "to_address", "transaction_fee", "type", "value"]).await?;
    }

    if config.save_as_sqlite {
        println!("{}", "[Step 3] Saving data to SQLite".yellow());
        storage::save_to_sqlite(pool, &transactions, "ethereum_transactions").await?;
    }

    println!("{}", format!("\nRetrieved and processed {} transactions for address {}", total_transactions, address).green());
    Ok(())
}