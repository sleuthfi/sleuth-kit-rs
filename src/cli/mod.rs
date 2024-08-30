use colored::*;
use dialoguer::{theme::ColorfulTheme, Select, Input, MultiSelect};
use crate::config::Config;
use crate::api::transpose;
use crate::helpers::storage;
use crate::helpers::setup_schema;
use crate::helpers::postgres;
use sqlx::SqlitePool;
use sqlx::postgres::PgPool;
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

pub async fn run_cli(
    config: &mut Config,
    sqlite_pool: Option<&SqlitePool>,
    pg_pool: Option<&PgPool>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", SLEUTH_LOGO.cyan());
    println!("{}", "Sleuth Kit is a flexible and extensible toolkit for blockchain investigation and intelligence gathering.".green());

    if config.transpose_api_key().is_none() {
        println!("\n{}", "Warning: Transpose API key is not set. Please run 'setup' to set it.".red());
    }

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&["Setup", "Query Ethereum Account", "Query Ethereum Transactions", "Settings", "Exit"])
            .interact()?;

        match selection {
            0 => setup(config, sqlite_pool, pg_pool).await?,
            1 => query_ethereum_account(config, sqlite_pool, pg_pool).await?,
            2 => query_ethereum_transactions(config, sqlite_pool, pg_pool).await?,
            3 => settings_menu(config).await?,
            4 => break,
            _ => unreachable!(),
        }

        config.save()?;
    }

    Ok(())
}

async fn setup(config: &mut Config, sqlite_pool: Option<&SqlitePool>, pg_pool: Option<&PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "[Step 1] Setting up database schema".yellow());
    if let Some(pool) = sqlite_pool {
        setup_schema::setup_database_schema(pool).await?;
    }
    if let Some(pool) = pg_pool {
        postgres::setup_postgres_schema(pool).await?;
    }
    println!("{}", "Database schema set up successfully.".green());

    println!("{}", "[Step 2] Configuring API keys".yellow());
    if config.transpose_api_key().is_none() {
        set_transpose_api_key(config).await?;
    } else {
        println!("Transpose API key is already set. Skipping this step.");
    }

    println!("{}", "[Step 3] Configuring storage options".yellow());
    let storage_options = vec!["CSV", "SQLite", "PostgreSQL"];
    let storage_selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how you would like to store the data (use space to select multiple)")
        .items(&storage_options)
        .interact()?;

    config.save_as_csv = storage_selections.contains(&0);
    config.save_as_sqlite = storage_selections.contains(&1);
    config.save_as_postgres = storage_selections.contains(&2);

    if config.save_as_postgres {
        set_postgres_credentials(config).await?;
        if let Some(postgres_url) = &config.postgres_url() {
            match PgPool::connect(postgres_url).await {
                Ok(pool) => {
                    println!("Successfully connected to PostgreSQL.");
                    match postgres::setup_postgres_schema(&pool).await {
                        Ok(_) => println!("PostgreSQL schema created successfully."),
                        Err(e) => eprintln!("Error creating PostgreSQL schema: {}", e),
                    }
                },
                Err(e) => eprintln!("Failed to connect to PostgreSQL: {}", e),
            }
        }
    }

    Ok(())
}

async fn set_postgres_credentials(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_id: String = Input::new().with_prompt("Enter your PostgreSQL workspace ID").interact_text()?;
    let api_key: String = Input::new().with_prompt("Enter your PostgreSQL API key").interact_text()?;
    let region: String = Input::new().with_prompt("Enter your PostgreSQL region").interact_text()?;
    let database_name: String = Input::new().with_prompt("Enter your PostgreSQL database name").interact_text()?;
    let branch_name: String = Input::new().with_prompt("Enter your PostgreSQL branch name").interact_text()?;

    let postgres_url = format!(
        "postgresql://{}:{}@{}.sql.xata.sh:5432/{}:{}?sslmode=require",
        workspace_id, api_key, region, database_name, branch_name
    );

    // Save the PostgreSQL URL to the .env file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "POSTGRES_URL={}", postgres_url)?;

    env::set_var("POSTGRES_URL", &postgres_url);
    config.save_as_postgres = true;
    println!("{}", "PostgreSQL credentials saved successfully.".green());

    Ok(())
}

