use std::sync::Arc;

use actix_web::{get, post, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response::ApiResponse, app_state::AppState, jwt::Claims};

#[derive(Deserialize)]
struct SubscriptionRequest {
    user_id: i32,
}

#[post("/subscribe-user")]
pub async fn subscribe_user(
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

#[post("/unsubscribe-user")]
pub async fn unsubscribe_user(
    app_state: web::Data<AppState>,
    claims: Claims,
    subscription_request: web::Json<SubscriptionRequest>,
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

    Ok(ApiResponse::new(
        200,
        "Unsubscribed successfully".to_owned(),
    ))
}