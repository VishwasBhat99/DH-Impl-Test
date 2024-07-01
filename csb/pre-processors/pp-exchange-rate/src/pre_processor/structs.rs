#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CurrencyConverter {
    pub source: String,
    pub target: String,
    pub ex_rt: f64,
}

impl CurrencyConverter {
    pub fn swap(&self) -> CurrencyConverter {
        CurrencyConverter {
            source: self.target.to_string(),
            target: self.source.to_string(),
            // ex_rt will never be equal to 0 as such cases are skipped while reading input file
            ex_rt: 1.0 / self.ex_rt,
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
}
