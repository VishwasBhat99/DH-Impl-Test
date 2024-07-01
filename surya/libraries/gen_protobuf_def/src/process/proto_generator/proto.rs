use super::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Feild>,
}
#[derive(Serialize, Deserialize)]
pub struct Feild {
    pub name: String,
    pub position: i32,
    pub typ: String,
}
