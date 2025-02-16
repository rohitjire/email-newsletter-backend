/// Application state structure containing shared resources.
///
/// This struct holds the database connection, which is shared across different parts of the application.
use sea_orm::DatabaseConnection;

/// Represents the application state.
pub struct AppState {
    /// Database connection instance.
    pub db: DatabaseConnection
}