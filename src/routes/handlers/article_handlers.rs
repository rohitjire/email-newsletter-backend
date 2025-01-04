
use actix_web::{post, web};
use chrono::Utc;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::ActiveModelTrait;

use crate::utils::{api_response, app_state, jwt::Claims};

#[derive(Serialize,Deserialize)]
struct ArticleModel {
    title: String,
    content: String,
}

#[post("/create")]
pub async fn create_article(
    app_state: web::Data<app_state::AppState>,
    claims:Claims,
    article_model: web::Json<ArticleModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    

    let article_entity = entity::article::ActiveModel {
        title: Set(article_model.title.clone()),
        content: Set(article_model.content.clone()),
        user_id: Set(claims.id),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };


    article_entity
    .insert(&app_state.db)
    .await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "Article created successfully".to_owned()))
}