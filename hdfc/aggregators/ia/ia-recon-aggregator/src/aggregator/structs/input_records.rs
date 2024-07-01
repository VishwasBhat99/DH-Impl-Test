#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct InputRecord {
    pub as_on: String,
    pub llg_id: i64,
    pub ccy_id: String,
    pub ex_rate: f64,
    pub bm: String,
    pub tenor: i64,
    pub spread: f64,
    pub rep_date: String,
    pub rep_freq: i64,
    pub bal: f64,
    pub rate: f64,
}
