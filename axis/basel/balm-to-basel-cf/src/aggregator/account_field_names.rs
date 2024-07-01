use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
#[derive(Debug, Deserialize, Serialize)]
pub struct ReqFields {
    pub acid: String,
    pub currency: String,
    pub out_bal_amt: String,
}

impl ReqFields {
    pub fn new_from_path(_path: &str) -> ReqFields {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the required Fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read required field input as string");
        let req_fields: ReqFields = serde_json::from_str(&buf[..])
            .expect("required fields json file was not well-formatted");
        req_fields
    }
}
