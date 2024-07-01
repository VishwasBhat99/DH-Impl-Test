use sdb_io::open_file_read;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_open_dt: String,
    pub initial_bal: String,
    pub prod_code: String,
    pub acc_id: String,
    pub cust_id: String,
    pub cust_name: String,
    pub division: String,
    pub mat_date: String,
    pub ccy: String,
    pub roi: String,
    pub benchmark: String,
    pub alm_line: String,
}

impl AccFieldNames {
    pub fn new_from_path(path: &str) -> AccFieldNames {
        let mut file = open_file_read(path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
