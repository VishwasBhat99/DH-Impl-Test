use chrono::NaiveDate;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct DerivatieKey {
    pub country: String,
    pub exposure_id: String,
    pub tenor: String,
}

#[derive(Debug, Clone)]
pub struct DerivatieValue {
    pub actual_country: String,
    pub fic_mis_date: NaiveDate,
    pub cust_id: String,
    pub d_mat_date: NaiveDate,
    pub schme_code_product: String,
    pub cust_name: String,
    pub native_ccy: String,
    pub native_mtm_amt: f64,
    pub cons_mtm_amt: f64,
    pub native_amt: f64,
    pub cons_amt: f64,
    pub cons_inr_mtm_amt: f64,
    pub cons_inr_notional_amt: f64,
    pub residual_days: i64,
    pub tenor: String,
    pub exchange_rt: f64,
}
