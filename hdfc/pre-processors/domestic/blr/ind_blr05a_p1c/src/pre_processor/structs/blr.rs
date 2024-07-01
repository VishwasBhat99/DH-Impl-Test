#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BLRInput {
    pub name: String,
    pub desc: String,
    pub face_value: String,
    pub pending_dt: String,
    pub opening_bal: String,
    pub highst_price_dt: String,
    pub highst_price_amt: String,
    pub lowst_price_dt: String,
    pub lowst_price_amt: String,
    pub closng_dt: String,
    pub closng_amt: String,
    pub std_deviation: String,
}

impl BLRInput {
    pub fn print(
        &self,
        opening_bal: f64,
        highst_price_amt: f64,
        lowst_price_amt: f64,
        closng_amt: f64,
        as_on_date: String,
    ) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            as_on_date,
            self.name,
            self.desc,
            self.face_value,
            opening_bal,
            self.highst_price_dt,
            highst_price_amt,
            self.lowst_price_dt,
            lowst_price_amt,
            closng_amt,
            self.std_deviation,
        )
    }
}
