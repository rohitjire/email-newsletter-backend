use actix_web::web;

use super::health_handler;

pub fn config(config: &mut web::ServiceConfig) {
    config
    .service(health_handler::greet)
    .service(health_handler::test);
}
