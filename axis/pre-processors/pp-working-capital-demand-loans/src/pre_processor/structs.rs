use chrono::NaiveDate;

#[derive(Debug)]
pub struct NpaData {
    pub classification: String,
    pub npa_amt: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BalmGamKey {
    pub acid: String,
    pub foracid: String,
}

#[derive(Debug, Clone)]
pub struct BalmGamValue {
    pub acid: String,
    pub clr_bal_amt: f64,
    pub cust_id: String,
    pub gl_sub_head_code: String,
    pub schm_code: String,
    pub acct_crncy_code: String,
    pub acct_open_date: NaiveDate,
}

impl BalmGamValue {
    pub fn new() -> BalmGamValue {
        BalmGamValue {
            acid: "".to_string(),
            clr_bal_amt: 0.0,
            cust_id: "".to_string(),
            gl_sub_head_code: "".to_string(),
            schm_code: "".to_string(),
            acct_crncy_code: "".to_string(),
            acct_open_date: NaiveDate::default(),
        }
    }
}
