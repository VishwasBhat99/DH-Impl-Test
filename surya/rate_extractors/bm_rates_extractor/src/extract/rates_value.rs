use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RatesValue {
    pub period: i64,
    pub uom: String,
    pub rate: f64,
}

impl RatesValue {
    pub fn new(period: i64, uom: String, rate: f64) -> RatesValue {
        RatesValue { period, uom, rate }
    }
}

impl Display for RatesValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.period, self.uom, self.rate)
    }
}
