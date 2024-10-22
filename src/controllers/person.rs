use actix_web::{http::header::ContentType, web::{self}, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use crate::{models::person, types::inicio::ResponseApi};
use crate::types::person::Person;

pub async fn persons() -> Result<impl Responder> {

    let persons_ = person::PersonModel::get_persons().unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(persons_))
}

pub async fn register_person(data: web::Json<Person>) -> Result<impl Responder> {

    let last_create = person::PersonModel::create_person(&data).unwrap();
    let response_ = ResponseApi {person_id: last_create};
    //Ok(web::Json(ResponseApi {person_id: last_create}))
    Ok(HttpResponse::Created().content_type(ContentType::json()).json(response_))
}

pub async fn get_person(person: web::Path<u32>) -> Result<impl Responder> {

    let person_id = person.into_inner();
    let person_data = person::PersonModel::get_person(&person_id).unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(person_data))
}

pub async fn update_person(req:HttpRequest, person: web::Path<u32>, data: Option<web::Json<Person>>) -> Result<impl Responder> {

    #[derive(Debug, Serialize, Deserialize)]
    struct ResponseUpdate {
        updated: bool
    }

    let person_id = person.into_inner();
    let data_ = match data {
        Some(d) => person::PersonModel::update_person(&req.method().to_string(), &person_id, Some(&d)).unwrap(),
        None => person::PersonModel::update_person(&req.method().to_string(), &person_id, None).unwrap()
    };
 
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(ResponseUpdate {updated: data_}))
}