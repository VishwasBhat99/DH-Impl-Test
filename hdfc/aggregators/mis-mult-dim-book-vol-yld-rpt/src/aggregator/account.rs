use super::llg_key::LLGKey;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct AccData {
    pub grp_key: LLGKey,
    pub aggr_data: AggrVal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggrVal {
    pub tot_amt: f64,
    pub wt_yield_rate: f64,
}

impl AggrVal {
    pub fn append_data(&mut self, new_data: Self) {
        self.tot_amt += new_data.tot_amt;
        self.wt_yield_rate += new_data.wt_yield_rate;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64) {
        self.tot_amt *= multiplier;
        self.wt_yield_rate *= multiplier;
    }
}

impl Display for AggrVal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}\n",
            self.tot_amt,
            if self.tot_amt != 0.0 {
                self.wt_yield_rate / self.tot_amt
            } else {
                0.0
            }
        )
    }
}
