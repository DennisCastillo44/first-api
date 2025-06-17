use core::fmt;
use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use serde::{Deserialize, Serialize};
use super::{login::LoginError, person::PersonError, user::UserErrors};

#[derive(Debug, Serialize, Deserialize)]
pub struct Responder_<T> {
    pub response: T,
    pub code: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponder_ {
    error_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorResponses {
    Login(LoginError),
    Person(PersonError),
    User(UserErrors)
}

impl<T> Responder_<T> {
    
    /* pub fn new(response: T, code: u16) -> Self {
        Responder_ { response, code }
    } */
}

//impl error::ResponseError for LoginError {}
//impl error::ResponseError for PersonError {}
//impl error::ResponseError for UserErrors {}

impl error::ResponseError for ErrorResponses {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(ErrorResponder_ {
            error_code: self.to_string()
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ErrorResponses::Login(LoginError::PasswordIncorrect) | ErrorResponses::Login(LoginError::UserUnknwon) => StatusCode::UNAUTHORIZED,
            ErrorResponses::User(UserErrors::UserExists) => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl fmt::Display for ErrorResponses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponses::Login(err) => write!(f, "{}", err),
            ErrorResponses::Person(err) => write!(f, "{}", err),
            ErrorResponses::User(err) => writeln!(f, "{}", err)
        }
    }
}
