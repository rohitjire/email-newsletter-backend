use actix_web::{get, web};
use sea_orm::EntityTrait;
 
use crate::utils::{
    api_response::{self},
    app_state,
    jwt::Claims,
};
 
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
    
 
    // Fetch user by ID from the database
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
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
