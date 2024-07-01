#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputRecord {
    pub as_on: String,
    pub country: String,
    pub ccy: String,
    pub llg_id: i64,
    pub lcy_amt: f64,
    pub fcy_amt: f64,
}
