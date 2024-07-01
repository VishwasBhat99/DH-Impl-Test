extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillSCFM {
    pub invoice_no: String,
    pub owner_id: String,
    pub owner_name: String,
    pub bills_os_id: String,
    pub currency: String,
    pub bills_outstanding: String,
    pub acct_open_date: String,
    pub maturity_date: String,
    pub interest_type: String,
    pub roi: String,
    pub next_repricing_date: String,
    pub last_repricing_date: String,
    pub repricing_frequency: String,
    pub benchmark: String,
    pub npa_classification: String,
    pub cust_classification: String,
    pub gl_code: String,
    pub constitution: String,
    pub segment_code: String,
    pub npa_amount: String,
    pub scfm_foracid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillsGL {
    pub gl_sub_head_code: String,
    pub foracid: String,
    pub cust_id: String,
    pub schm_code: String,
    pub schm_type: String,
    pub clr_bal_amt: String,
    pub un_clr_bal_amt: String,
    pub acct_crncy_code: String,
}
