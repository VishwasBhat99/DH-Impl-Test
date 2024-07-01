use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub branch: String,
    pub product: String,
    pub sub_product_code: String,
    pub customer_id: String,
    pub customer_grp_id: String,
    pub customer_name: String,
    pub customer_title: String,
    pub gl_code_1: String,
    pub gl_code_2: String,
    pub gl_code_3: String,
    pub gl_code_4: String,
    pub loan_subtype: String,
    pub repayment_frequency: String,
    pub guarantor_id: String,
    pub guarantor_name: String,
    pub loan_sanction_date: String,
    pub account_value_date: String,
    pub loan_disbursement_date: String,
    pub account_maturity_date: String,
    pub currency: String,
    pub ost_bal_ccy: String,
    pub ost_bal_lcy: String,
    pub purpose_of_loan: String,
    pub ltv: String,
    pub is_restruct: String,
    pub last_restructured_date: String,
    pub rating: String,
    pub internal_rating: String,
    pub external_rating_agenecy: String,
    pub external_rating: String,
    pub pd: String,
    pub cust_category: String,
    pub sector: String,
    pub industry: String,
    pub cust_class_1: String,
    pub cust_class_2: String,
    pub iis_amt: String
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path)
            .expect("Cannot open the account req fields metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
