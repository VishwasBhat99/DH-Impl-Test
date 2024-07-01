use crate::configuration_parameters::{self, ConfigurationParameters};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CustDataKey {
    pub cust_id: String,
    pub ccy: String,
}
#[derive(Debug, Clone)]
pub struct CustDataValue {
    pub amt_hcy: f64,
    pub amt_ccy: f64,
    pub int_rate: f64,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TopNIntRateKey {
    pub cust_id: String,
    pub ccy: String,
}

#[derive(Debug, Clone)]
pub struct TopNIntRateValue {
    pub country_cd: String,
    pub as_on_dt: String,
    pub sa_amt_hcy: f64,
    pub sa_amt_ccy: f64,
    pub ca_amt_hcy: f64,
    pub ca_amt_ccy: f64,
    pub sa_int_rt: f64,
    pub ca_int_rt: f64,
    pub tdwd_int_rt: f64,
    pub tdnwd_int_rt: f64,
}

impl TopNIntRateValue {
    pub fn new() -> TopNIntRateValue {
        TopNIntRateValue {
            country_cd: "".to_string(),
            as_on_dt: "".to_string(),
            sa_amt_hcy: 0.0,
            sa_amt_ccy: 0.0,
            ca_amt_hcy: 0.0,
            ca_amt_ccy: 0.0,
            sa_int_rt: 0.0,
            ca_int_rt: 0.0,
            tdwd_int_rt: 0.0,
            tdnwd_int_rt: 0.0,
        }
    }
}
