use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{clone, io::Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqiredFields {
    pub input_rec_banking_row: String,
    pub input_rec_trading_row: String,
    pub input_pay_banking_row: String,
    pub input_pay_trading_row: String,
    pub sheet_name: String,
    pub currency: String,
}

impl ReqiredFields {
    pub fn new_from_path(_path: &str) -> ReqiredFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Unable to open Required-Config-Fields-File");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string from Req-Fields File");
        let req_fields: ReqiredFields = serde_json::from_str(&buf[..])
            .expect("Required-Config-Fields-File was not well-formatted");
        req_fields
    }
}
