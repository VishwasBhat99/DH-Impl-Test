#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acc_no: String,
    pub branch_cd: String,
    pub cust_no: String,
    pub ucc_id: String,
    pub ccy: String,
    pub produ: String,
    pub gl: String,
    pub open_dt: String,
    pub os_bal: String,
    pub os_bal_cry: String,
    pub int_rt: String,
    pub int_type: String,
    pub int_bm: String,
    pub spread: String,
    pub inoperative: String,
    pub int_accrd: String,
    pub const_cd: String,
    pub const_desc: String,
    pub as_on: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.acc_no,
            self.branch_cd,
            self.cust_no,
            self.ucc_id,
            self.ccy,
            self.produ,
            self.gl,
            self.open_dt,
            self.os_bal,
            self.os_bal_cry,
            self.int_rt,
            self.int_type,
            self.int_bm,
            self.spread,
            self.inoperative,
            self.int_accrd,
            self.const_cd,
        )
    }
}
