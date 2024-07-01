#[derive(Debug, Clone)]
pub struct AggregateData {
    pub calc_flag: String,
    pub calculated_avg_bal: f64,
    pub input_avg_bal: f64,
    pub int_rate: f64,
    pub days: f64,
    pub acr_int_amt_ccy: f64,
    pub acr_int_amt_hcy: f64,
}

impl AggregateData {
    pub fn add(
        &mut self,
        calc_flag: String,
        bal: f64,
        int_rt: f64,
        acr_int_amt_ccy: f64,
        acr_int_amt_hcy: f64,
    ) {
        self.calc_flag = calc_flag;
        self.calculated_avg_bal += bal;
        self.int_rate += int_rt;
        self.acr_int_amt_ccy = acr_int_amt_ccy;
        self.acr_int_amt_hcy = acr_int_amt_hcy;
    }
}

#[derive(Debug, Clone)]
pub struct InputBalances {
    pub input_avg_bal: f64,
    pub days: f64,
    pub acr_int_amt_ccy: f64,
    pub acr_int_amt_hcy: f64,
}
