/// Routes configuration for subscription endpoints.
/// This module registers routes for subscribing and unsubscribing users with authentication.
use actix_web::{middleware::from_fn, web};
use crate::middlewares;
use super::subscription_handlers;

/// Configures routes for subscription-related operations.
/// 
/// # Routes:
/// - `/subscription/subscribe-user`: Subscribe to a user (requires authentication).
/// - `/subscription/unsubscribe-user`: Unsubscribe from a user (requires authentication).
/// 
/// # Middleware:
/// - `auth_middlewares::check_auth_middleware`: Ensures user authentication.
/// 
/// # Arguments:
/// * `config` - Actix Web `ServiceConfig` to which routes are added.
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/secure/subscription")
            .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))// Auth Middleware
            .service(subscription_handlers::subscribe_user)// Endpoint to subscribe a user
            .service(subscription_handlers::unsubscribe_user)// Endpoint to unsubscribe a user
            .service(subscription_handlers::my_subscriptions)
            .service(subscription_handlers::my_subscribers)
    )
    .service(web::scope("/subscription")
        .service(subscription_handlers::unsubscribe_user_from_email)
    );
}
