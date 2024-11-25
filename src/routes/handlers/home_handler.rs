use actix_web::{get, web, Responder};

#[get("/ping/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
