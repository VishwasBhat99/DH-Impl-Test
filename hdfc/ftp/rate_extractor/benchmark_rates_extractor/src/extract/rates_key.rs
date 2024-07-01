use rbdate::NaiveDate;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct RatesKey {
    pub curve_id: i64,
    pub as_on: NaiveDate,
}

impl RatesKey {
    pub fn new(curve_id: i64, as_on: NaiveDate) -> RatesKey {
        RatesKey { curve_id, as_on }
    }
}

impl Display for RatesKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.curve_id, self.as_on)
    }
}
