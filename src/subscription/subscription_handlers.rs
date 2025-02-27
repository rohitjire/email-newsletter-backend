
use std::{collections::HashMap, sync::Arc};
use actix_web::{get, post, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use crate::utils::{api_response::ApiResponse, app_state::AppState, jwt::Claims};


/// Handlers for user subscription operations.
/// Provides endpoints to subscribe and unsubscribe from other users.


/// Request model for subscription operations.
#[derive(Serialize,Deserialize)]
pub struct SubscriptionRequest {
    pub user_id: i32,
}
/// Response model for subscription operations.
#[derive(Serialize, FromQueryResult)]
pub struct SubscriptionResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// Endpoint to subscribe to another user.
/// Validates and inserts a subscription record.
#[post("/subscribe-user")]
pub async fn subscribe_user (
    app_state: web::Data<AppState>,
    claims: Claims,
    subscription_request: web::Json<SubscriptionRequest>,
) -> Result<ApiResponse, ApiResponse> {
    let subscriber_id = claims.id;
    let subscribed_to_id = subscription_request.user_id;

    let db = Arc::clone(&app_state.db);

    if subscriber_id == subscribed_to_id {
        return Err(ApiResponse::new(
            400,
            "Cannot subscribe to yourself.".to_owned(),
        ));
    }

    // Check if already subscribed
    let is_already_subscribed = entity::subscription::Entity::find()
        .filter(entity::subscription::Column::SubscriberUserId.eq(subscriber_id))
        .filter(entity::subscription::Column::SubscribedUserId.eq(subscribed_to_id))
        .one(&*db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .is_some();

    if is_already_subscribed {
        return Err(ApiResponse::new(
            400,
            "Already subscribed to this user.".to_owned(),
        ));
    }

    // Insert new subscription
    let subscription = entity::subscription::ActiveModel {
        subscribed_user_id: Set(subscribed_to_id),
        subscriber_user_id: Set(subscriber_id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    subscription
        .insert(&*db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "Subscribed successfully".to_owned()))
}

/// Endpoint to unsubscribe from a user.
/// Deletes a subscription record if found.
#[get("/unsubscribe-user")]
pub async fn unsubscribe_user(
    app_state: web::Data<AppState>,
    claims: Claims,
    subscription_request: web::Query<SubscriptionRequest>,
) -> Result<ApiResponse, ApiResponse> {
    let subscriber_id = claims.id;
    let subscribed_to_id = subscription_request.user_id;
    let db = Arc::clone(&app_state.db);

    let delete_result = entity::subscription::Entity::delete_many()
        .filter(entity::subscription::Column::SubscribedUserId.eq(subscribed_to_id))
        .filter(entity::subscription::Column::SubscriberUserId.eq(subscriber_id))
        .exec(&*db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    if delete_result.rows_affected == 0 {
        return Err(ApiResponse::new(404, "Subscription not found".to_owned()));
    }

    Ok(ApiResponse::new(200, "Unsubscribed successfully".to_owned()))
}

/// Endpoint to unsubscribe from a user.
/// Deletes a subscription record if found.
#[get("/unsubscribe-user-from-email")]
pub async fn unsubscribe_user_from_email(
    app_state: web::Data<AppState>,
    claims: web::Query<HashMap<String, String>>,
    subscription_request: web::Query<SubscriptionRequest>,
) -> Result<ApiResponse, ApiResponse> {
    let subscriber_id = claims.get("subscriber_id").and_then(|s| s.parse::<i32>().ok()).unwrap();
    let subscribed_to_id = subscription_request.user_id;
    let db = Arc::clone(&app_state.db);

    let delete_result = entity::subscription::Entity::delete_many()
        .filter(entity::subscription::Column::SubscribedUserId.eq(subscribed_to_id))
        .filter(entity::subscription::Column::SubscriberUserId.eq(subscriber_id))
        .exec(&*db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    if delete_result.rows_affected == 0 {
        return Err(ApiResponse::new(404, "Subscription not found".to_owned()));
    }

    Ok(ApiResponse::new(200, "Unsubscribed successfully".to_owned()))
}

/// Endpoint to get subscriptions of the user.
/// Finds all the subscribed users.
#[get("/my-subscriptions")]
pub async fn my_subscriptions(
    app_state: web::Data<AppState>,
    claims: Claims,
) -> Result<ApiResponse, ApiResponse> {

    let db = Arc::clone(&app_state.db);

    let subscriptions = entity::subscription::Entity::find()
    .filter(entity::subscription::Column::SubscriberUserId.eq(claims.id))
    .join_rev(
        JoinType::InnerJoin,
        entity::user::Entity::belongs_to(entity::subscription::Entity)
            .from(entity::user::Column::Id)
            .to(entity::subscription::Column::SubscribedUserId)
            .into(),
    )
    .select_also(entity::user::Entity)
    .all(&*db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?
    .into_iter()
    .filter_map(|(_subscription, user_opt)| {
        user_opt.map(|user| SubscriptionResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    })
    .collect::<Vec<SubscriptionResponse>>();

    let res_str = serde_json::to_string(&subscriptions)
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str))
}

/// Endpoint to get subscribers of the user.
/// Finds all the subscribers of the users.
#[get("/my-subscribers")]
pub async fn my_subscribers(
    app_state: web::Data<AppState>,
    claims: Claims,
) -> Result<ApiResponse, ApiResponse> {

    let db = Arc::clone(&app_state.db);

    let subscribers = entity::subscription::Entity::find()
    .filter(entity::subscription::Column::SubscribedUserId.eq(claims.id))
    .join_rev(
        JoinType::InnerJoin,
        entity::user::Entity::belongs_to(entity::subscription::Entity)
            .from(entity::user::Column::Id)
            .to(entity::subscription::Column::SubscriberUserId)
            .into(),  // Convert RelationBuilder to RelationDef
    )
    .select_also(entity::user::Entity)
    .all(&*db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?
    .into_iter()
    .filter_map(|(_subscription, user_opt)|
        user_opt.map(|user| SubscriptionResponse {
            id: user.id,
            name: user.name,
            email: user.email,
    }))
    .collect::<Vec<SubscriptionResponse>>();

    let res_str = serde_json::to_string(&subscribers)
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str))      
}