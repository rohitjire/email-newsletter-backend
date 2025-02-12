use actix_web::{middleware::from_fn, web};

use crate::middlewares;

use super::article_handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("secure/article")
        .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))
        .service(article_handlers::create_article)
        .service(article_handlers::my_article)
    )
    .service(
        web::scope("/article")
        .service(article_handlers::one_article)
        .service(article_handlers::all_articles)
    );
}