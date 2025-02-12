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
struct CreateArticleModel {
    pub title: String,
    pub content: String
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
    article_model: web::Json<CreateArticleModel>,
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

    // // Fetch subscribers
    // let subscribers: Vec<entity::user::Model> = entity::subscription::Entity::find()
    //     .filter(entity::subscription::Column::SubscriberUserId.eq(claims.id))
    //     .find_with_related(entity::user::Entity)
    //     .all(&app_state.db)
    //     .await
    //     .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    //     .into_iter()
    //     .map(|(_, user)| user)
    //     .collect();

    // // Send email to subscribers
    // for subscriber in subscribers {
    //     send_email(
    //         &subscriber.email,
    //         "New Article Posted",
    //         &format!("A new article titled '{}' has been posted.", article_model.title),
    //     )
    //     .await
    //     .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    // }

    Ok(api_response::ApiResponse::new(200, "Article created successfully".to_owned()))
}

#[get("/all-article")]
pub async fn all_articles(
    app_state: web::Data<app_state::AppState>,
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

    let articles: Vec<ArticleModel> = entity::article::Entity::find()
    .all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .into_iter()
    .map(|article: entity::article::Model|
        ArticleModel {
            id: article.id,
            title: article.title,
            content: article.content,
            uuid: article.uuid,
            user_id: article.user_id,
            created_at: article.created_at,
            image: article.image,
            user: None
        }
    ).collect();
    let res_str = serde_json::to_string(&articles)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}


#[get("/get-by-uuid/{article_uuid}")]
pub async fn one_article(
    app_state: web::Data<app_state::AppState>,
    article_uuid: web::Path<Uuid>,
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

    let articles: ArticleModel = entity::article::Entity::find()
    .filter(entity::article::Column::Uuid.eq(article_uuid.clone()))
    .find_also_related(entity::user::Entity)
    .one(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
    .map(|article|
        ArticleModel {
            id: article.0.id,
            title: article.0.title,
            content: article.0.content,
            uuid: article.0.uuid,
            user_id: article.0.user_id,
            created_at: article.0.created_at,
            image: article.0.image,
            user: article.1.map(|item | UserModel { name: item.name, email: item.email })
        }
    )
    .ok_or(api_response::ApiResponse::new(404, "No article Found".to_string()))?;
    let res_str = serde_json::to_string(&articles)
    .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}

#[get("/my-article")]
pub async fn my_article(
    app_state: web::Data<app_state::AppState>,
    claim: Claims
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

    let articles: Vec<ArticleModel> = entity::article::Entity::find()
    .filter(entity::article::Column::UserId.eq(claim.id)).all(&app_state.db).await
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?
    .into_iter()
    .map(|article|
        ArticleModel {
            id: article.id,
            title: article.title,
            content: article.content,
            uuid: article.uuid,
            image: article.image,
            user_id: article.user_id,
            created_at: article.created_at,
            user: None
        }  
    ).collect();
    let res_str = serde_json::to_string(&articles)
    .map_err(|err| api_response::ApiResponse::new(500,err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}