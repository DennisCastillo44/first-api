use std::fmt;
use serde::{Deserialize, Serialize};
use crate::models::person_new::{PersonModel, PersonObject};
use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub enum PersonError {
    ErrorPerson,
    CreatePersonError,
    UnknwonPerson,
    UpdatePersonError,
    DeletePersonError
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub active: bool,
}

impl Person {
    
    pub fn new(first_name: String, last_name: String, birthday: String, active: bool) -> Person {
        Person {first_name, last_name, birthday, active}
    }

    pub fn create(&self) -> Result<u64, PersonError> {

        match PersonModel::create_person(&self) {
            Ok(p) => Ok(p as u64),
            Err(_) => Err(PersonError::CreatePersonError)
        }
    }

    pub fn get_persons() -> Result<Vec<PersonObject>, PersonError> {

        let persons = PersonModel::get_persons()?;

        Ok(persons)
    }

    pub fn get_person(personid: u64) -> Result<PersonObject, PersonError> {

        let person = PersonModel::get_person(&personid)?;

        Ok(person)
    }

    pub fn update_person(&self, personid: &u64) -> Result<bool, PersonError> {
        
        match PersonModel::update_person(&personid, &self) {
            Ok(_) => Ok(true),
            Err(_) => Err(PersonError::UpdatePersonError)
        }
    }

    pub fn delete_person(personid: u64) -> Result<bool, PersonError> {
        
        match PersonModel::delete_person(personid) {
            Ok(deleted) => Ok(deleted),
            Err(_) => Err(PersonError::DeletePersonError) 
        }
    }

    pub fn verify_person(personid: u64) -> Result<bool, PersonError> {

       let exists = PersonModel::verify_person(personid)?;

       if exists {
            Ok(exists)
       } else {
           Err(PersonError::UnknwonPerson)
       }

    }
}

impl fmt::Display for PersonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PersonError::ErrorPerson => write!(f, "person-process-error"),
            PersonError::CreatePersonError => write!(f, "person-create-error"),
            PersonError::UnknwonPerson => write!(f, "unknown-person"),
            PersonError::UpdatePersonError => write!(f, "person-update-error"),
            PersonError::DeletePersonError => write!(f, "person-delete-error")
        }
    }
}

impl From<rusqlite::Error> for PersonError {
    fn from(value: rusqlite::Error) -> PersonError {
        match value {
            rusqlite::Error::QueryReturnedNoRows => PersonError::UnknwonPerson,
            _ => PersonError::ErrorPerson
        }
    }
}

impl error::ResponseError for PersonError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            PersonError::UnknwonPerson => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }

    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(ErrorResponse {
            error_code: self.to_string()
        })
    }
}