use chrono;
use csv::Writer;
use serde_json::Value;
use sqlx::Column;
use sqlx::Row;
use sqlx::TypeInfo;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::error::Error;
use std::io::stdin;
use std::io::{self, Write};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut currect_dir = env::current_exe()
        .expect("Could not get currect ")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();

    currect_dir.push(".env");
    currect_dir.push(".env");
    currect_dir.push(".env");

    let env_path = Path::new(&currect_dir);
    dotenvy::from_path(env_path).ok();

    // Better error handling for environment variables
    let database_url =
        env::var("DATABASE_URL").map_err(|_| "Missing environment variable: DATABASE_URL")?;
    let database_type =
        env::var("DATABASE_TYPE").map_err(|_| "Missing environment variable: DATABASE_TYPE")?;
    let database_name =
        env::var("DATABASE_NAME").map_err(|_| "Missing environment variable: DATABASE_NAME")?;
    let database_port =
        env::var("DATABASE_PORT").map_err(|_| "Missing environment variable: DATABASE_PORT")?;
    let database_user =
        env::var("DATABASE_USER").map_err(|_| "Missing environment variable: DATABASE_USER")?;
    let database_password =
        env::var("DATABASE_PW").map_err(|_| "Missing environment variable: DATABASE_PW")?;
    let table_name_list_str =
        env::var("TABLE_NAME").map_err(|_| "Missing environment variable: TABLE_NAME")?;
    let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let table_name_list_str =
        env::var("TABLE_NAME").map_err(|_| "Missing environment variable: TABLE_NAME")?;
    let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let table_name_list_str =
        env::var("TABLE_NAME").map_err(|_| "Missing environment variable: TABLE_NAME")?;
    let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let table_name_list_str =
        env::var("TABLE_NAME").map_err(|_| "Missing environment variable: TABLE_NAME")?;
    let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let table_name_list_str =
        env::var("TABLE_NAME").map_err(|_| "Missing environment variable: TABLE_NAME")?;
    let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let output_path =
        env::var("OUTPUT_PATH").map_err(|_| "Missing environment variable: OUTPUT_PATH")?;
        let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let table_name_list_str =
        env::var("TABLE_NAME").map_err(|_| "Missing environment variable: TABLE_NAME")?;
    let csv_output_prefix = env::var("CSV_OUTPUT_PREFIX")
        .map_err(|_| "Missing environment variable: CSV_OUTPUT_PREFIX")?;
    let output_path =

    let full_database_url = format!(
        "{}://{}:{}@{}:{}/{}",
        database_type, database_user, database_password, database_url, database_port, database_name
    );
    let table_name_list: Vec<&str> = table_name_list_str.split(',').map(|s| s.trim()).collect();

    // Create MySQL connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&full_database_url)
        .await?;

    // Fetch all rows
    for each_table in table_name_list {
        export_table(&pool, &each_table, &output_path, &csv_output_prefix).await?;
    }

    // println!("Press Enter to exit...");
    // let _ = io::stdout().flush();
    // let mut input = String::new();
    // let _ = stdin().read_line(&mut input);

    Ok(())
}

fn create_output_path(output_path: &str, folder_name: &str) -> Result<String, Box<dyn Error>> {
    let final_path = if output_path == "" {
        // Create folder in current directory
        std::fs::create_dir_all(folder_name)?;
        folder_name.to_string()
    } else {
        // Create folder in specified output path
        let path = Path::new(output_path);
        let folder_path = path.join(folder_name);
        std::fs::create_dir_all(&folder_path)?;
        folder_path.to_string_lossy().to_string()
    };

    Ok(final_path)
}

async fn export_table(
    pool: &sqlx::MySqlPool,
    table_name: &str,
    output_path: &str,
    csv_output_prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let query = format!("SELECT * FROM {}", table_name);
    let rows = sqlx::query(&query).fetch_all(pool).await?;

    let now = chrono::Local::now();
    let folder_name = format!("{}", now.format("%Y-%m-%d_%H-%M-%S"));

    // Get the full path where CSV should be written
    let csv_directory = create_output_path(&output_path, &folder_name)?;

    let mut file_format = format!(
        "{}_{}_{}.csv",
        csv_output_prefix,
        table_name,
        now.format("%Y-%m-%d_%H-%M-%S")
    );

    if csv_output_prefix == "" {
        file_format = format!("{}_{}.csv", table_name, now.format("%Y-%m-%d_%H-%M-%S"));
    }

    let full_file_path = Path::new(&csv_directory).join(&file_format);

    // Open CSV writer
    let mut wtr = Writer::from_path(full_file_path)?;

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
                "bool" | "boolean" => row.try_get::<bool, _>(col.name()).map(|v| v.to_string()),
                "text" | "varchar" | "char" | "longtext" => row.try_get::<String, _>(col.name()),
                "json" => {
                    // Decode MySQL JSON type as serde_json::Value then stringify
                    row.try_get::<Value, _>(col.name()).map(|v| v.to_string())
                }
                "datetime" | "timestamp" => row
                    .try_get::<chrono::NaiveDateTime, _>(col.name())
                    .map(|v| v.to_string()),
                _ => Ok(String::from("[unsupported]")),
            }
            .unwrap_or_else(|_| String::from(""));

            record.push(value);
        }

        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    println!("✅ Table: {} was exported successfully.", table_name);

    Ok(())
}
