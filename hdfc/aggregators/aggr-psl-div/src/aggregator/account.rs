use super::agg_key::AGGKey;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct AccData {
    pub grp_key: AGGKey,
    pub aggr_data: AggrVal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggrVal {
    pub avg_bal: f64,
    pub eop_bal: f64,
}

impl AggrVal {
    pub fn append_data(&mut self, new_data: Self) {
        self.avg_bal += new_data.avg_bal;
        self.eop_bal += new_data.eop_bal;
    }
}

impl Display for AggrVal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}|{}",
            self.avg_bal,
            if self.avg_bal != 0.0 {
                self.eop_bal / self.avg_bal
            } else {
                0.0
            },
        )
    }
}
