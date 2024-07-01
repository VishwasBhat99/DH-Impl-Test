use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Debug, Deserialize)]
pub struct RequiredFields {
    pub concat: String,
    pub curr_code: String,
    pub intt_rate: String,
    pub prin_amt: String,
}

impl RequiredFields {
    pub fn new_from_path(_path: &str) -> RequiredFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: RequiredFields =
            serde_json::from_str(&buf).expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
