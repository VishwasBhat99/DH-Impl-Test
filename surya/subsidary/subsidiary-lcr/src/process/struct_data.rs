use rbdate::NaiveDate;

pub struct Data {
    pub as_on_date: String,
    pub subsidiary_id: String,
    pub currency: String,
    pub llg_code: String,
    pub outbal_con: f64,
    pub outbal_fcy: f64,
}

impl Data {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.as_on_date,
            self.subsidiary_id,
            self.currency,
            self.llg_code,
            self.outbal_con,
            self.outbal_fcy
        )
    }
}
