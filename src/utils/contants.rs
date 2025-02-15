/// Module for storing application constants.
///
/// This module loads configuration values from environment variables
/// and provides them as static references using `lazy_static!`.
use std::env;

use lazy_static::lazy_static;

lazy_static! {
    /// Host address for the application server.
    pub static ref HOST_ADDRESS: String = set_host_address();
    /// Port number for the application server.
    pub static ref PORT: u16 = set_port();
    /// Database connection URL.
    pub static ref DATABASE_URL: String = set_database_url();
    /// Secret key for JWT authentication.
    pub static ref SECRET: String = set_secret();
}

/// Retrieves the host address from the environment variables.
/// Defaults to `127.0.0.1` if not set.
fn set_host_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or("127.0.0.1".to_string())
}

/// Retrieves the port number from the environment variables.
/// Expects a valid u16 value.
fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .unwrap()
        .parse::<u16>()
        .expect("Can't parse the port")
}

/// Retrieves the database URL from the environment variables.
fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

/// Retrieves the secret key from the environment variables.
fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET").unwrap()
}
