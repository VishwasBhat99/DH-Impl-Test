use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub struct AggrData {
    pub ca_amt: f64,
    pub sa_amt: f64,
    pub td_amt: f64,
    pub depth: i64,
    pub order: i64,
}

impl AggrData {
    pub fn add_data(&mut self, new_data: AggrData) {
        self.ca_amt += new_data.ca_amt;
        self.sa_amt += new_data.sa_amt;
        self.td_amt += new_data.td_amt;
    }
}

impl Display for AggrData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:.2}|{:.2}|{:.2}|{}|{}",
            self.ca_amt, self.sa_amt, self.td_amt, self.depth, self.order
        )
    }
}
