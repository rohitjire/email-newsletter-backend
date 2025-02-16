/// Middleware for authorization checks.
/// This module provides a middleware to validate JWT tokens from request headers.
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    Error, HttpMessage,
};

use crate::utils::{
    api_response::{self, ApiResponse},
    jwt::decode_jwt,
};

/// Middleware to verify authorization token.
/// 
/// # Arguments:
/// * `req` - The incoming service request.
/// * `next` - The next service to call if authorization succeeds.
/// 
/// # Returns:
/// * `ServiceResponse` if authorized, or an error response otherwise.
pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Retrieve the Authorization header
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }
    
    // Extract and decode the Bearer token
    let token: String = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer", "")
        .trim()
        .to_owned();

    // Decode the JWT token and extract claims 
    let claim = decode_jwt(token.clone()).map_err(|err| {
        eprintln!("JWT decoding error: {:?}", err);
        Error::from(api_response::ApiResponse::new(401, "Invalid token".to_string()))
    })?;

    // Store claims in the request extensions for downstream use
    req.extensions_mut().insert(claim.claims);
    
    // Proceed with the next service in the middleware chain
    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
