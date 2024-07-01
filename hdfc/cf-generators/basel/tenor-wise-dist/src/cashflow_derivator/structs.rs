use rbdate::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct Key {
    pub acc_no: String,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.acc_no,)
    }
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub custid: i64,
    pub classid: i64,
    pub curr: String,
    pub mat_date: NaiveDate,
    pub tot_amt: f64,
    pub tot_nwd_amt: f64,
}

impl<'a> Display for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}\n",
            self.custid, self.classid, self.curr, self.mat_date, self.tot_amt, self.tot_nwd_amt,
        )
    }
}
