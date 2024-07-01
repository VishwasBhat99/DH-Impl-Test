use std::fmt;
#[derive(Debug, Default, Clone)]
pub struct UcicDet {
    pub ucic_id: String,
    pub ucic_name: String,
    pub cust_type: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CustId {
    pub cust_id: String,
    pub ccy: String,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CustVal {
    pub ca_amt_ccy: String,
    pub sa_amt_ccy: String,
    pub td_wd_amt_ccy: String,
    pub td_nwd_amt_ccy: String,
    pub rd_ccy: String,
    pub ca_amt_hcy: String,
    pub sa_amt_hcy: String,
    pub td_wd_amt_hcy: String,
    pub td_nwd_amt_hcy: String,
    pub ca_int_rt: String,
    pub sa_int_rt: String,
    pub td_wd_int_rt: String,
    pub td_nwd_int_rt: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Exrt {
    pub from_ccy: String,
    pub to_ccy: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GroupVal {
    pub cust_type: String,
    pub cust_name: String,
    pub tot_hcy_amt: String,
}

impl UcicDet {
    pub fn default() -> UcicDet {
        UcicDet {
            ucic_id: "123".to_string(),
            ucic_name: "NONE".to_string(),
            cust_type: "NONE".to_string(),
        }
    }
}
