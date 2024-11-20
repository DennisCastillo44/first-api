use actix_web::{http::header::ContentType, web::{self}, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use crate::{lib_::hashing256::Hash, models::{person, user::{self}}};
use crate::types::person::{Person, User as typeUser};

pub async fn persons() -> Result<impl Responder> {

    let persons_ = person::PersonModel::get_persons().unwrap();
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(persons_))
}

pub async fn register_person(data: web::Json<Person>) -> Result<impl Responder> {

    #[derive(Serialize, Deserialize, Debug)]
    struct ResponseCreatePerson {
        id_person: u64,
        id_user: u64
    }

    let mut responseCreate = ResponseCreatePerson {
        id_person: 0,
        id_user: 0
    };

    let user_data = match &data.user_data {
        Some(t) => t,
        None => &typeUser {
            username: String::from(""),
            password: String::from("")
        }
    };
    
    match person::PersonModel::create_person(&data) {
        Ok(person) => {
            
            responseCreate.id_person = person as u64;
            let mut new_user = user::User::new(&user_data.username, &user_data.password, Some(person as u64));
            match new_user.create_user() {
                Ok(result) => responseCreate.id_user = result as u64,
                Err(_) => println!("No se pudo crear al usuario")
            }
        },
        Err(_) => print!("No se logró crar persona")
    };
    
    Ok(HttpResponse::Created().content_type(ContentType::json()).json(responseCreate))
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