#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyConverter {
    pub source: String,
    pub target: String,
    pub ex_rt: f64,
}
