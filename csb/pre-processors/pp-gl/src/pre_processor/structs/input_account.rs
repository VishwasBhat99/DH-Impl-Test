#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub gl_cd: String,
    pub os_bal: String,
    pub ccy: String,
    pub os_bal_lcy: String,
    pub br_cd: String,
    pub dr: String,
    pub cr: String,
    pub as_on: String,
}

impl InputAccount {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|",
            self.gl_cd,
            self.os_bal,
            self.ccy,
            self.os_bal_lcy,
            self.br_cd,
            self.dr,
            self.cr,
            self.as_on,
        )
    }
}
