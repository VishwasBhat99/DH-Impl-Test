use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub average_balance: String,
    pub accr_int: String,
    pub base_rate: String,
    pub final_ftp_rate: String,
    pub value_date: String,
    pub mis1: String,
    pub mis2: String,
    pub psl_code: String,
    pub prod_code_type: String,
    pub rate_flag: String,
    pub currency: String,
    pub alm_line: String,
    pub current_outstanding_td: String,
    pub adj1: String,
    pub adj2: String,
    pub adj3: String,
    pub adj4: String,
    pub adj5: String,
    pub adj6: String,
    pub adj7: String,
    pub adj8: String,
    pub adj9: String,
    pub adj10: String,
    pub margin_amt: String,
    pub fixed_spread: String,
    pub variable_spread: String,
    pub aggr_keys: Vec<String>,
    pub aggr_keys_report2: Vec<String>,
    pub ftp_with_psl_amt: String,
    pub psl_amt: String,
    pub ftp_without_psl_amt: String,
    pub int_income_gl_amt: String,
    pub int_cancellation_gl_amt:String,
    pub overdue_int_gl_amt: String,
    pub gr_ofs_gl_amt: String,
    pub ui_ofs_gl_amt: String,
    pub re_ofs_gl_amt: String,
    pub is_ofs_gl_amt: String,
    pub woff_gl_amt:String,
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
