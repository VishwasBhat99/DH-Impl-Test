use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_no: String,
    pub acc_open_dt: String,
    pub val_dt: String,
    pub mat_dt: String,
    pub ccy: String,
    pub prod_code: String,
    pub mis1: String,
    pub gl_liability: String,
    pub gl_int_comp: String,
    pub concat: String,
    pub division: String,
    pub alm_line: String,
    pub ia_line: String,
    pub rate: String,
    pub rate_var: String,
    pub rate_var2: String,
    pub amt_initl_deposit: String,
    pub cust_id: String,
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
