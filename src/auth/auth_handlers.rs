/// Handlers for authentication endpoints.
/// This module provides `register` and `login` endpoints for user management.
use actix_web::{post, web};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;
use std::sync::Arc;

use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response, app_state};

/// Request model for user registration.
#[derive(Serialize, Deserialize)]
pub struct RegisterModel {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Request model for user login.
#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

/// Endpoint to register a new user.
/// Creates a new user record and returns their ID.
#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    
    let db = Arc::clone(&app_state.db);

    // Validate email format
    if !register_json.email.contains('@') {
        return Err(ApiResponse::new(400, "Invalid email format".to_owned()));
    }

    // Check if the email already exists
    let existing_user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&register_json.email))
        .one(&*db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    if existing_user.is_some() {
        return Err(ApiResponse::new(400, "Email already registered".to_owned()));
    }

    // Ensure name is not empty
    if register_json.name.trim().is_empty() {
        return Err(ApiResponse::new(400, "Name cannot be empty".to_owned()));
    }

    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)), // Hashing the password
        ..Default::default()
    }
    .insert(&*db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!("{}", user_model.id),
    ))
}


/// Endpoint to log in an existing user.
/// Validates user credentials and returns a JWT token.
#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {

    let db = Arc::clone(&app_state.db);

    // Validate email format
    if !login_json.email.contains('@') {
        return Err(ApiResponse::new(400, "Invalid email format".to_owned()));
    }

    // Check if the user exists
    let user_data = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&login_json.email))
        .one(&*db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    if user_data.is_none() {
        return Err(ApiResponse::new(404, "User not found".to_owned()));
    }

    let user_data = user_data.unwrap();

    // Validate password
    if user_data.password != digest(&login_json.password) {
        return Err(ApiResponse::new(401, "Incorrect password".to_owned()));
    }

    let token = encode_jwt(user_data.email, user_data.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!("{{'token': '{}'}}", token),
    ))
}
