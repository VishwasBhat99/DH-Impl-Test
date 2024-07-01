#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BLRInput {
    pub srl_no: String,
    pub name_of_the_branch: String,
    pub country: String,
    pub currency: String,
    pub particulars_of_breach: String,
    pub date_of_breach: String,
    pub amount_of_breach: String,
    pub amount_of_penalty: String,
    pub action: String,
}

impl BLRInput {
    pub fn print(&self, as_on_date: String) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            as_on_date,
            self.name_of_the_branch,
            self.country,
            self.currency,
            self.particulars_of_breach,
            self.particulars_of_breach,
            self.date_of_breach,
            self.amount_of_breach,
            self.amount_of_penalty,
            self.action,
            self.srl_no,
        )
    }
}
