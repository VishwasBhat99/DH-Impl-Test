use super::{input_record::*, product_description::*};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct AggregatedKey {
    pub country: String,
    pub as_on: String,
    pub ccy: String,
    pub prod_cd: String,
    pub prod_name: String,
}

impl AggregatedKey {
    pub fn new(input_fields: &InputRecord, prod_desc: &ProdDescOutput) -> Self {
        AggregatedKey {
            country: input_fields.country.to_string(),
            as_on: input_fields.as_on.to_string(),
            ccy: input_fields.ccy.to_string(),
            prod_cd: prod_desc.prod_cd.to_string(),
            prod_name: prod_desc.prod_name.to_string(),
        }
    }
}

impl Display for AggregatedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}",
            self.country, self.as_on, self.ccy, self.prod_cd, self.prod_name,
        )
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AggregatedValue {
    pub ttl_lcy_amt: f64,
    pub ttl_fcy_amt: f64,
}

impl AggregatedValue {
    pub fn new(input_fields: &InputRecord) -> Self {
        AggregatedValue {
            ttl_lcy_amt: input_fields.lcy_amt,
            ttl_fcy_amt: input_fields.fcy_amt,
        }
    }
    pub fn add(&mut self, aggr_value: AggregatedValue) {
        self.ttl_lcy_amt += aggr_value.ttl_lcy_amt;
        self.ttl_fcy_amt += aggr_value.ttl_fcy_amt;
    }
}

impl Display for AggregatedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}|{}\n", self.ttl_fcy_amt, self.ttl_lcy_amt,)
    }
}
