use std::env;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST_ADDRESS: String = set_host_address();
    pub static ref PORT: u16 = set_port();
}

fn set_host_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>()
}