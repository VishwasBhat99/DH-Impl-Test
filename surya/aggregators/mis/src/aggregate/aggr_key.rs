use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AggrKey {
    pub dim1: String,
    pub dim2: String,
    pub dim3: String,
    pub ccy: String,
}

impl Display for AggrKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}|{}", self.dim1, self.dim2, self.dim3, self.ccy)
    }
}
