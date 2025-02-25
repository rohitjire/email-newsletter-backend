//! Handlers for managing articles in the newsletter backend.
//! 
//! This module provides functions for creating, retrieving, and listing articles,
//! along with sending notification emails to subscribers.

use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{post, web};
use actix_web::get;
use chrono::{NaiveDateTime, Utc};
use sea_orm::{JoinType, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;

use crate::email::email_service;
use crate::utils::api_response::ApiResponse;
use crate::utils::{api_response, app_state, jwt::Claims};

/// Represents an article with associated metadata.
#[derive(Serialize,Deserialize)]
pub struct ArticleModel {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub uuid: Uuid,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub image: Option<String>,
    pub user: Option<UserModel>
}

/// Represents the request model for creating an article.
#[derive(Serialize,Deserialize)]
pub struct CreateArticleModel {
    pub title: String,
    pub content: String
}

#[derive(Serialize,Deserialize)]
pub struct UserModel {
    name: String,
    email: String,
}

#[post("/create")]
pub async fn create_article(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
    article_model: web::Json<CreateArticleModel>,
    query: web::Query<HashMap<String, String>>, 
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let db = Arc::clone(&app_state.db);

    let article_entity = entity::article::ActiveModel {
        title: Set(article_model.title.clone()),
        content: Set(article_model.content.clone()),
        user_id: Set(claims.id),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    let inserted_article = article_entity
        .insert(&*db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;


    let send_email = query.get("send_email")
        .map(|v| v == "true")
        .unwrap_or(true); // Default: true

    if send_email {
        let subscribers = entity::subscription::Entity::find()
            .filter(entity::subscription::Column::SubscribedUserId.eq(claims.id))
            .join_rev(
                JoinType::InnerJoin,
                entity::user::Entity::belongs_to(entity::subscription::Entity)
                    .from(entity::user::Column::Id)
                    .to(entity::subscription::Column::SubscriberUserId)
                    .into(),
            )
            .select_also(entity::user::Entity)
            .all(&*db)
            .await
            .map_err(|err| ApiResponse::new(500, err.to_string()))?
            .into_iter()
            .filter_map(|(_, user)| user)
            .collect::<Vec<entity::user::Model>>();

        let article_link =format!(
            "http://localhost:8080/article/get-by-uuid/{}",inserted_article.uuid
        );

        for subscriber in subscribers {
            let unsubscribe_link = format!(
                "http://localhost:8080/subscription/unsubscribe-user?user_id={}&subscriber_id={}",
                claims.id, subscriber.id
            );

            email_service::send_newsletter_email(
                &subscriber.email,
                &inserted_article.title,
                &inserted_article.content[..50],
                &article_link,
                &unsubscribe_link,
            )
            .await
            .map_err(|err| ApiResponse::new(500, err.to_string()))?;
        }
    }

    Ok(api_response::ApiResponse::new(200, "Article created successfully".to_owned()))
}

#[get("/all-article")]
pub async fn all_articles(
    app_state: web::Data<app_state::AppState>,
)-> Result<api_response::ApiResponse, api_response::ApiResponse> {

    let db = Arc::clone(&app_state.db);


    let articles: Vec<ArticleModel> = entity::article::Entity::find()
    .all(&*db).await
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

    let db = Arc::clone(&app_state.db);

    let articles: ArticleModel = entity::article::Entity::find()
    .filter(entity::article::Column::Uuid.eq(article_uuid.clone()))
    .find_also_related(entity::user::Entity)
    .one(&*db).await
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

    let db = Arc::clone(&app_state.db);

    let articles: Vec<ArticleModel> = entity::article::Entity::find()
    .filter(entity::article::Column::UserId.eq(claim.id)).all(&*db).await
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