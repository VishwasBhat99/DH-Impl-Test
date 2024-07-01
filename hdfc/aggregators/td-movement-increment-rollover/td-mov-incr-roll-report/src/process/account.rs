use super::llg_key::LLGKey;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct AccData {
    pub grp_key: LLGKey,
    pub data: Val,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Val {
    pub rate: f64,
    pub rate_var: f64,
    pub rate_var2: f64,
    pub amt_initl_dep: f64,
    pub yld: f64,
}

impl Val {
    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.amt_initl_dep *= multiplier;
        self.yld *= multiplier;
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}\n",
            self.rate, self.rate_var, self.rate_var2, self.amt_initl_dep, self.yld,
        )
    }
}
