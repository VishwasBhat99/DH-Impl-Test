use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGVal {
    pub tot_amt: String,
    pub src_yield: String,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub acc_no: String,
    pub as_on: String,
    pub source: String,
    pub acc_open_dt: String,
    pub ccy: String,
    pub prod_code: String,
    pub scheme_id: String,
    pub mis_1: String,
    pub mis_2: String,
    pub mis_3: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub concat: String,
    pub npa_flag: String,
    pub division: String,
    pub alm_line: String,
    pub ia_line: String,
    pub org_tenor: String,
    pub alco_mapping: String,
    pub psl_non_psl: String,
    pub custom1: String,
    pub custom2: String,
}

impl LLGVal {
    pub fn new() -> LLGVal {
        LLGVal {
            tot_amt: "0.0".to_string(),
            src_yield: "0.0".to_string(),
        }
    }
}

impl LLGKey {
    pub fn new(
        acc_no: String,
        as_on: String,
        source: String,
        acc_open_dt: String,
        ccy: String,
        prod_code: String,
        scheme_id: String,
        mis_1: String,
        mis_2: String,
        mis_3: String,
        raw_bm: String,
        final_bm: String,
        concat: String,
        npa_flag: String,
        division: String,
        alm_line: String,
        ia_line: String,
        org_tenor: String,
        alco_mapping: String,
        psl_non_psl: String,
        custom1: String,
        custom2: String,
    ) -> LLGKey {
        LLGKey {
            acc_no,
            as_on,
            source,
            acc_open_dt,
            ccy,
            prod_code,
            scheme_id,
            mis_1,
            mis_2,
            mis_3,
            raw_bm,
            final_bm,
            concat,
            npa_flag,
            division,
            alm_line,
            ia_line,
            org_tenor,
            alco_mapping,
            psl_non_psl,
            custom1,
            custom2,
        }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.acc_no,
            self.as_on,
            self.source,
            self.acc_open_dt,
            self.ccy,
            self.prod_code,
            self.scheme_id,
            self.mis_1,
            self.mis_2,
            self.mis_3,
            self.raw_bm,
            self.final_bm,
            self.concat,
            self.npa_flag,
            self.division,
            self.alm_line,
            self.ia_line,
            self.org_tenor,
            self.alco_mapping,
            self.psl_non_psl,
            self.custom1,
            self.custom2,
        )
    }
}
