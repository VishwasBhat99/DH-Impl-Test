use super::agg_key::AGGKey;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct AccData {
    pub grp_key: AGGKey,
    pub data: Val,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Val {
    pub int_accrued: f64,
    pub int_accrued_rev: f64,
    pub penalty_amt: f64,
    pub premat_inc: f64,
    pub balance: f64,
    pub last_withdraw: f64,
    pub org_int_rt: f64,
    pub revised_rt: f64,
    pub pen_int_rt: f64,
}

impl Val {
    pub fn append_data(&mut self, new_data: Self) {
        self.balance += new_data.balance;
        self.org_int_rt = ((self.org_int_rt * self.balance / 100.0
            + new_data.org_int_rt * new_data.balance / 100.0)
            / (self.balance + new_data.balance))
            * 100.0;
        self.revised_rt = ((self.revised_rt * self.balance / 100.0
            + new_data.revised_rt * new_data.balance / 100.0)
            / (self.balance + new_data.balance))
            * 100.0;
        self.pen_int_rt = ((self.pen_int_rt * self.balance / 100.0
            + new_data.pen_int_rt * new_data.balance / 100.0)
            / (self.balance + new_data.balance))
            * 100.0;
        self.int_accrued += new_data.int_accrued;
        self.int_accrued_rev += new_data.int_accrued_rev;
        self.penalty_amt += new_data.penalty_amt;
        self.premat_inc += new_data.premat_inc;
        self.last_withdraw += new_data.last_withdraw;
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.int_accrued,
            self.int_accrued_rev,
            self.penalty_amt,
            self.premat_inc,
            self.balance,
            self.last_withdraw,
            self.org_int_rt,
            self.revised_rt,
            self.pen_int_rt,
        )
    }
}
