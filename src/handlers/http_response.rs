use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use serde::{Deserialize, Serialize};
use super::login::LoginError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Responder_<T> {
    pub response: T,
    pub code: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponder_ {
    error_code: String
}

impl<T> Responder_<T> {
    
    pub fn new(response: T, code: u16) -> Self {
        Responder_ { response, code }
    }
}

impl error::ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(ErrorResponder_ {
            error_code: self.to_string()
        })
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            LoginError::PasswordIncorrect | LoginError::UserUnknwon => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
