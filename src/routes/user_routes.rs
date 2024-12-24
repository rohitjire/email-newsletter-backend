use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
        .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))
        .service(handlers::user_handlers::user)
    );
}
