use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub ccy: String,
    pub prod_code: String,
    pub mis1: String,
    pub gl_liab: String,
    pub gl_int_comp: String,
    pub concat: String,
    pub div: String,
    pub alm_line: String,
    pub ia_line: String,
    pub bal_lcy: String,
    pub int_comp: String,
    pub rate: String,
    pub rate_var: String,
    pub rate_var2: String,
    pub open_date: String,
    pub mat_date: String,
    pub cust_id: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account required fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account required fields json file was not well-formatted");
        req_fields
    }
}
