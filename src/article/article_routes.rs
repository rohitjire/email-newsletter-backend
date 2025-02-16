/// Route configuration for `article` endpoints.
/// This function defines secure and public routes for article operations.
use actix_web::{middleware::from_fn, web};
use crate::middlewares;
use super::article_handlers;

/// Configures routes for article-related operations.
/// 
/// # Arguments
/// * `config` - Mutable reference to Actix Web's `ServiceConfig`.
/// 
/// ## Routes:
/// - **Secure Routes** (`/secure/article`): Require authentication middleware.
///   - `create_article`: Create a new article.
///   - `my_article`: View articles created by the authenticated user.
/// 
/// - **Public Routes** (`/article`): Accessible without authentication.
///   - `one_article`: View a single article by ID.
///   - `all_articles`: View all articles.
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
