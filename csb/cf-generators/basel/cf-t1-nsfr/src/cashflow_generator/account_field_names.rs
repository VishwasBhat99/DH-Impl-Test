use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_id: String,
    pub cust_id: String,
    pub prod_code: String,
    pub ost_bal: String,
    pub int_rate: String,
    pub currency: String,
    pub pt_str_1: String,
    pub pt_str_2: String,
    pub pt_str_3: String,
    pub pt_str_4: String,
    pub pt_str_5: String,
    pub pt_int_1: String,
    pub pt_int_2: String,
    pub pt_int_3: String,
    pub pt_int_4: String,
    pub pt_int_5: String,
    pub pt_f64_1: String,
    pub pt_f64_2: String,
    pub pt_f64_3: String,
    pub pt_f64_4: String,
    pub pt_f64_5: String,
    pub cashflows: String,
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
