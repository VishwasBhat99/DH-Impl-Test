use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub amount: String,
    pub interest_rate: String,
    pub account_open_date: String,
    pub pass_through: Vec<String>,
}

impl AccFieldNames {
    pub fn new_from_path(path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
