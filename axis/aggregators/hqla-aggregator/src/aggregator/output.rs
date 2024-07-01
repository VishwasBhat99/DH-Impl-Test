use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    pub isin_no: String,
    pub book_category: String,
    pub updated_book_category: String,
    pub actual_fv: f64,
    pub actual_bv: f64,
    pub actual_mv: f64,
    pub pledge_fv: f64,
    pub pledge_bv: f64,
    pub pledge_mv: f64,
    pub out_fv: f64,
    pub out_bv: f64,
    pub out_mv: f64,
    pub mat_date: NaiveDate,
    pub coupon: f64,
    pub repo_mat_date: NaiveDate,
    pub accrued_days: i64,
    pub accrued_interest: f64,
    pub dirty_price: f64,
    pub appr_depr: f64,
}
