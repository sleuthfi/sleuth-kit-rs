<h1 align="center">Sleuth Kit 🔍</h1>

<p align="center">
  <b>A Flexible OSINT Toolkit for Blockchain Investigations.</b>
</p>

<p align="center">
 <a href="#overview">Overview</a> •
 <a href="#features">Features</a> •
 <a href="#project-structure">Project Structure</a> •
 <a href="#installation">Installation</a> •
 <a href="#usage">Usage</a> •
 <a href="#configuration">Configuration</a> •
 <a href="#contributing">Contributing</a> •
 <a href="#license">License</a>
</p>

<div align="center">

![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)

</div>

## Overview

Sleuth Kit is a flexible and extensible OSINT toolkit designed for blockchain investigations and intelligence gathering. It provides a suite of tools for compiling your own Data Lake of blockchain data and building custom tools for blockchain intelligence.

### Current Roadmap

- [ ] Add support for bitcoin
- [ ] Add support for other EVM chains (Base, Arbitrum, Optimism, etc.)
- [ ] Add support for Solana
- [ ] Implement cross-chain timing analysis
- [ ] Expand data tools:
  - [ ] Address monitoring
  - [ ] Smart contract event logging

> [!NOTE]
> This is the Rust version of the original [Python Sleuth Kit](https://github.com/sleuthfi/sleuth-kit).

## How It Works

```mermaid
sequenceDiagram
participant User
participant CLI
participant Config
participant API
participant Helpers
participant Database
participant CSV
participant PostgreSQL
User->>CLI: Run sleuth command
CLI->>Config: Load configuration
CLI->>Helpers: Setup database schema
Helpers->>Database: Create SQLite tables if not exist
Helpers->>PostgreSQL: Create PostgreSQL tables if not exist
loop Main Menu
User->>CLI: Choose option
alt Setup
CLI->>User: Prompt for Transpose API key
User->>CLI: Enter Transpose API key
CLI->>Config: Save Transpose API key
CLI->>User: Prompt for storage options
User->>CLI: Select storage options
CLI->>Config: Save storage preferences
alt PostgreSQL selected
CLI->>User: Prompt for PostgreSQL credentials
User->>CLI: Enter PostgreSQL credentials
CLI->>Config: Save PostgreSQL URL
end
CLI-->>User: Display setup success message
else Query Ethereum Account
CLI->>User: Prompt for Ethereum address
User->>CLI: Enter Ethereum address
CLI->>API: query_ethereum_account(address)
API->>API: load_sql_query('ethereum_accounts.sql')
API->>API: query_transpose(sql_query, params)
API-->>CLI: Return account data
alt Save as CSV
CLI->>Helpers: save_to_csv(data, 'ethereum-accounts.csv', fields)
Helpers->>CSV: Write data
end
alt Save as SQLite
CLI->>Helpers: save_to_sqlite(data, 'ethereum_accounts')
Helpers->>Database: Insert or update data
end
alt Save as PostgreSQL
CLI->>Helpers: save_to_postgres(data, 'ethereum_accounts')
Helpers->>PostgreSQL: Insert or update data
end
CLI-->>User: Display result message
else Query Ethereum Transactions
CLI->>User: Prompt for Ethereum address
User->>CLI: Enter Ethereum address
CLI->>API: query_ethereum_transactions(address)
API->>API: load_sql_query('ethereum_transactions.sql')
loop Fetch all transactions
API->>API: query_transpose(sql_query, params)
end
API-->>CLI: Return all transactions
alt Save as CSV
CLI->>Helpers: save_to_csv(data, 'ethereum-transactions.csv', fields)
Helpers->>CSV: Write transactions
end
alt Save as SQLite
CLI->>Helpers: save_to_sqlite(data, 'ethereum_transactions')
Helpers->>Database: Insert or update transactions
end
alt Save as PostgreSQL
CLI->>Helpers: save_to_postgres(data, 'ethereum_transactions')
Helpers->>PostgreSQL: Insert or update transactions
end
CLI-->>User: Display result message
else Settings
CLI->>User: Display current settings
User->>CLI: Choose setting to modify
CLI->>Config: Update configuration
CLI-->>User: Display updated settings
else Exit
CLI-->>User: Exit program
end
```

## Features

- 🔍 **Ethereum Account Queries**: Retrieve detailed information about Ethereum accounts
- 💼 **Transaction Analysis**: Fetch and analyze Ethereum transactions
- 💾 **Flexible Data Storage**: Save data in CSV, SQLite, and PostgreSQL formats
- 🔧 **Extensible Framework**: Easily add support for more blockchains and data sources
- 🖥️ **Interactive CLI**: User-friendly command-line interface with a settings menu
- 🔐 **Secure Configuration**: Environment-based configuration for API keys and database credentials
- 🔄 **Configurable Storage Options**: Choose between CSV, SQLite, and PostgreSQL storage
- 📊 **PostgreSQL Support**: Efficiently store and manage large datasets

## Project Structure

```
sleuth-kit/
├── assets/
│   └── sleuth-kit-logo.png
├── src/
│   ├── api/
│   │   ├── mod.rs
│   │   └── transpose.rs
│   ├── cli/
│   │   └── mod.rs
│   ├── config/
│   │   └── mod.rs
│   ├── helpers/
│   │   ├── mod.rs
│   │   ├── setup_schema.rs
│   │   └── storage.rs
│   ├── sql/
│   │   ├── ethereum_accounts.sql
│   │   └── ethereum_transactions.sql
│   └── main.rs
├── data/
│   ├── csv/
│   └── sqlite/
├── Cargo.toml
├── .gitattributes
├── .gitignore
├── LICENSE
└── README.md
```

## Installation

### Prerequisites

Ensure you have Rust installed on your system. If not, install it using the following methods:

#### macOS

1. Install Homebrew

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

2. Install Rust

```bash
brew install rust
```


#### Windows
Download and run the Rust installer from the [official website](https://www.rust-lang.org/tools/install).

### Installing Sleuth Kit

You can install the latest version of the Sleuth Kit using Cargo:

```bash
cargo install sleuth-kit
```


## Usage

1. Run the Sleuth Kit CLI:
   ```
   sleuth
   ```

2. Follow the interactive prompts to:
   - Set up the database schema
   - Configure your Transpose API key
   - Query Ethereum account data
   - Query Ethereum transaction data

## Configuration

Ensure that the `.env` file is set up with the following variables:

- `TRANSPOSE_API_KEY`: Your Transpose API key
- `SAVE_AS_CSV`: Set to "true" to save data as CSV (default: true)
- `SAVE_AS_SQLITE`: Set to "true" to save data in SQLite (default: true)

## Contributing

If you'd like to contribute to the Sleuth Kit project, follow these steps:

1. Fork the repository on GitHub.

2. Clone your forked repository:
   ```
   git clone https://github.com/yourusername/sleuth-kit.git
   cd sleuth-kit
   ```

3. Install dependencies:
   ```
   cargo build
   ```

4. Set up the environment variables:
   Create a `.env` file in the project root and add the necessary variables.

5. Run the project:
   ```
   cargo run
   ```

5. Make your changes and create a pull request with a clear description of the changes and their purpose.

## License

This project is licensed under the [GNU Affero General Public License v3.0](LICENSE).
