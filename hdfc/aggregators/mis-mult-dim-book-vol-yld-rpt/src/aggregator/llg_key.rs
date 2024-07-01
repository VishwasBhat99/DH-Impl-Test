use rbdate::*;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub as_on: NaiveDate,
    pub source: String,
    pub ccy: String,
    pub prod_code: String,
    pub scheme_id: String,
    pub mis1: String,
    pub mis2: String,
    pub mis3: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub concat: String,
    pub npa_flag: String,
    pub div: String,
    pub alm_line: String,
    pub ia_line: String,
    pub org_tenor: String,
    pub alco: String,
    pub psl: String,
    pub rate_bucket: String,
}

impl<'a> LLGKey {
    pub fn new(
        as_on: NaiveDate,
        source: String,
        ccy: String,
        prod_code: String,
        scheme_id: String,
        mis1: String,
        mis2: String,
        mis3: String,
        raw_bm: String,
        final_bm: String,
        concat: String,
        npa_flag: String,
        div: String,
        alm_line: String,
        ia_line: String,
        org_tenor: String,
        alco: String,
        psl: String,
        rate_bucket: String,
    ) -> LLGKey {
        LLGKey {
            as_on,
            source,
            ccy,
            prod_code,
            scheme_id,
            mis1,
            mis2,
            mis3,
            raw_bm,
            final_bm,
            concat,
            npa_flag,
            div,
            alm_line,
            ia_line,
            org_tenor,
            alco,
            psl,
            rate_bucket,
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
            self.scheme_id,
            self.mis1,
            self.mis2,
            self.mis3,
            self.raw_bm,
            self.final_bm,
            self.concat,
            self.npa_flag,
            self.div,
            self.alm_line,
            self.ia_line,
            self.org_tenor,
            self.alco,
            self.psl,
            self.rate_bucket,
        )
    }
}
