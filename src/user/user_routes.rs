use actix_web::{middleware::from_fn, web};

use crate::middlewares;

use super::user_handlers;

/// Configures user-related routes and applies authentication middleware.
///
/// # Arguments
/// * `config` - A mutable reference to the Actix web service configuration.
///
/// This function sets up a scoped route under `/user`, ensuring that requests
/// pass through the authentication middleware before reaching the handler.
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
        .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))
        .service(user_handlers::user)
        .service(user_handlers::get_all_users)
    );
}
