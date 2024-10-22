use serde::{Serialize, Deserialize};
//pub use crate::models::assignatures;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub active: bool,
    pub contact: Option<Contact>
    //pub assignatures: Vec<assignatures::Assignatures>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub email: String,
    pub phone: String
}

