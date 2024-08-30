use sqlx::{sqlite::SqlitePool, query};

pub async fn setup_database_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("Setting up ethereum_accounts table...");
    match query(
        "CREATE TABLE IF NOT EXISTS ethereum_accounts (
            address TEXT PRIMARY KEY,
            created_timestamp TEXT,
            creator_address TEXT,
            last_active_timestamp TEXT,
            type TEXT
        )"
    ).execute(pool).await {
        Ok(_) => println!("ethereum_accounts table created successfully."),
        Err(e) => eprintln!("Error creating ethereum_accounts table: {}", e),
    }

    println!("Setting up ethereum_transactions table...");
    match query(
        "CREATE TABLE IF NOT EXISTS ethereum_transactions (
            transaction_hash TEXT PRIMARY KEY,
            base_fee_per_gas NUMERIC,
            block_number INTEGER,
            contract_address TEXT,
            fees_burned NUMERIC,
            fees_rewarded NUMERIC,
            fees_saved NUMERIC,
            from_address TEXT,
            gas_limit NUMERIC,
            gas_price NUMERIC,
            gas_used NUMERIC,
            input TEXT,
            internal_failed_transaction_count INTEGER,
            internal_transaction_count INTEGER,
            log_count INTEGER,
            max_fee_per_gas NUMERIC,
            max_priority_fee_per_gas NUMERIC,
            nonce INTEGER,
            output TEXT,
            position INTEGER,
            timestamp TIMESTAMP,
            to_address TEXT,
            transaction_fee NUMERIC,
            type INTEGER,
            value NUMERIC
        )"
    ).execute(pool).await {
        Ok(_) => println!("ethereum_transactions table created successfully."),
        Err(e) => eprintln!("Error creating ethereum_transactions table: {}", e),
    }

    Ok(())
}