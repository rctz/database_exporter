# Database Exporter

A blazing-fast, asynchronous Rust command-line tool to export MySQL tables to CSV files with automatic timestamping.

## Features

- **Asynchronous & non-blocking** – built with Tokio for maximum throughput
- **Connection pooling** via SQLx for efficient resource usage
- **Zero-config setup** – just drop a `.env` file with your settings (see below)
- **Full table export** to CSV with column headers preserved
- **Automatic data-type handling** (integers, floats, booleans, text, dates & timestamps)
- **Timestamped output files** for easy versioning
- **Multi-table export** – supply a comma-separated list and get a CSV per table
- **Customizable file naming & output directory** – tweak `CSV_OUTPUT_PREFIX` & `OUTPUT_PATH`
- **Pre-compiled Windows binary** (`db_deleter.exe`) included for quick start
- Small binary size & minimal runtime dependencies

## Prerequisites

- Rust (edition 2024)
- MySQL database
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/rctz/db_exporter.git
   cd db_exporter
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

The binary will be available at `target/release/db_exporter`.

**Windows users:** A pre-built binary (`db_deleter.exe`) is included in the repository root – no Rust toolchain required. Simply double-click or run it from PowerShell.

## Configuration (.env)

1. Copy the example environment file:

   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file with your database connection & export settings:

   ```env
   # Database driver (currently supports only MySQL)
   DATABASE_TYPE=mysql

   # Connection details
   DATABASE_URL=localhost       # host or IP
   DATABASE_PORT=3306           # port
   DATABASE_NAME=my_database    # schema / database name
   DATABASE_USER=my_user        # username
   DATABASE_PW=secret           # password

   # Comma-separated list of tables to export
   TABLE_NAME=robots,stations

   # Customize CSV filenames
   CSV_OUTPUT_PREFIX=export

   # Directory where the timestamped folder will be created (leave blank for CWD)
   OUTPUT_PATH=
   ```

## Usage

```bash
# Run with Cargo (requires Rust toolchain)
cargo run --release

# Or execute the compiled binary directly
./target/release/db_exporter             # Linux / macOS
.\db_deleter.exe                         # Windows (pre-built)
```

The tool will create a CSV file with the specified base name and a timestamp, e.g. `output_2025-07-15_15-23-05.csv`.

## Supported Data Types

- Integer types (INT, INTEGER, BIGINT, SMALLINT)
- Floating point numbers (FLOAT, DOUBLE, DECIMAL)
- Boolean values (BOOL, BOOLEAN)
- Text (TEXT, VARCHAR, CHAR, LONGTEXT)
- Date/Time (DATETIME, TIMESTAMP)
- JSON (JSON)

## Error Handling

- The tool will exit with an error if:
  - The database connection fails
  - Any of the specified tables don't exist
  - There are permission issues
  - The output files cannot be created

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
