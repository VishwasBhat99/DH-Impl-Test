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
    pub bal_lcy: f64,
    pub int_comp: f64,
    pub rate: f64,
    pub rate_var: f64,
    pub rate_var2: f64,
    pub amt: f64,
    pub yld: f64,
}

impl Val {
    pub fn append_data(&mut self, new_data: Self) {
        self.amt += new_data.amt;
        self.int_comp += new_data.int_comp;
        self.rate = ((self.rate * self.bal_lcy / 100.0 + new_data.rate * new_data.bal_lcy / 100.0)
            / (self.bal_lcy + new_data.bal_lcy))
            * 100.0;
        self.rate_var = ((self.rate_var * self.bal_lcy / 100.0
            + new_data.rate_var * new_data.bal_lcy / 100.0)
            / (self.bal_lcy + new_data.bal_lcy))
            * 100.0;
        self.rate_var2 = ((self.rate_var2 * self.bal_lcy / 100.0
            + new_data.rate_var2 * new_data.bal_lcy / 100.0)
            / (self.bal_lcy + new_data.bal_lcy))
            * 100.0;
        self.yld = self.rate + self.rate_var + self.rate_var2;
        self.bal_lcy += new_data.bal_lcy;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.amt *= multiplier;
        self.yld *= multiplier;
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|\n",
            self.bal_lcy,
            self.int_comp,
            self.rate,
            self.rate_var,
            self.rate_var2,
            self.amt,
            self.yld,
        )
    }
}
