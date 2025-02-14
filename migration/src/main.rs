/// Main entry point for the Rust email newsletter backend.
/// This initializes and runs the migration CLI using `sea_orm_migration`.
use sea_orm_migration::prelude::*;

/// Main asynchronous function.
/// 
/// This function delegates to the CLI handler to run database migrations.
/// Uses `async-std` for asynchronous execution.
#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
