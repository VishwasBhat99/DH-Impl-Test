#[derive(Hash, Eq, PartialEq, Debug, Clone, Default)]
pub struct StamperKey {
    pub dim_id: String,
    pub dim_item_id: String,
    pub rlg_item_id: String,
}

impl StamperKey {
    pub fn new() -> Self {
        StamperKey {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StamperValues {
    pub avg_bal: f64,
    pub int_rate: f64,
    pub avg_int_rate: f64,
    pub avg_ftp_rate: f64,
    pub int_amt: f64,
    pub ftp_rate: f64,
    pub ftp_amt: f64,
    pub sum_prod_int_rt_avg_bal: f64,
    pub sum_prod_ftp_rt_ftp_amt: f64,
}

impl StamperValues {
    pub fn new() -> Self {
        StamperValues {
            ..Default::default()
        }
    }
}
