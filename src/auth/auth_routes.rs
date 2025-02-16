/// Routes for authentication endpoints.
/// This module configures `/auth` routes and connects them to handlers.
use actix_web::web;
use super::auth_handlers;

/// Configures authentication routes.
/// 
/// # Routes:
/// - `/auth/register`: User registration handler.
/// - `/auth/login`: User login handler.
/// 
/// # Arguments:
/// * `config` - Actix Web `ServiceConfig` to which routes are added.
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
        .service(auth_handlers::register)
        .service(auth_handlers::login)
    );
}