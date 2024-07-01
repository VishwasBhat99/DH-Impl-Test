use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub currency: String,
    pub category: i32,
}

impl LLGKey {
    pub fn new(currency: String, category: i32) -> LLGKey {
        LLGKey { currency, category }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.currency, self.category)
    }
}
