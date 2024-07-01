use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub source_code: i32,
}

impl LLGKey {
    pub fn new(source_code: i32) -> LLGKey {
        LLGKey { source_code }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.source_code)
    }
}
