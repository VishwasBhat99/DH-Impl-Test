#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputTenor {
    pub as_on: String,
    pub llg_id: i64,
    pub tenor_id: i64,
    pub tenor_desc: String,
    pub tenor_st_yrs: i64,
    pub tenor_st_mons: i64,
    pub tenor_st_days: i64,
    pub tenor_end_yrs: i64,
    pub tenor_end_mons: i64,
    pub tenor_end_days: i64,
    pub is_in_use: String,
}
