#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DynBucket {
    pub gl: String,
    pub days: i64,
}
