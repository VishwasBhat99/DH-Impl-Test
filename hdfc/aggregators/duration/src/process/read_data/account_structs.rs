use super::super::chrono::NaiveDate;
use std::collections::HashMap;

pub struct AccountInfo {
    pub as_on_dt: NaiveDate,
    pub llg_id: String,
    pub account_number: String,
    pub ccy_id: String,
    pub bal_amt_ccy: f64,
    pub bal_amt_hcy: f64,
    pub duration: f64,
}

impl AccountInfo {
    pub fn display(&self, delimiter: &str) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}",
            self.as_on_dt,
            delimiter,
            self.llg_id,
            delimiter,
            self.account_number,
            delimiter,
            self.ccy_id,
            delimiter,
            self.bal_amt_ccy,
            delimiter,
            self.bal_amt_ccy,
            delimiter,
            self.duration
        )
    }
}
#[derive(PartialEq, Eq, Hash)]
pub struct AggrKey {
    pub llg_id: String,
    pub currency_id: String,
}

#[derive(Clone)]
pub struct AggrData {
    pub balance: f64,
    pub weighted_avg: f64,
}
pub type Accounts = HashMap<AggrKey, AggrData>;
