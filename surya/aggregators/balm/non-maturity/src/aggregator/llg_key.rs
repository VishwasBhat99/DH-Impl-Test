// TODO: Lib
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub currency: String,
    pub category: i32,
    pub cf_type: String,
}

// TODO: Can we get some performance back by making LLGKey `Copy` since we know
// size upfront, and we can avoid UTF-8?

impl LLGKey {
    pub fn new(currency: String, category: i32, cf_type: String) -> LLGKey {
        LLGKey {
            currency,
            category,
            cf_type,
        }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.currency, self.category, self.cf_type)
    }
}
