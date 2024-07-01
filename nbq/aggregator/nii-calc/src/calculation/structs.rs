use rbdate::NaiveDate;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct OutputData {
    pub source_name: String,
    pub as_on: String,
    pub analysis_no: String,
    pub llg_id: i32,
    pub acc_id: String,
    pub branch_id: String,
    pub customer_id: String,
    pub prd_code: String,
    pub gl_code: String,
    pub currency: String,
    pub amount_lcy: f64,
    pub amount_ccy: String,
    pub int_rate: f64,
    pub int_bm_code: String,
    pub int_bm_rate: String,
    pub spread: f64,
    pub min_rate: f64,
    pub max_rate: String,
    pub new_rate: f64,
    pub rate_diff: f64,
    pub impact_amt: f64,
}

impl Display for OutputData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.source_name,
            self.as_on,
            self.analysis_no,
            self.llg_id,
            self.acc_id,
            self.branch_id,
            self.customer_id,
            self.prd_code,
            self.gl_code,
            self.currency,
            self.amount_lcy,
            self.amount_ccy,
            self.int_rate,
            self.int_bm_code,
            self.int_bm_rate,
            self.spread,
            self.min_rate,
            self.max_rate,
            self.new_rate,
            self.rate_diff,
            self.impact_amt
        )
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct BMRatesKey {
    pub as_on: NaiveDate,
    pub bm_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BMRatesValue {
    pub analysis_no: String,
    pub rate: f64,
}
