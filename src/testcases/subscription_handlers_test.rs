/// Integration tests for subscription handlers.
/// This module contains tests for subscribing, unsubscribing, and retrieving subscriptions.
#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::subscription::subscription_handlers::SubscriptionRequest;
    use crate::subscription::subscription_routes::config;
    use crate::utils::app_state::AppState;
    use crate::utils::jwt::encode_jwt;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use actix_web::{test, App};
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serial_test::serial;

    /// Test subscribing to a user.
    #[actix_web::test]
    #[serial]
    pub async fn test_subscribe_user() {
        let token = encode_jwt("author@example.com".to_string(), 2).unwrap();

        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            // First query: Check for existing subscription (returns None)
            .append_query_results(vec![vec![]] as Vec<Vec<entity::subscription::Model>>)
            // Second query: Insert new subscription
            .append_query_results(vec![vec![entity::subscription::Model {
                id: 1,
                subscribed_user_id: 1,
                subscriber_user_id: 2,
                created_at: Utc::now().naive_local(),
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let mock_db = Arc::new(mock_db);

        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });

        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let subscription_request = SubscriptionRequest { user_id: 1 };

        let req = test::TestRequest::post()
            .uri("/subscription/subscribe-user")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&subscription_request)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
    
    /// Test unsubscribing from a user.
    #[actix_web::test]
    #[serial]
    pub async fn test_unsubscribe_user() {
        let token = encode_jwt("author@example.com".to_string(), 2).unwrap();

        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::subscription::Model {
                id: 1,
                subscribed_user_id: 1,
                subscriber_user_id: 2,
                created_at: Utc::now().naive_local(),
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let mock_db = Arc::new(mock_db);

        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });

        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let req = test::TestRequest::get()
            .uri("/subscription/unsubscribe-user?user_id=1")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// Test retrieving the authenticated user's subscriptions.
    #[actix_web::test]
    #[serial]
    pub async fn test_my_subscriptions() {
        let token = encode_jwt("author@example.com".to_string(), 2).unwrap();

        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(
                entity::subscription::Model {
                    id: 1,
                    subscribed_user_id: 1,
                    subscriber_user_id: 2,
                    created_at: Utc::now().naive_local(),
                },
                Some(entity::user::Model {
                    id: 1,
                    name: "Test User".to_string(),
                    email: "testuser@example.com".to_string(),
                    password: "hashed_password".to_string(),
                }),
            )]])
            .into_connection();

        let mock_db = Arc::new(mock_db);

        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });

        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let req = test::TestRequest::get()
            .uri("/subscription/my-subscriptions")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// Test retrieving the authenticated user's subscribers.
    #[actix_web::test]
    #[serial]
    pub async fn test_my_subscribers() {
        let token = encode_jwt("author@example.com".to_string(), 2).unwrap();

        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(
                entity::subscription::Model {
                    id: 1,
                    subscribed_user_id: 2,
                    subscriber_user_id: 1,
                    created_at: Utc::now().naive_local(),
                },
                Some(entity::user::Model {
                    id: 1,
                    name: "Subscriber User".to_string(),
                    email: "subscriber@example.com".to_string(),
                    password: "hashed_password".to_string(),
                }),
            )]])
            .into_connection();

        let mock_db = Arc::new(mock_db);

        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });

        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let req = test::TestRequest::get()
            .uri("/subscription/my-subscribers")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}