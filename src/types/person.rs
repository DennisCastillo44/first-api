use serde::{Serialize, Deserialize};
//pub use crate::models::assignatures;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub active: bool,
    pub contact: Option<Contact>,
    pub user_data: Option<User>
    //pub assignatures: Vec<assignatures::Assignatures>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub email: String,
    pub phone: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String
}