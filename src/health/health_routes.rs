/// Routes configuration for health check endpoints.
/// This module registers routes for the `greet` and `test` endpoints.
use actix_web::web;
use super::health_handler;

/// Configures health-related routes.
/// 
/// # Routes:
/// - `/ping/{name}`: Greet a user with their name.
/// - `/test-all-users`: Test database connectivity by querying users.
/// 
/// # Arguments:
/// * `config` - Mutable reference to Actix Web's `ServiceConfig`.
pub fn config(config: &mut web::ServiceConfig) {
    config
    .service(health_handler::greet)
    .service(health_handler::test);
}
