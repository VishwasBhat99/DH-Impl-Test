use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]

pub struct PropertyDescriptor {
    pub name: String,
    pub typ: String,
    pub position: usize,
    pub start_pos: usize,
    pub max_len: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMetadata {
    pub fields: Vec<PropertyDescriptor>,
}
impl AccountMetadata {
    pub fn new_from_path(path: &str) -> AccountMetadata {
        let mut file = sdb_io::open_file_read(path).expect("Cannot open the metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let metadata_file: AccountMetadata = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        metadata_file
    }
}
