use actix_web::{middleware::from_fn, web};

use crate::middlewares;

use super::user_handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
        .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))
        .service(user_handlers::user)
    );
}
