/// Module for handling API responses in Actix-web.
///
/// Provides a structured response format with status codes and error handling.
use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};
use  std::fmt::Display;

/// Represents an API response with a status code and a message body.
#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

/// Creates a new `ApiResponse`.
    ///
    /// # Arguments
    /// * `status_code` - The HTTP status code for the response.
    /// * `body` - The response message body.
impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body: BoxBody = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }
 
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}
 
impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} \n Status Code: {}", self.body, self.status_code)
    }
}
