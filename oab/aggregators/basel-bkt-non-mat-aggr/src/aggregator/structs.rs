use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
#[derive(Debug, Copy, Clone)]
pub struct AggregateData {
    pub total_amt: f64,
}

impl AggregateData {
    pub fn new() -> AggregateData {
        AggregateData { total_amt: 0.0 }
    }

    pub fn add_from_builder(&mut self, other: AggregateData) {
        self.total_amt += other.total_amt;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.total_amt *= multiplier;
    }

    pub fn add_amount(&mut self, amt: f64) {
        self.total_amt += amt;
    }
}

#[derive(Debug)]
pub struct CashflowAggregated {
    pub amount: f64,
}

impl Display for AggregateData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.2}", self.total_amt)
    }
}
