use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct NormalizeData {
    pub acc_no: String,
    pub date: NaiveDate,
    pub tot_prin_amt: f64,
    pub int_rt: f64,
}

impl NormalizeData {
    pub fn new() -> NormalizeData {
        NormalizeData {
            acc_no: String::new(),
            date: NaiveDate::from_ymd(2099, 12, 31),
            tot_prin_amt: DEFAULT_FLOAT,
            int_rt: DEFAULT_FLOAT,
        }
    }

    pub fn insert(&mut self, acc_no: String, dt: NaiveDate, prin_amt: f64, int_rt: f64) {
        self.acc_no = acc_no;
        self.date = dt;
        self.tot_prin_amt = prin_amt;
        self.int_rt = int_rt;
    }
}

impl Display for NormalizeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}\n",
            self.acc_no,
            self.date.format("%d-%m-%Y"),
            self.tot_prin_amt,
            self.int_rt
        )
    }
}
