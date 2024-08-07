use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub struct AggregateData {
    pub tot_prin_amt: f64,
    pub rt_prin_amt_weighted: f64,
}

impl AggregateData {
    pub fn new() -> AggregateData {
        AggregateData {
            tot_prin_amt: 0.0,
            rt_prin_amt_weighted: 0.0,
        }
    }

    pub fn add_data(&mut self, p_a: f64, r: f64) {
        self.tot_prin_amt += p_a;
        self.rt_prin_amt_weighted += p_a * r;
    }

    pub fn add_from_builder(&mut self, other: AggregateData) {
        self.tot_prin_amt += other.tot_prin_amt;
        self.rt_prin_amt_weighted += other.rt_prin_amt_weighted;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.tot_prin_amt *= multiplier;
        self.rt_prin_amt_weighted *= multiplier;
    }
}

impl Display for AggregateData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:.2}|{:.2}",
            self.tot_prin_amt,
            if self.tot_prin_amt == 0.0 {
                0.0
            } else {
                self.rt_prin_amt_weighted / self.tot_prin_amt
            }
        )
    }
}
