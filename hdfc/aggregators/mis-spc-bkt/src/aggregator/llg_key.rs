use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

// TODO: p2 fields needs to be removed as, report id is being used in place of it
#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub scheme_id: String,
    pub rate_flag: String,
    pub rep_index: String,
    pub alm_line: String,
    pub al_line: String,
    pub p1: String,
    pub p2: String,
    pub p3: String,
}

impl LLGKey {
    pub fn new(
        scheme_id: String,
        rate_flag: String,
        rep_index: String,
        alm_line: String,
        al_line: String,
        p1: String,
        p2: String,
        p3: String,
    ) -> LLGKey {
        LLGKey {
            scheme_id,
            rate_flag,
            rep_index,
            alm_line,
            al_line,
            p1,
            p2,
            p3,
        }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|",
            self.scheme_id,
            self.rate_flag,
            self.rep_index,
            self.alm_line,
            self.al_line,
            self.p1,
            self.p2,
            self.p3,
        )
    }
}
