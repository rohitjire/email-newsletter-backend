/// Integration tests for article handlers.
/// This module contains tests for creating, fetching, and listing articles.
#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::{
        article::{self, article_routes::config},
        utils::{app_state::AppState, jwt::encode_jwt},
    };
    use actix_web::{http::StatusCode, test, web, App};
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serial_test::serial;
    use uuid::Uuid;

    /// Test creating an article without sending an email.
    #[actix_web::test]
    #[serial]
    pub async fn test_create_article_without_email() {
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::article::Model {
                id: 1,
                title: "Test Article".to_string(),
                content: "Test Content".to_string(),
                user_id: 1,
                uuid: Uuid::new_v4(),
                created_at: Utc::now().naive_local(),
                image: None,
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::user::Model {
                id: 1,
                name: "Author".to_string(),
                email: "author@example.com".to_string(),
                password: "password".to_string(),
            }]])
            .into_connection();

        let mock_db = Arc::new(mock_db);

        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });

        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        // Generate mock JWT token
        let token = encode_jwt("author@example.com".to_string(), 1).unwrap();

        let article_data = article::article_handlers::CreateArticleModel {
            title: "Test Article".to_string(),
            content: "Test Content".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/secure/article/create?send_email=false")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&article_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// Test fetching all articles.
    #[actix_web::test]
    #[serial]
    pub async fn test_all_articles() {
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::article::Model {
                id: 1,
                title: "Test Article".to_string(),
                content: "Test Content".to_string(),
                user_id: 1,
                uuid: Uuid::new_v4(),
                created_at: Utc::now().naive_local(),
                image: None,
            }]])
            .into_connection();

        let mock_db = Arc::new(mock_db);
        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });
        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let req = test::TestRequest::get()
            .uri("/article/all-article")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// Test fetching a single article by UUID.
    #[actix_web::test]
    #[serial]
    pub async fn test_one_article() {
        let test_uuid = Uuid::new_v4();
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![(
                entity::article::Model {
                    id: 1,
                    title: "Test Article".to_string(),
                    content: "Test Content".to_string(),
                    user_id: 1,
                    uuid: test_uuid,
                    created_at: Utc::now().naive_local(),
                    image: None,
                },
                Some(entity::user::Model {
                    id: 1,
                    name: "Test User".to_string(),
                    email: "test@example.com".to_string(),
                    password: "password".to_string(),
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
            .uri(&format!("/article/get-by-uuid/{}", test_uuid))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    /// Test fetching articles belonging to the authenticated user.
    #[actix_web::test]
    #[serial]
    pub async fn test_my_articles() {
        let token = encode_jwt("author@example.com".to_string(), 1).unwrap();
        let test_uuid = Uuid::new_v4();
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![entity::article::Model {
                id: 1,
                title: "My Article".to_string(),
                content: "My Content".to_string(),
                user_id: 1,
                uuid: test_uuid,
                created_at: Utc::now().naive_local(),
                image: None,
            }]])
            .into_connection();

        let mock_db = Arc::new(mock_db);
        let app_state = web::Data::new(AppState {
            db: Arc::clone(&mock_db),
        });
        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(config)).await;

        let req = test::TestRequest::get()
            .uri("/secure/article/my-article")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
