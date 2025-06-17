use serde::{Deserialize, Serialize};
use crate::{lib_::hashing256::Hash, models::{user::User, user_new}};
use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse, HttpResponseBuilder};

//type Result<T> = std::result::Result<T, LoginError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginError {
    PasswordIncorrect,
    UserUnknwon,
    ErrorUnknwon
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String, 
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub person_id: u64,
    pub user_id: u64,
}

impl LoginUser {

    pub fn new(username: &String, password: &String) -> LoginUser {
        LoginUser {username: username.to_string(), password: password.to_string()}
    }

    pub fn login(&self) -> Result<LoginResponse, LoginError> {
    
        let username = &self.username;
        let user = user_new::UserModel::get_user_by_username(username.to_string())?;
        let password_: Vec<&str> = user.password.split("??_").collect();
        let mut input_password = String::from(password_[1]);
        input_password.push_str(&self.password);
        let hash_ = Hash::new(input_password).generate_hash();

        if password_[0] == hash_ {
            Ok(LoginResponse {
                person_id: user.person_id,
                user_id: user.user_id,
                //session_date: session_datetime.to_string()
            })
        } else {
            Err(LoginError::PasswordIncorrect)
        }
    }
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::PasswordIncorrect => write!(f, "password-incorrect"),
            LoginError::UserUnknwon => write!(f, "user-unknown"),
            _ => write!(f, "error-unknown")
        }
    }
}

impl From<rusqlite::Error> for LoginError {
    fn from(err: rusqlite::Error) -> LoginError {
        match err {
            rusqlite::Error::QueryReturnedNoRows => LoginError::UserUnknwon,
            _ => LoginError::ErrorUnknwon
        }
    }
}

impl error::ResponseError for LoginError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            LoginError::PasswordIncorrect | LoginError::UserUnknwon => StatusCode::UNAUTHORIZED,
            _=> StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(ErrorResponse {
            error_code: self.to_string()
        })
    }
}