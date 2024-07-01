use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_id: String,
    pub cust_id: Option<String>,
    pub currency: Option<String>,
    pub amount: Option<String>,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the Required Fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Required Fields json file was not well-formatted");
        req_fields
    }
}
