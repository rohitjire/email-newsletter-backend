use std::{error::Error, fmt::Display};

use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

mod routes;
mod utils;

#[derive(Debug)]
struct MainError {
    message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error: {}", self.message)
    }
}


impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), MainError> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::contants::PORT).clone();
    let host_address = (*utils::contants::HOST_ADDRESS).clone();
    let database_url: String = (*utils::contants::DATABASE_URL).clone();

    let db: DatabaseConnection =
        Database::connect(database_url)
            .await
            .map_err(|err| MainError {
                message: err.to_string(),
            })?;
    Migrator::up(&db, None).await.map_err(|err| MainError {
        message: err.to_string(),
    })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
    })
    .bind((host_address, port))
    .map_err(|err| MainError {message : err.to_string()})?
    .run()
    .await
    .map_err(|err| MainError {message : err.to_string()})
}
