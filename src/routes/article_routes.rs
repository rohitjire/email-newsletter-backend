use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/article")
        .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))
        .service(handlers::article_handlers::create_article)
    );
}
