use std::sync::Arc;

use sea_orm::DatabaseConnection;


pub struct AppState {
    pub db: Arc<DatabaseConnection>
}