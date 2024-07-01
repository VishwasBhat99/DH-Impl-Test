#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AggrKey {
    pub dim_item_id: String,
    pub rlg_item_id: String,
    pub aorl: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggrValues {
    pub average_balance: f64,
    pub interest_amount: f64,
    pub ftp_amount: f64,
    pub sum_prod_int_rt_bal_amt: f64,
    pub sum_prod_ftp_rt_bal_amt: f64,
}

impl AggrValues {
    pub fn new() -> Self {
        AggrValues {
            average_balance: 0.0,
            interest_amount: 0.0,
            ftp_amount: 0.0,
            sum_prod_int_rt_bal_amt: 0.0,
            sum_prod_ftp_rt_bal_amt: 0.0,
        }
    }
}
