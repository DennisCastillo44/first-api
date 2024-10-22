use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
    pub name: String,
    pub age: u8,
    pub live: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseApi {
    pub person_id: i64
}