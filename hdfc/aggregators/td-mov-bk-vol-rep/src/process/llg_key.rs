use rbdate::*;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    as_on: NaiveDate,
    source: String,
    ccy: String,
    prod_code: String,
    mis1: String,
    gl_liab: String,
    gl_int_comp: String,
    concat: String,
    div: String,
    alm_line: String,
    ia_line: String,
    alco: String,
    org_tenor: String,
    res_tenor: String,
    ia_tenor: String,
    bucket_val: String,
    cat_val: String,
    lcr_val: String,
    wd_nwd_val: String,
}

impl<'a> LLGKey {
    pub fn new(
        as_on: NaiveDate,
        source: String,
        ccy: String,
        prod_code: String,
        mis1: String,
        gl_liab: String,
        gl_int_comp: String,
        concat: String,
        div: String,
        alm_line: String,
        ia_line: String,
        alco: String,
        org_tenor: String,
        res_tenor: String,
        ia_tenor: String,
        bucket_val: String,
        cat_val: String,
        lcr_val: String,
        wd_nwd_val: String,
    ) -> LLGKey {
        LLGKey {
            as_on,
            source,
            ccy,
            prod_code,
            mis1,
            gl_liab,
            gl_int_comp,
            concat,
            div,
            alm_line,
            ia_line,
            alco,
            org_tenor,
            res_tenor,
            ia_tenor,
            bucket_val,
            cat_val,
            lcr_val,
            wd_nwd_val,
        }
    }
}

impl<'a> Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.as_on.format("%d-%m-%Y"),
            self.source,
            self.ccy,
            self.prod_code,
            self.mis1,
            self.gl_liab,
            self.gl_int_comp,
            self.concat,
            self.div,
            self.alm_line,
            self.ia_line,
            self.alco,
            self.org_tenor,
            self.res_tenor,
            self.ia_tenor,
            self.bucket_val,
            self.cat_val,
            self.lcr_val,
            self.wd_nwd_val,
        )
    }
}
