use serde_json::Value;
use std::fs::OpenOptions;
use csv::Writer;
use sqlx::{sqlite::SqlitePool};
use std::fs;
use std::path::Path;

pub async fn save_to_csv(data: &[Value], filepath: &str, fieldnames: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to save data to CSV at: {}", filepath);
    
    // Create the directory if it doesn't exist
    if let Some(parent) = Path::new(filepath).parent() {
        fs::create_dir_all(parent)?;
        println!("Directory created or already exists: {:?}", parent);
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filepath)?;
    println!("File opened successfully: {}", filepath);

    let mut wtr = Writer::from_writer(file);
    
    wtr.write_record(fieldnames)?;
    
    for record in data {
        let mut row = Vec::new();
        for field in fieldnames {
            row.push(record[field].as_str().unwrap_or("").to_string());
        }
        wtr.write_record(&row)?;
    }
    
    wtr.flush()?;
    println!("Data successfully written to CSV");
    Ok(())
}

pub async fn save_to_sqlite(pool: &SqlitePool, data: &[Value], table_name: &str) -> Result<(), sqlx::Error> {
    for record in data {
        let columns = record.as_object().unwrap().keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        let placeholders = (0..record.as_object().unwrap().len()).map(|i| format!("${}", i + 1)).collect::<Vec<_>>().join(", ");
        
        let sql = format!("INSERT OR REPLACE INTO {} ({}) VALUES ({})", table_name, columns, placeholders);
        
        let mut query = sqlx::query(&sql);
        for value in record.as_object().unwrap().values() {
            query = query.bind(value.as_str().unwrap_or(""));
        }
        
        query.execute(pool).await?;
    }
    
    Ok(())
}