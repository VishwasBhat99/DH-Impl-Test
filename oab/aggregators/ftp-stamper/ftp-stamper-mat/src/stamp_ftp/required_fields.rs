use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Debug, Deserialize)]
pub struct RequiredFields {
    pub account_number: String,
    pub currency: String,
    pub amount: String,
    pub int_rate: String,
    pub rate_flag: String,
    pub value_date: String,
    pub open_date: String,
    pub mat_date: String,
    pub lst_reprice_date: String,
    pub nxt_reprice_date: String,
    pub rep_freq: String,
    pub a_or_l: String,
    pub dim1: String,
    pub dim2: String,
    pub dim3: String,
    pub dim4: String,
    pub customer_id: String,
    pub rl1: String,
    pub rl2: String,
    pub rl3: String,
    pub gl_code: String,
    pub prod_code: String,
    pub div_code: String,
    pub mis_code_1: String,
    pub mis_code_2: String,
    pub mis_code_3: String,
    pub eop_bal_ccy: String,
    pub cashflows: String,
}

impl RequiredFields {
    pub fn new_from_path(_path: &str) -> RequiredFields {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the required fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string from required fields file");
        let req_fields: RequiredFields =
            serde_json::from_str(&buf).expect("Required fields json file was not well-formatted");

        req_fields
    }
}
