use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub llg_cd: i32,
    pub ccy: String,
    pub tenor: i64,
}

impl LLGKey {
    pub fn new(llg_cd: i32, ccy: String, tenor: i64) -> LLGKey {
        LLGKey { llg_cd, ccy, tenor }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.llg_cd, self.ccy, self.tenor,)
    }
}
