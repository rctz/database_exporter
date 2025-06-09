use dotenvy::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;
use sqlx::Column;
use csv::Writer;
use std::error::Error;
use sqlx::TypeInfo;
use chrono;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let now = chrono::Local::now();
    
    // Load environment variables from `.env`
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let table_name = env::var("TABLE_NAME")?;
    let csv_output = env::var("CSV_OUTPUT")?;

    // Create MySQL connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Fetch all rows
    let query = format!("SELECT * FROM {}", table_name);
    let rows = sqlx::query(&query).fetch_all(&pool).await?;

    // Open CSV writer
    let mut wtr = Writer::from_path(format!("{}_{}.csv", csv_output, now.format("%Y-%m-%d_%H-%M-%S")))?;

    if let Some(row) = rows.get(0) {
        // Write CSV headers
        let columns = row.columns();
        let headers: Vec<&str> = columns.iter().map(|c| c.name()).collect();
        wtr.write_record(&headers)?;
    }

    for row in rows {
        let columns = row.columns();
        let mut record = vec![];
    
        for col in columns {
            let type_info = col.type_info().name().to_lowercase();
            let value = match type_info.as_str() {
                "int" | "integer" | "bigint" | "smallint" => {
                    row.try_get::<i64, _>(col.name()).map(|v| v.to_string())
                }
                "float" | "double" | "decimal" => {
                    row.try_get::<f64, _>(col.name()).map(|v| v.to_string())
                }
                "bool" | "boolean" => {
                    row.try_get::<bool, _>(col.name()).map(|v| v.to_string())
                }
                "text" | "varchar" | "char" | "longtext" => {
                    row.try_get::<String, _>(col.name())
                }
                "datetime" | "timestamp" => {
                    row.try_get::<chrono::NaiveDateTime, _>(col.name())
                        .map(|v| v.to_string())
                }
                _ => Ok(String::from("[unsupported]")),
            }
            .unwrap_or_else(|_| String::from(""));
    
            record.push(value);
        }
    
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    println!("✅ Data exported to CSV successfully.");

    Ok(())
}