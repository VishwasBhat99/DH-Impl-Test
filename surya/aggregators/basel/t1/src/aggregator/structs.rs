use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub struct AggregateData {
    pub tot_prin_amt: f64,
    pub tot_prin_amt_ccy: f64,
}

impl AggregateData {
    pub fn new() -> AggregateData {
        AggregateData {
            tot_prin_amt: 0.0,
            tot_prin_amt_ccy: 0.0,
        }
    }

    pub fn add_data(&mut self, p_a: f64, tot: f64) {
        self.tot_prin_amt += p_a;
        self.tot_prin_amt_ccy += tot;
    }

    pub fn add_from_builder(&mut self, other: AggregateData) {
        self.tot_prin_amt += other.tot_prin_amt;
        self.tot_prin_amt_ccy += other.tot_prin_amt_ccy;
    }
}
impl Display for AggregateData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.2}", self.tot_prin_amt)
    }
}
