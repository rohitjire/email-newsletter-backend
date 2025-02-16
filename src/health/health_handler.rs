/// Handlers for health check endpoints.
/// Provides endpoints to check server health and database connectivity.
use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};

use crate::utils::{api_response, app_state::AppState};

/// Endpoint to greet a user by name.
/// 
/// # Path Parameters:
/// - `name`: The name to include in the greeting.
/// 
/// # Response:
/// Returns a `200 OK` status with a personalized greeting message.
#[get("/ping/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {name}!"))
}

/// Endpoint to test database connectivity by querying all users.
/// 
/// # Response:
/// Returns a `200 OK` status if the database query runs successfully.
#[get("/test-all-users")]
pub async fn test(app_state: web::Data<AppState>) -> impl Responder {
    app_state.db.query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "select * from user;"))
    .await
    .unwrap();
    api_response::ApiResponse::new(200, "Test".to_string())
}
