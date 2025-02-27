use std::{str, sync::Arc};

use actix_web::{get, web};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
 
use crate::utils::{
    api_response::{self},
    app_state,
    jwt::Claims,
};

#[derive(Serialize,Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// Handler for fetching a user's details based on their authentication claims.
///
/// # Arguments
/// * `app_state` - Shared application state containing the database connection.
/// * `claim_data` - JWT claims containing the user's ID.
///
/// # Returns
/// * `ApiResponse` - A JSON response containing the user's name and email if found.
///
/// # Errors
/// * Returns a `500` error if database lookup fails.
/// * Returns a `404` error if the user is not found.
#[get("/get-user")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<api_response::ApiResponse,api_response::ApiResponse> {
    
    let db = Arc::clone(&app_state.db);
 
    // Fetch user by ID from the database
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&*db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User not found".to_owned(),
        ))?;

    // Return user details as a JSON response
    Ok(api_response::ApiResponse::new(
        200,
        format!("{{'name' : '{}' , 'email': '{}' }}" , user_model.name , user_model.email)
    ))
}

/// Handler for fetching all users from the database.
///
/// # Arguments
/// * `app_state` - Shared application state containing the database connection.
///
/// # Returns
/// * `ApiResponse` - A JSON response containing a list of users (`id`, `name`, `email`).
///
/// # Errors
/// * Returns a `500` error if the database query fails.
/// * Returns a `404` error if no users are found.
#[get("/get-all-users")]
pub async fn get_all_users(
    app_state: web::Data<app_state::AppState>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    
    let db = Arc::clone(&app_state.db);

    // Fetch all users from the database
    let users = entity::user::Entity::find()
        .all(&*db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    if users.is_empty() {
        return Err(api_response::ApiResponse::new(404, "No users found".to_owned()));
    }

    // Map database models into UserModel
    let user_list: Vec<UserModel> = users.into_iter()
        .map(|u: entity::user::Model| UserModel {
            id: u.id,
            name: u.name,
            email: u.email,
        })
        .collect();

        let res_str = serde_json::to_string(&user_list)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    
        Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}
