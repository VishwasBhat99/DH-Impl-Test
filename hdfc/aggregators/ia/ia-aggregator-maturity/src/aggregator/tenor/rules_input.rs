#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputRules {
    pub step1: i64,
    pub step2: i64,
    pub step3: i64,
    pub lhs: String,
    pub comp: String,
    pub rhs: String,
    pub llg_id: i64,
}
