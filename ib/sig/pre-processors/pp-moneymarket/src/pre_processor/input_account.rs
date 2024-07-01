extern crate serde;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub rpt_dt: String,
    pub br: String,
    pub dealno: String,
    pub asset_liability: String,
    pub cust_no: String,
    pub cust: String,
    pub prodtype: String,
    pub deal_dt: String,
    pub vdate: String,
    pub mdate: String,
    pub rdate: String,
    pub ccy: String,
    pub principal: String,
    pub rate: String,
    pub amt_gd: String,
    pub int_rt: String,
    pub accr_int: String,
    pub gl_code: String,
    pub cust_type: String,
    pub brca: String,
    pub acct_suffix: String,
}
