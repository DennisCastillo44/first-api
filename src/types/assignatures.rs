use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignatures {
    pub id: u16,
    pub name: String,
    pub credits: u8
}