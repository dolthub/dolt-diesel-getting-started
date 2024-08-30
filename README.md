This is a demo of using Diesel ORM with the Dolt database.

Detailed instructions and walkthrough of code in this [blog](https://github.com/dolthub/dolt-diesel-getting-started)

# Quick Start
1. Install Dolt, Rust, and Diesel CLI on your system.
2. Clone this repository.
3. Run `dolt sql-server` somewhere
4. `echo "DATABASE_URL=<db_connection_string>" > .env`
5. Export the MySQL client library path and version
    - `export MYSQLCLIENT_LIB_DIR=<path_to_mysql_lib>`
    - `export MYSQLCLIENT_VERSION=<mysql_version>`
6. `diesel setup`
7. `cargo run`
