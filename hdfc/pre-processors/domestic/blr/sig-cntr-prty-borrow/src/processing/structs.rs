#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub country_code: String,
    pub ason_date: String,
    pub currency_id: String,
    pub ucic_id: String,
    pub ucic_name: String,
    pub borr_bal_lcy: f64,
    pub borr_bal_fcy: f64,
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
            self.borr_bal_fcy,
            self.borr_bal_lcy,
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TotalBalance {
    pub tot_liab_amt: f64,
}

impl TotalBalance {
    pub fn new() -> Self {
        TotalBalance {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OutputLines {
    pub liab_op_line: String,
}

impl OutputLines {
    pub fn new() -> Self {
        OutputLines {
            ..Default::default()
        }
    }
}
