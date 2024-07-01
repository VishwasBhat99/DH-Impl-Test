use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct ReconKey {
    pub currency: String,
    pub gl_type: String,
    pub gl_code: String,
}

impl ReconKey {
    pub fn new(currency: String, gl_type: String, gl_code: String) -> ReconKey {
        ReconKey {
            currency,
            gl_type,
            gl_code,
        }
    }
}

impl Display for ReconKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.currency, self.gl_type, self.gl_code)
    }
}
