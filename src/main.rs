use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

mod utils;
mod routes;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (*utils::contants::PORT).clone();
    let host_address =(*utils::contants::HOST_ADDRESS).clone();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(greet)
    })
    .bind((host_address, port))?
    .run()
    .await
}