use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub account_id: String,
    pub acc_open_date: String,
    pub acc_crncy_code: String,
    pub out_bal: String,
    pub out_bal_lcy: String,
    pub maturity_date: String,
    pub interest_rate: String,
    pub next_reprise_date: String,
    pub last_reprise_date: String,
    pub gl_code: String,
    pub scheme_code: String,
    pub customer_id: String,
    pub customer_type: String,
    pub cust_const_code: String,
    pub customer_name: String,
    pub tot_int_amt: String,
    pub tot_prin_amt: String,
    pub pt_f64_1: String,
    pub pt_f64_2: String,
    pub pt_f64_3: String,
    pub pt_f64_4: String,
    pub pt_i64_1: String,
    pub pt_i64_2: String,
    pub pt_i64_3: String,
    pub pt_i64_4: String,
    pub pt_str_1: String,
    pub pt_str_2: String,
    pub pt_str_3: String,
    pub pt_str_4: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path)
            .expect("Cannot open the account metadata/req-fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames =
            serde_json::from_str(&buf[..]).expect("Required fields file was not well-formatted");
        req_fields
    }
}
