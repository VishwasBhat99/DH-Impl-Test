use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub ca_cust_id: String,
    pub sa_cust_id: String,
    pub td_cust_id: String,
    pub rd_cust_id: String,
    pub ca_mat_date: String,
    pub sa_mat_date: String,
    pub td_mat_date: String,
    pub rd_mat_date: String,
    pub ca_const_code: String,
    pub sa_const_code: String,
    pub td_const_code: String,
    pub rd_const_code: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the account json file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames =
            serde_json::from_str(&buf[..]).expect("Account json file was not well-formatted");
        req_fields
    }
}
