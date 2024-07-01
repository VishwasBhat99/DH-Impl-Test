use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acid: String,
    pub foracid: String,
    pub cust_id: String,
    pub cust_name: String,
    pub schm_code: String,
    pub schm_type: String,
    pub acct_crncy_code: String,
    pub acct_open_date: String,
    pub out_bal_amt_con: String,
    pub nfs: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
