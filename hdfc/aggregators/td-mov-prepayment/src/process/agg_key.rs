use rbdate::*;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct AGGKey {
    as_on: NaiveDate,
    acc_open_dt: NaiveDate,
    val_dt: NaiveDate,
    withdraw_dt: NaiveDate,
    mat_dt: NaiveDate,
    ccy: String,
    prod_code: String,
    mis1: String,
    concat: String,
    div: String,
    alm_line: String,
    ia_line: String,
    org_tenor: String,
    pp_tenor: String,
    ia_tenor: String,
    cat_val: String,
    lcr_val: String,
    wd_nwd_val: String,
}

impl<'a> AGGKey {
    pub fn new(
        as_on: NaiveDate,
        acc_open_dt: NaiveDate,
        val_dt: NaiveDate,
        withdraw_dt: NaiveDate,
        mat_dt: NaiveDate,
        ccy: String,
        prod_code: String,
        mis1: String,
        concat: String,
        div: String,
        alm_line: String,
        ia_line: String,
        org_tenor: String,
        pp_tenor: String,
        ia_tenor: String,
        cat_val: String,
        lcr_val: String,
        wd_nwd_val: String,
    ) -> AGGKey {
        AGGKey {
            as_on,
            acc_open_dt,
            val_dt,
            withdraw_dt,
            mat_dt,
            ccy,
            prod_code,
            mis1,
            concat,
            div,
            alm_line,
            ia_line,
            org_tenor,
            pp_tenor,
            ia_tenor,
            cat_val,
            lcr_val,
            wd_nwd_val,
        }
    }
}

impl<'a> Display for AGGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.as_on.format("%d-%m-%Y"),
            self.acc_open_dt.format("%d-%m-%Y"),
            self.val_dt.format("%d-%m-%Y"),
            self.withdraw_dt.format("%d-%m-%Y"),
            self.mat_dt.format("%d-%m-%Y"),
            self.ccy,
            self.prod_code,
            self.mis1,
            self.concat,
            self.div,
            self.alm_line,
            self.ia_line,
            self.org_tenor,
            self.pp_tenor,
            self.ia_tenor,
            self.cat_val,
            self.lcr_val,
            self.wd_nwd_val
        )
    }
}
