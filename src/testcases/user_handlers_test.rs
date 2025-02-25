//! User Handlers Test Module
//!
//! This module contains unit tests for user-related API handlers, ensuring
//! correct functionality of user retrieval endpoints.

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

    /// Tests the `GET /user/get-user` endpoint.
    ///
    /// - Mocks a database with a sample user.
    /// - Encodes a JWT token for authorization.
    /// - Sends a request and checks if the response status is `200 OK`.
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

        // Initialize the test application with user routes
        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        // Create the test request with headers
        let req = test::TestRequest::get()
            .uri("/user/get-user")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        // Execute the request and check the response
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}