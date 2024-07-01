use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct ExKey {
    pub from_curr: String,
    pub to_curr: String,
}

impl ExKey {
    pub fn new(from_currency: &str, to_currency: &str) -> ExKey {
        ExKey {
            from_curr: from_currency.to_string(),
            to_curr: to_currency.to_string(),
        }
    }
}

impl Display for ExKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "from_curr:{}, to_curr{}", self.from_curr, self.to_curr)
    }
}
