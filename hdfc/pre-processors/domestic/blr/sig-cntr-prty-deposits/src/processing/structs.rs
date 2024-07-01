#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub country_code: String,
    pub ason_date: String,
    pub currency_id: String,
    pub ucic_id: String,
    pub ucic_name: String,
    pub sa_bal_lcy: f64,
    pub sa_bal_hcy: f64,
    pub ca_bal_lcy: f64,
    pub ca_bal_hcy: f64,
    pub td_bal_lcy: f64,
    pub td_bal_hcy: f64,
    pub tot_bal_lcy: f64,
    pub tot_bal_hcy: f64,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}\n",
            self.country_code,
            self.ason_date,
            self.currency_id,
            self.ucic_id,
            self.ucic_name,
            self.tot_bal_lcy,
            self.tot_bal_hcy
        )
    }
}
