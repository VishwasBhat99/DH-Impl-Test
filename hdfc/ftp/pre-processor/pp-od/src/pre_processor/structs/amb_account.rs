#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AMB {
    pub cod_acc_no: String,
    pub cr_avg_bal: String,
    pub dr_avg_bal: f64,
    pub source: String,
    pub br_no: String,
    pub prod: String,
    pub mis1: String,
    pub mis2: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AMBData {
    pub dr_avg_bal: f64,
    pub mis2: String,
}
