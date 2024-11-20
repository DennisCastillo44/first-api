use serde::{Deserialize, Serialize};
use crate::{lib_::hashing256::Hash, models::user::User};

type Result<T> = std::result::Result<T, LoginError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginError {
    PasswordIncorrect,
    UserUnknwon,
    ErrorUnknwon
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String, 
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub person_id: u32,
    pub user_id: u32,
}

impl LoginUser {

    pub fn new(username: &String, password: &String) -> LoginUser {
        LoginUser {username: username.to_string(), password: password.to_string()}
    }

    pub fn login(&self) -> Result<LoginResponse> {
    
        let user: User = User::new(&self.username, &self.password, None);
        let user_ = user.get_user_by_username()?;
        let password_: Vec<&str> = user_.password.split("??_").collect();
        let mut input_password = String::from(password_[1]);
        input_password.push_str(&self.password);
        let hash_ = Hash::new(input_password).generate_hash();

        let person_id = match user_.id_person {
            Some(id) => id as u32,
            None => 1u32
        };

        let user_id = match user_.id_user {
            Some(id) => id as u32,
            None => 1u32
        };

        if password_[0] == hash_ {
            Ok(LoginResponse {
                person_id: person_id,
                user_id: user_id,
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
            LoginError::UserUnknwon => write!(f, "user-unknwon"),
            _ => write!(f, "error-unknwon")
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