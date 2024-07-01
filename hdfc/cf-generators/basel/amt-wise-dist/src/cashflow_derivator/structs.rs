use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct KeyID {
    pub custid: i64,
    pub classid: i64,
}

impl Display for KeyID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.custid, self.classid,)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct AccsData {
    pub curr: String,
    pub tot_amt: f64,
    pub tot_nwd_amt: f64,
    pub tot_accs: f64,
    pub tot_nwd_accs: f64,
    pub ca_accs: f64,
    pub sa_accs: f64,
    pub td_accs: f64,
    pub rd_accs: f64,
    pub ca_nwd_accs_op: f64,
    pub ca_nwd_accs_nonop: f64,
    pub sa_nwd_accs: f64,
    pub td_nwd_accs: f64,
    pub rd_nwd_accs: f64,
    pub td_amt: f64,
    pub rd_amt: f64,
    pub sa_amt: f64,
    pub ca_amt: f64,
}

impl Display for AccsData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            self.curr,
            self.tot_amt,
            self.tot_nwd_amt,
            self.tot_accs,
            self.tot_nwd_accs,
            self.ca_accs,
            self.sa_accs,
            self.td_accs,
            self.rd_accs,
            self.ca_nwd_accs_op,
            self.ca_nwd_accs_nonop,
            self.sa_nwd_accs,
            self.td_nwd_accs,
            self.rd_nwd_accs,
            self.td_amt,
            self.rd_amt,
            self.sa_amt,
            self.ca_amt
        )
    }
}
