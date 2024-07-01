use rbdate::NaiveDate;

#[derive(Debug, Clone)]
pub struct CashflowDates {
    pub call_dt: NaiveDate,
    pub put_dt: NaiveDate,
    pub mat_dt: NaiveDate,
}
