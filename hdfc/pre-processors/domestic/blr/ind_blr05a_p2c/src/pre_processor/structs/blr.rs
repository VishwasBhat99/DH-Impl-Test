#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BLRInput {
    pub srl_no: String,
    pub particulars_of_breach: String,
    pub breach_dt: String,
    pub breach_amt: String,
    pub penalty_amt: String,
    pub bank_action: String,
}

impl BLRInput {
    pub fn print(&self, as_on_date: String) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}\n",
            as_on_date,
            self.srl_no,
            self.particulars_of_breach,
            self.breach_dt,
            self.breach_amt,
            self.penalty_amt,
            self.bank_action,
            self.srl_no,
        )
    }
}
