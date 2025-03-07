/// Module for handling JSON Web Token (JWT) authentication.
///
/// This module provides functionality for encoding and decoding JWTs,
/// as well as extracting claims from requests.
use std::future;

use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use super::contants;
 
 /// Structure representing JWT claims.
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Expiration time (Unix timestamp).
    pub exp: usize,
    /// Issued-at time (Unix timestamp).
    pub iat: usize,
    /// Email associated with the token.
    pub email: String,
    /// User ID.
    pub id: i32,
}
 
 /// Implements extraction of claims from an Actix request.
impl FromRequest for Claims {
    type Error = actix_web::Error;
 
    type Future = future::Ready<Result<Self, Self::Error>>;
 
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
 
        match req.extensions().get::<Claims>() {
            Some(claim) => future::ready(Ok(claim.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claims"))),
        }
       
    }
}

/// Encodes a JWT token with the given email and user ID.
///
/// # Arguments
/// * `email` - The email to include in the token.
/// * `id` - The user ID to include in the token.
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token.
/// * `Err(jsonwebtoken::errors::Error)` - If encoding fails.
pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::hours(24);
 
    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };
 
    let secret = (*contants::SECRET).clone();
 
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
 
 /// Decodes a JWT token and retrieves the claims.
///
/// # Arguments
/// * `jwt` - The JWT token as a string.
///
/// # Returns
/// * `Ok(TokenData<Claims>)` - The extracted claims.
/// * `Err(jsonwebtoken::errors::Error)` - If decoding fails.
pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = (*contants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    claim_data
}