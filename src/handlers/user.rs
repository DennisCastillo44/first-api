use core::fmt;
use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::models::user_new::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct User { 
   pub username: String,
   pub password: String,
   pub person_id: Option<u64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub person_id: u64,
    pub user_id: u64,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserErrors {
    UserExists,
    UserUnknwon,
    UnknwonError
}

impl User {
    
    pub fn new(username: String, password: String, person_id: Option<u64>) -> User {
        let user = match person_id {
            Some(id) => User {username, password, person_id: Some(id)}, 
            None =>  User {username, password, person_id: None}
        };
        
        user
    }

    pub fn create(&self) -> Result<u64, UserErrors> {

        let iduser = user_new::UserModel::create_user(&self)?;
        Ok(iduser as u64)
    }

    pub fn get_user_by_username(username: String) -> Result<UserResponse, UserErrors> {

        let user = user_new::UserModel::get_user_by_username(username)?;
        Ok(UserResponse {
            user_id: user.user_id,
            person_id: user.person_id,
            username: user.username,
            password: user.password
        })
    }

    pub fn verify_user(&self) -> Result<bool, UserErrors> {

        let existuser = user_new::UserModel::verify_user(&self)?;
        if existuser {
            return Err(UserErrors::UserExists)
        }

        Ok(existuser)
    }
    /* pub fn get_user(&self) -> Result<UserResponse, rusqlite::Error> {
          
    } */
}

impl From<rusqlite::Error> for UserErrors {
    fn from(value: rusqlite::Error) -> UserErrors {
        match value {
            rusqlite::Error::QueryReturnedNoRows => UserErrors::UserUnknwon,
            _ => UserErrors::UnknwonError
        }
    }
}

impl fmt::Display for UserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserErrors::UserExists => write!(f,"{}", "user-exists"),
            UserErrors::UserUnknwon => write!(f, "user-unknown"),
            _ => write!(f, "{}", "unknown-error")
        }
    }
}

impl error::ResponseError for UserErrors {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            UserErrors::UserExists => StatusCode::CONFLICT,
            _=> StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(ErrorResponse {
            error_code: self.to_string()
        })
    }
}