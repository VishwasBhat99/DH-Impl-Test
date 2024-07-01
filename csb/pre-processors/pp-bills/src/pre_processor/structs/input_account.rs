#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub bill_id: String,
    pub client_id: String,
    pub clients_name: String,
    pub lbm_gl: String,
    pub prod_cd: String,
    pub bal: String,
    pub curr: String,
    pub bal_lcy: String,
    pub open_dt: String,
    pub mat_dt: String,
    pub int_type: String,
    pub int_rt: String,
    pub next_rep_dt: String,
    pub last_rep_dt: String,
    pub rep_freq: String,
    pub benchmark: String,
    pub cust_class: String,
    pub npa_class: String,
    pub ason: String,
    pub bank_name: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.bill_id,
            self.client_id,
            self.clients_name,
            self.lbm_gl,
            self.prod_cd,
            self.bal,
            self.curr,
            self.bal_lcy,
            self.open_dt,
            self.mat_dt,
            self.int_type,
            self.int_rt,
            self.next_rep_dt,
            self.last_rep_dt,
            self.rep_freq,
            self.benchmark,
            self.cust_class,
            self.npa_class,
            self.ason,
            self.bank_name
        )
    }
}
