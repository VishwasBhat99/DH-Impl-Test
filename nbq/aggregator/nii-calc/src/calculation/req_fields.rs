use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReqFields {
    pub acc_id: String,
    pub branch_id: String,
    pub customer_id: String,
    pub prd_code: String,
    pub gl_code: String,
    pub currency: String,
    pub amount_lcy: String,
    pub amount_ccy: String,
    pub int_rate: String,
    pub int_bm_code: String,
    pub int_bm_rate: String,
    pub spread: String,
    pub min_rate: String,
    pub max_rate: String,
}

impl ReqFields {
    pub fn new_from_path(_path: &str) -> ReqFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: ReqFields = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
