use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub currency: String,
    pub llg_id: i32,
}

impl LLGKey {
    pub fn new(currency: String, llg_id: i32) -> LLGKey {
        LLGKey { currency, llg_id }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.currency, self.llg_id)
    }
}
