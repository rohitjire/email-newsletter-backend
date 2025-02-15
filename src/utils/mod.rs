/// Module that organizes utility components for the project.
///
/// This module includes configurations, authentication utilities, and response handling.

/// Contains global constants used across the application.
pub mod contants;
/// Provides standardized API response structures and error handling.
pub mod api_response;
/// Manages the shared application state, including database connections.
pub mod app_state;
/// Handles JSON Web Token (JWT) authentication and validation.
pub mod jwt;