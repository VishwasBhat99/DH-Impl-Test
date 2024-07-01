use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct InputFields {
    pub input_file: String,
    pub cust_id_field: String,
    pub amount_field: String,
    pub metadata_files: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub master_file: String,
    pub claim_id_file: String,
    pub input_files: Vec<InputFields>,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the Input config  File");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames =
            serde_json::from_str(&buf[..]).expect("Input config file was not well-formatted");
        req_fields
    }
}
