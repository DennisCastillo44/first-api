use actix_web::{http::header::ContentType, web::{self}, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use crate::{handlers::{person::Person, user::User}, models::person};
use crate::handlers::person as person_;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRequest {
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub active: bool,
    pub user_data: UserRequest
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseCreatePerson {
    id_person: u64,
    id_user: u64
}

#[derive(Debug, Serialize, Deserialize)]
struct DeleteResponse {
    delete: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseUpdate {
    updated: bool
}

pub async fn persons() -> Result<impl Responder> {

    let persons_ = person_::Person::get_persons()?; //person::PersonModel::get_persons().unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(persons_))
}

pub async fn register_person(data: web::Json<PersonRequest>) -> Result<impl Responder> {
    
    User::new(data.user_data.username.to_string(), data.user_data.password.to_string(), None).verify_user()?;
    let person = person_::Person::new(data.first_name.to_string(), data.last_name.to_string(), data.birthday.to_string(), data.active);
    let idperson = person.create()?;
    let user = User::new(data.user_data.username.to_string(), data.user_data.password.to_string(), Some(idperson));
    let userid = user.create()?;
    
    Ok(HttpResponse::Created().content_type(ContentType::json()).json(ResponseCreatePerson {
        id_person: idperson,
        id_user: userid
    }))
}

pub async fn get_person(person: web::Path<u64>) -> Result<impl Responder> {

    let person_id = person.into_inner();
    let person_data = person_::Person::get_person(person_id)?; //person::PersonModel::get_person(&person_id).unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(person_data))
}

pub async fn update_person(personid: web::Path<u64>, data: web::Json<Person>) -> Result<impl Responder> {
    
    let person_id = personid.into_inner();
    person_::Person::verify_person(person_id)?;
    let person = person_::Person::new(data.first_name.to_string(), data.last_name.to_string(), data.birthday.to_string(), data.active);
    let update = person.update_person(&person_id)?;
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(ResponseUpdate {updated: update}))
}

pub async fn delete_person(person: web::Path<u64>) -> Result<impl Responder>  {

    let person_id = person.into_inner();
    person_::Person::verify_person(person_id)?;
    let delete = person_::Person::delete_person(person_id)?;
    Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(DeleteResponse {
        delete: delete
    }))
}