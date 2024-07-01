use rbdate::NaiveDate;

#[derive(Clone, Debug)]
pub struct RepaySchedData {
    pub cf_date: NaiveDate,
    pub principal_amount: f64,
    pub interest_amount: f64,
}
