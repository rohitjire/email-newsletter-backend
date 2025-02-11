use actix_web::{middleware::from_fn, web};
use crate::routes::handlers::subscription_handlers;

use super::middlewares;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/subscription")
            .wrap(from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(subscription_handlers::subscribe_user)
            .service(subscription_handlers::unsubscribe_user)
    );
}
