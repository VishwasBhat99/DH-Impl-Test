#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyConverter {
    pub sr_no: String,
    pub source: String,
    pub target: String,
    pub typ: String,
    pub ex_rt: f64,
    pub dt: String,
}

impl CurrencyConverter {
    pub fn swap(&self) -> CurrencyConverter {
        CurrencyConverter {
            sr_no: self.sr_no.to_string(),
            source: self.target.to_string(),
            target: self.source.to_string(),
            typ: self.typ.to_string(),
            // ex_rt will never be equal to 0 as such cases are skipped while reading input file
            ex_rt: 1.0 / self.ex_rt,
            dt: self.dt.to_string(),
        }
    }

    pub fn print(&self) -> String {
        format!("{}|{}|{}", self.source, self.target, self.ex_rt,)
    }

    pub fn print_rev_order(&self) -> String {
        format!(
            "{}|{}|{}",
            self.target,
            self.source,
            // ex_rt will never be equal to 0 as such cases are skipped while reading input file
            1.0 / self.ex_rt,
        )
    }

    pub fn print_lcy_equi_rt(&self, lcy_rt: f64) -> String {
        format!(
            "{}|{}|{}",
            self.target,
            "INR",
            // ex_rt will never be equal to 0 as such cases are skipped while reading input file
            (1.0 / self.ex_rt) * lcy_rt,
        )
    }
}
