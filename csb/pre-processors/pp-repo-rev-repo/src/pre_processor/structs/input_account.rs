#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub deal_no: String,
    pub book_value: String,
    pub ccy: String,
    pub cntr_party_id: String,
    pub cntr_party_name: String,
    pub cntr_party_type: String,
    pub repo_dt: String,
    pub repo_mat_dt: String,
    pub int_rate: String,
    pub int_amt: String,
    pub treas_gl_cd: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.deal_no,
            self.book_value,
            self.ccy,
            self.cntr_party_id,
            self.cntr_party_name,
            self.cntr_party_type,
            self.repo_dt,
            self.repo_mat_dt,
            self.int_rate,
            self.int_amt,
            self.treas_gl_cd,
        )
    }
}
