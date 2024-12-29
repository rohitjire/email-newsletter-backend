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

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }

    let token: String = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer", "")
        .trim()
        .to_owned();
 
    let claim = decode_jwt(token.clone()).map_err(|err| {
        eprintln!("JWT decoding error: {:?}", err);
        Error::from(api_response::ApiResponse::new(401, "Invalid token".to_string()))
    })?;
 
    req.extensions_mut().insert(claim.claims);
    
    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
