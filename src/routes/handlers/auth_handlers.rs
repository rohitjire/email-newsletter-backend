use actix_web::{post, web, Responder};

use sea_orm::Set;
use sea_orm::ActiveModelTrait;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;

use crate::utils::{api_response, app_state};

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String
}


#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json:web::Json<RegisterModel> ) -> impl Responder {

        let user_model = entity::user::ActiveModel {
            name: Set(register_json.name.clone()),
            email: Set(register_json.email.clone()),
            password: Set(digest(&register_json.password)),
            ..Default::default()
        }.insert(&app_state.db).await.unwrap();

        api_response::ApiResponse::new(200, format!("{}", user_model.id))
}       



