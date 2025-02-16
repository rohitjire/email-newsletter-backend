/// Handlers for user subscription operations.
/// Provides endpoints to subscribe and unsubscribe from other users.
use actix_web::{post, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde::Deserialize;
use crate::utils::{api_response::ApiResponse, app_state::AppState, jwt::Claims};

/// Request model for subscription operations.
#[derive(Deserialize)]
struct SubscriptionRequest {
    user_id: i32,
}

/// Endpoint to subscribe to another user.
/// Validates and inserts a subscription record.
#[post("/subscribe-user")]
pub async fn subscribe_user(
    app_state: web::Data<AppState>,
    claims: Claims,
    subscription_request: web::Json<SubscriptionRequest>,) -> Result<ApiResponse, ApiResponse> {

    let subscriber_id = claims.id;
    let subscribed_to_id = subscription_request.user_id; 

    if subscriber_id == subscribed_to_id {
        return Err(ApiResponse::new(400, "Cannot subscribe to yourself.".to_owned()));
    }

    // Check if already subscribed
    let existing_subscription = entity::subscription::Entity::find()
        .filter(entity::subscription::Column::SubscriberUserId.eq(subscriber_id))
        .filter(entity::subscription::Column::SubscribedUserId.eq(subscribed_to_id))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    if existing_subscription.is_some() {
        return Err(ApiResponse::new(400, "Already subscribed to this user.".to_owned()));
    }

    // Insert new subscription
    let subscription = entity::subscription::ActiveModel {
        subscribed_user_id: Set(subscribed_to_id),
        subscriber_user_id: Set(subscriber_id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    subscription
        .insert(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "Subscribed successfully".to_owned()))
}

/// Endpoint to unsubscribe from a user.
/// Deletes a subscription record if found.
#[post("/unsubscribe-user")]
pub async fn unsubscribe_user(
    app_state: web::Data<AppState>,
    claims: Claims,
    subscription_request: web::Json<SubscriptionRequest>,
) -> Result<ApiResponse, ApiResponse> {
    let subscriber_id = claims.id;
    let subscribed_to_id = subscription_request.user_id;

    let delete_result = entity::subscription::Entity::delete_many()
        .filter(entity::subscription::Column::SubscribedUserId.eq(subscribed_to_id))
        .filter(entity::subscription::Column::SubscriberUserId.eq(subscriber_id))
        .exec(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    if delete_result.rows_affected == 0 {
        return Err(ApiResponse::new(404, "Subscription not found".to_owned()));
    }

    Ok(ApiResponse::new(200, "Unsubscribed successfully".to_owned()))
}