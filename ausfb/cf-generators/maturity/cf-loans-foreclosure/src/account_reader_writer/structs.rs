use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct CashflowData {
    pub interest_flow: f64,
    pub principal_flow: f64,
    pub flow_date: NaiveDate,
}
