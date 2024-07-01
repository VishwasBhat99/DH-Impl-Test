use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct LLGKey {
    pub as_on: String,
    pub source: String,
    pub report_id: String,
    pub bkt_scheme_id: String,
    pub ccy: String,
    pub dim_1: String,
    pub dim_2: String,
    pub dim_3: String,
    pub dim_4: String,
    pub dim_5: String,
    pub dim_6: String,
    pub dim_7: String,
    pub dim_8: String,
    pub dim_9: String,
    pub dim_10: String,
    pub dim_11: String,
    pub dim_12: String,
    pub dim_13: String,
    pub dim_14: String,
    pub dim_15: String,
    pub bkt_scheme: String,
}

impl LLGKey {
    pub fn new(
        as_on: String,
        source: String,
        report_id: String,
        bkt_scheme_id: String,
        ccy: String,
        dim_1: String,
        dim_2: String,
        dim_3: String,
        dim_4: String,
        dim_5: String,
        dim_6: String,
        dim_7: String,
        dim_8: String,
        dim_9: String,
        dim_10: String,
        dim_11: String,
        dim_12: String,
        dim_13: String,
        dim_14: String,
        dim_15: String,
        bkt_scheme: String,
    ) -> LLGKey {
        LLGKey {
            as_on,
            source,
            report_id,
            bkt_scheme_id,
            ccy,
            dim_1,
            dim_2,
            dim_3,
            dim_4,
            dim_5,
            dim_6,
            dim_7,
            dim_8,
            dim_9,
            dim_10,
            dim_11,
            dim_12,
            dim_13,
            dim_14,
            dim_15,
            bkt_scheme,
        }
    }
}

impl Display for LLGKey {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            self.as_on,
            self.source,
            self.report_id,
            self.bkt_scheme_id,
            self.ccy,
            self.dim_1,
            self.dim_2,
            self.dim_3,
            self.dim_4,
            self.dim_5,
            self.dim_6,
            self.dim_7,
            self.dim_8,
            self.dim_9,
            self.dim_10,
            self.dim_11,
            self.dim_12,
            self.dim_13,
            self.dim_14,
            self.dim_15,
            self.bkt_scheme
        )
    }
}
