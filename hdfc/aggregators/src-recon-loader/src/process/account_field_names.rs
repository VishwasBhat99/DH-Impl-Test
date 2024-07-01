use sdb_io;
use std::io::Read;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug)]
pub struct InputFields {
    pub source: String,
    pub llg_id: String,
    pub gl_code: String,
    pub currency: String,
    pub lcy_amount: String,
    pub master_gl_code_col: usize,
    pub master_gl_desc_col: usize
}

impl InputFields {
    pub fn new_from_path(_path: &str) -> InputFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account required fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as usize");
        let req_fields: InputFields = serde_json::from_str(&buf[..])
            .expect("Account required fields json file was not well-formatted");
        req_fields
    }
}
