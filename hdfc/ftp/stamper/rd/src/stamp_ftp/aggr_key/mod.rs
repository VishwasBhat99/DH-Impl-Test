use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
pub mod read_aggr_file;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub cust_id: i64,
    pub start_dt: i64,
    pub mat_dt: i64,
    pub ccy: String,
}

impl Customer {
    pub fn new(cust_id: i64, start_dt: i64, mat_dt: i64, ccy: String) -> Customer {
        Customer {
            cust_id,
            start_dt,
            mat_dt,
            ccy,
        }
    }
}

impl Display for Customer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}",
            self.cust_id, self.start_dt, self.mat_dt, self.ccy
        )
    }
}
