use core::fmt;
use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, error, http::{header::{ContentType, AUTHORIZATION}, StatusCode}, middleware::{ErrorHandlerResponse, Next}, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorMiddleware {
    TokenError,
    TokenErrorExpiredError
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthErrorResponse {
    error_code: String
}

pub async fn token_middleware(request: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {

    let token = request.headers().get(AUTHORIZATION);
    if token.is_none() {
        //Err(ErrorMiddleware::TokenError);
        
    }

    next.call(request).await
}

impl fmt::Display for ErrorMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorMiddleware::TokenError => write!(f, "token-not-provider"),
            ErrorMiddleware::TokenErrorExpiredError => write!(f, "token-expired")
        }
    }
}

impl error::ResponseError for ErrorMiddleware {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(AuthErrorResponse {
            error_code: self.to_string()
        })
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::BAD_REQUEST
    }
}

