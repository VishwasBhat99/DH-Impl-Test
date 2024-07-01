use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggrKey {
    pub llg_id: String,
    pub as_on_date: String,
    pub ccy: String,
}

impl Display for AggrKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.llg_id, self.as_on_date, self.ccy)
    }
}
