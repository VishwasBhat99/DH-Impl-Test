use super::llg_key::LLGKey;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct AccData {
    pub grp_key: LLGKey,
    pub npa_data: NpaVal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NpaVal {
    pub amt_as_per_npa: f64,
    pub amt_as_per_src: f64,
    pub yield_rate: f64,
    pub unser_int_sus: String,
    pub unser_oth_inc: String,
    pub npa_date: String,
}

impl NpaVal {
    pub fn append_data(&mut self, new_data: Self) {
        self.amt_as_per_src += new_data.amt_as_per_src;
        self.yield_rate += new_data.yield_rate;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.amt_as_per_src *= multiplier;
        self.yield_rate *= multiplier;
    }
}

impl Display for NpaVal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|\n",
            self.amt_as_per_npa,
            self.amt_as_per_src,
            if self.amt_as_per_src != 0.0 {
                self.yield_rate / self.amt_as_per_src
            } else {
                0.0
            },
            self.unser_int_sus,
            self.unser_oth_inc,
            self.npa_date,
        )
    }
}
