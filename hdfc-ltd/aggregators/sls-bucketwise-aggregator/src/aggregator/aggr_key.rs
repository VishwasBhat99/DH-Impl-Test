use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggrKey {
    pub as_on_date: String,
    pub country: String,
    pub currency: String,
    pub llg: String,
}

impl Display for AggrKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.as_on_date, self.country, self.currency, self.llg
        )
    }
}
