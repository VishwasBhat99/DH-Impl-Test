use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub currency: String,
    pub int_rate: String,
    pub amount: String,
}

impl AccFieldNames {
    pub fn new_from_path(path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Can not read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("required field json file was not well-formatted");
        req_fields
    }
}
