#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use crate::{
        auth::{self, auth_routes::config},
        utils::app_state::AppState,
    };
    use actix_web::{http::StatusCode, test, web, App};
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serial_test::serial;

    #[actix_web::test]
    #[serial]
    pub async fn test_register() {
        let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![]] as Vec<Vec<entity::user::Model>>)
            .append_query_results(vec![vec![entity::user::Model {
                id: 1,
                name: "Author".to_string(),
                email: "author@test.com".to_string(),
                password: "12345".to_string(),
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

        let user_model = auth::auth_handlers::RegisterModel {
            name: "Author".to_string(),
            email: "author@test.com".to_string(),
            password: "12345".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .insert_header(("Content-Type", "application/json"))
            .set_json(&user_model)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

    }
}