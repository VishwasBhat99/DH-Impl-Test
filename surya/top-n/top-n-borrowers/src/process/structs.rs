#[derive(Debug, Default, Clone)]
pub struct UcicDet {
    pub ucic_id: String,
    pub ucic_name: String,
    pub cust_type: String,
}

#[derive(Debug, Default, Clone)]
pub struct CustVal {
    pub cust_name: String,
    pub cust_type: String,
    pub tot_amt_hcy: String,
    pub npa_class: String,
}