async fn query_ethereum_account(config: &Config, sqlite_pool: Option<&SqlitePool>, pg_pool: Option<&PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key().is_none() {
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
        if let Some(pool) = sqlite_pool {
            println!("{}", "[Step 3] Saving data to SQLite".yellow());
            storage::save_to_sqlite(pool, &account_data, "ethereum_accounts").await?;
        } else {
            println!("SQLite pool is not available. Skipping SQLite save.");
        }
    }

    if config.save_as_postgres {
        if let Some(pool) = pg_pool {
            println!("{}", "[Step 4] Saving data to PostgreSQL".yellow());
            match postgres::save_to_postgres(pool, &account_data, "ethereum_accounts").await {
                Ok(_) => println!("Data saved to PostgreSQL successfully."),
                Err(e) => eprintln!("Error saving data to PostgreSQL: {}", e),
            }
        } else {
            println!("PostgreSQL pool is not available. Skipping PostgreSQL save.");
        }
    }

    println!("{}", format!("\nRetrieved account data for address {}", address).green());
    Ok(())
}

async fn query_ethereum_transactions(config: &Config, sqlite_pool: Option<&SqlitePool>, pg_pool: Option<&PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    if config.transpose_api_key().is_none() {
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
        if let Some(pool) = sqlite_pool {
            println!("{}", "[Step 3] Saving data to SQLite".yellow());
            storage::save_to_sqlite(pool, &transactions, "ethereum_transactions").await?;
        } else {
            println!("SQLite pool is not available. Skipping SQLite save.");
        }
    }

    if config.save_as_postgres {
        if let Some(pool) = pg_pool {
            println!("{}", "[Step 4] Saving data to PostgreSQL".yellow());
            postgres::save_to_postgres(pool, &transactions, "ethereum_transactions").await?;
        } else {
            println!("PostgreSQL pool is not available. Skipping PostgreSQL save.");
        }
    }

    println!("{}", format!("\nRetrieved and processed {} transactions for address {}", total_transactions, address).green());
    Ok(())
}

async fn settings_menu(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nCurrent Settings:");
    println!("Transpose API Key: {}", if config.transpose_api_key().is_some() { "Set" } else { "Not Set" });
    println!("Save as CSV: {}", config.save_as_csv);
    println!("Save as SQLite: {}", config.save_as_sqlite);
    println!("Save as PostgreSQL: {}", config.save_as_postgres);
    println!("PostgreSQL URL: {}", if config.postgres_url().is_some() { "{workspace_id}:{api_key}@{region}.sql.xata.sh:5432/{database_name}:{branch_name}" } else { "Not Set" });

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Settings")
        .default(0)
        .items(&["Set Transpose API Key", "Configure Storage Options", "Back"])
        .interact()?;

    match selection {
        0 => set_transpose_api_key(config).await?,
        1 => configure_storage_options(config).await?,
        2 => return Ok(()),
        _ => unreachable!(),
    }

    Ok(())
}

async fn configure_storage_options(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let storage_options = vec!["CSV", "SQLite", "PostgreSQL"];
    let mut initial_selection = vec![false, false, false];
    if config.save_as_csv { initial_selection[0] = true; }
    if config.save_as_sqlite { initial_selection[1] = true; }
    if config.save_as_postgres { initial_selection[2] = true; }

    let storage_selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how you would like to store the data (use space to select multiple)")
        .items(&storage_options)
        .defaults(&initial_selection)
        .interact()?;

    config.save_as_csv = storage_selections.contains(&0);
    config.save_as_sqlite = storage_selections.contains(&1);
    config.save_as_postgres = storage_selections.contains(&2);

    if config.save_as_postgres && config.postgres_url().is_none() {
        set_postgres_credentials(config).await?;
    }

    config.save()?;
    Ok(())
}

async fn set_transpose_api_key(_config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = Input::new().with_prompt("Enter your Transpose API key").interact_text()?;
    
    // Save the API key to the .env file
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(".env")?;

    writeln!(file, "TRANSPOSE_API_KEY={}", api_key)?;

    println!("{}", "Transpose API key saved successfully.".green());

    Ok(())
}