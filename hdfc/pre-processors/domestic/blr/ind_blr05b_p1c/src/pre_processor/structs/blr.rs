#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BLRInput {
    pub type_of_instrument: String,
    pub face_value: String,
    pub date_of_issue: String,
    pub amount_outstanding: String,
    pub date_of_maturity: String,
    pub coupon: String,
    pub opening: String,
    pub high: String,
    pub low: String,
    pub closing: String,
}

impl BLRInput {
    pub fn print(
        &self,
        face_value: f64,
        amount_outstanding: f64,
        as_on_date: String,
    ) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            as_on_date,
            self.type_of_instrument,
            self.type_of_instrument,
            face_value,
            self.date_of_issue,
            amount_outstanding,
            self.date_of_maturity,
            self.coupon,
            self.opening,
            self.high,
            self.low,
            self.closing,
        )
    }
}
