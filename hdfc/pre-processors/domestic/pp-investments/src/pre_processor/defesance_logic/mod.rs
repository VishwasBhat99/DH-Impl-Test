pub mod implemenation;
pub mod split;

use rbdate::NaiveDate;

#[derive(Debug)]
pub struct TradingAccount {
    pub mat_dt: NaiveDate,
    pub acc_pt: String,
}
