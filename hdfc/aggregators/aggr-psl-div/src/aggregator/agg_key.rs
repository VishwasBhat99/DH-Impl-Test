use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AGGKey {
    pub as_on: String,
    pub source: String,
    pub psl_code: String,
    pub division_code: String,
}

impl<'a> AGGKey {
    pub fn new(as_on: String, source: String, psl_code: String, division_code: String) -> AGGKey {
        AGGKey {
            as_on,
            source,
            psl_code,
            division_code,
        }
    }
}

impl<'a> Display for AGGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.as_on, self.source, self.psl_code, self.division_code
        )
    }
}
