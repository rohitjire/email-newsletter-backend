
use actix_web::{post, web};
use actix_web::get;
use chrono::{NaiveDateTime, Utc};
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

use crate::utils::{api_response, app_state, jwt::Claims};

#[derive(Serialize,Deserialize)]
struct ArticleModel {
    // title: String,
    // content: String,
    pub id: i32,
    pub title: String,
    pub content: String,
    pub uuid: Uuid,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub image: Option<String>,
    pub user: Option<UserModel>
}

#[derive(Serialize,Deserialize)]
struct UserModel {
    name: String,
    email: String,
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

#[get("all-posts")]
pub async fn all_posts(
    app_state: web::Data<app_state::AppState>,
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

    let posts: Vec<ArticleModel> = entity::article::Entity::find()
    .all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .into_iter()
    .map(|post| 
        ArticleModel {
            id: post.id,
            title: post.title,
            content: post.content,
            uuid: post.uuid,
            user_id: post.user_id,
            created_at: post.created_at,
            image: post.image,
        }
    ).collect();
    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}


#[get("post/[post_uuid")]
pub async fn one_posts(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>,
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

    let posts: ArticleModel = entity::article::Entity::find()
    .filter(entity::article::Column::Uuid.eq(post_uuid.clone()))
    .find_also_related(entity::user::Entity)
    .one(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .map(|post| 
        ArticleModel {
            id: post.0.id,
            title: post.0.title,
            content: post.0.content,
            uuid: post.0.uuid,
            user_id: post.0.user_id,
            created_at: post.0.created_at,
            image: post.0.image,
            user: post.1.map(|item | UserModel { name: item.name, email: item.email })
        }
    )
    .ok_or(api_response::ApiResponse::new(404, "No Post Found".to_string()))?;
    let res_str = serde_json::to_string(&posts)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}