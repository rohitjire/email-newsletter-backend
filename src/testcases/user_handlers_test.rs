/// Integration tests for user handlers.
/// This module contains tests for retrieving user information.
#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::{
        user::user_routes::config,
        utils::{app_state::AppState, jwt::encode_jwt},
    };
    use actix_web::{http::StatusCode, test, web, App};
    use sea_orm::{DatabaseBackend, MockDatabase};
    use serial_test::serial;

    /// Test retrieving a user profile.
    #[actix_web::test]
    #[serial]
    pub async fn test_get_user() {
        let token = encode_jwt("author@test.com".to_string(), 1).unwrap();

        // Mock database with a sample user
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::user::Model {
                id: 1,
                name: "Author".to_string(),
                email: "author@test.com".to_string(),
                password: "12345".to_string(),
            }]])
            .into_connection();

        let mock_db = Arc::new(mock_db);

        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });

        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let req = test::TestRequest::get()
            .uri("/user/get-user")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    #[serial]
    async fn test_get_all_users() {
        let token = encode_jwt("author@test.com".to_string(), 1).unwrap();

        let mock_db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::user::Model {
                id: 1,
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                password: "12345".to_string(),
            }]])
            .into_connection();

        let app_state = web::Data::new(AppState { db: Arc::new(mock_db) });
        let app = 
        test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;
        
        let req = test::TestRequest::get()
            .uri("/user/get-all-users")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);
    }
}