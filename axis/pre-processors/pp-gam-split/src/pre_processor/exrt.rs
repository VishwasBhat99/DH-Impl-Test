use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct ExrtKey {
    pub from_currency: String,
    pub to_currency: String,
}

impl ExrtKey {
    pub fn new(from_currency: String, to_currency: String) -> ExrtKey {
        ExrtKey {
            from_currency,
            to_currency,
        }
    }
}

impl Display for ExrtKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.from_currency, self.to_currency)
    }
}
