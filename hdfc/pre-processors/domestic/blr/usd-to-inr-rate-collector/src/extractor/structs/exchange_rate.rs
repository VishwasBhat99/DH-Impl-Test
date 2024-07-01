#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExchangeRate {
    pub source_ccy: String,
    pub target_ccy: String,
    pub rate: f64,
}
