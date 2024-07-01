use chrono::Datelike;
use rbdate::NaiveDate;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AggrKey {
    pub coa: String,
    pub mis1_code: String,
    pub div_desc: String,
}

impl Display for AggrKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.coa, self.mis1_code, self.div_desc,)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggrVal {
    pub eop: f64,
    pub avg_bal: f64,
    pub tpr: f64,
    pub specprov_till_prev_mon: f64,
    pub specprov_till_ason_mon: f64,
    pub specprov_till_prev_year: f64,
    pub writeoff_till_prev_mon: f64,
    pub writeoff_till_ason_mon: f64,
    pub writeoff_till_prev_year: f64,
    pub specytdprov_till_prev_mon: f64,
    pub specytdprov_till_ason_mon: f64,
    pub specytdprov_till_prev_year: f64,
    pub float_avg_bal: f64,
    pub float_amount: f64,
}

impl AggrVal {
    pub fn append_data(&mut self, new_data: Self, as_on_date: NaiveDate) {
        self.eop += new_data.eop;
        self.tpr = ((self.tpr * self.avg_bal / 100.0 + new_data.tpr * new_data.avg_bal / 100.0)
            / (self.avg_bal + new_data.avg_bal))
            * 100.0;
        self.avg_bal += new_data.avg_bal;
        self.specprov_till_prev_mon += new_data.specprov_till_prev_mon;
        self.specprov_till_ason_mon += new_data.specprov_till_ason_mon;
        self.specprov_till_prev_year += new_data.specprov_till_prev_year;
        self.writeoff_till_prev_mon += new_data.writeoff_till_prev_mon;
        self.writeoff_till_ason_mon += new_data.writeoff_till_ason_mon;
        self.writeoff_till_prev_year += new_data.writeoff_till_prev_year;
        self.specytdprov_till_prev_mon = self.specprov_till_prev_mon - self.specprov_till_prev_year
            + self.writeoff_till_prev_mon;
        self.specytdprov_till_ason_mon = self.specprov_till_ason_mon - self.specprov_till_prev_year
            + self.writeoff_till_ason_mon;
        self.specytdprov_till_prev_year =
            self.specytdprov_till_ason_mon - self.specytdprov_till_prev_mon;
        self.float_avg_bal = self.specprov_till_ason_mon;
        self.float_amount =
            (self.float_avg_bal * self.tpr * (rbdate::get_days_from_month(as_on_date) as f64))
                / (100.00
                    * if is_leap_year(as_on_date.year()) {
                        366.00
                    } else {
                        365.00
                    });
    }
}

impl Display for AggrVal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.eop,
            self.avg_bal,
            self.tpr,
            self.specprov_till_prev_mon,
            self.specprov_till_ason_mon,
            self.specprov_till_prev_year,
            self.writeoff_till_prev_mon,
            self.writeoff_till_ason_mon,
            self.writeoff_till_prev_year,
            self.specytdprov_till_prev_mon,
            self.specytdprov_till_ason_mon,
            self.specytdprov_till_prev_year,
            self.float_avg_bal,
            self.float_amount,
        )
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}
