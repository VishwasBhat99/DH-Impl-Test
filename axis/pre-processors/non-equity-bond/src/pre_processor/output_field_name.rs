use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug)]
pub struct AccFieldNames {
    pub AsOnDate: NaiveDate,
    pub RptItem: String,
    pub RptDesc: String,
    pub FaceAmt: f64,
    pub IssueDate: NaiveDate,
    pub OutsdngAmt: f64,
    pub MatDate: NaiveDate,
    pub Coupons: f64,
    pub OpenPrice: f64,
    pub HighPrice: f64,
    pub LowPrice: f64,
    pub ClosePrice: f64,
}
