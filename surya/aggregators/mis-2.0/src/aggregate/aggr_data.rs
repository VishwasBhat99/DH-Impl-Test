#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub tot_prin_amt: f64,
    pub tot_prin_amt_lcy: f64,
    pub rt_prin_amt_weighted: f64,
    pub min_amount_ccy: f64,
    pub min_amount_hcy: f64,
    pub max_amount_ccy: f64,
    pub max_amount_hcy: f64,
    pub int_accured: f64,
    pub min_int_rate: f64,
    pub max_int_rate: f64,
    pub avg_days_contract_mat_sum: f64,
    pub avg_days_residual_mat_sum: f64,
    pub no_of_depositers: i64,
    pub total_bal: f64,
}

impl Data {
    pub fn append_data(&mut self, new_data: Data) {
        self.tot_prin_amt += new_data.tot_prin_amt;
        self.tot_prin_amt_lcy += new_data.tot_prin_amt_lcy;
        self.rt_prin_amt_weighted += new_data.rt_prin_amt_weighted;
        if self.min_amount_ccy > new_data.min_amount_ccy {
            self.min_amount_ccy = new_data.min_amount_ccy;
        }
        if self.min_amount_hcy > new_data.min_amount_hcy {
            self.min_amount_hcy = new_data.min_amount_hcy;
        }
        if self.max_amount_ccy < new_data.max_amount_ccy {
            self.max_amount_ccy = new_data.max_amount_ccy;
        }
        if self.max_amount_hcy < new_data.max_amount_hcy {
            self.max_amount_hcy = new_data.max_amount_hcy;
        }
        if self.min_int_rate > new_data.min_int_rate {
            self.min_int_rate = new_data.min_int_rate;
        }
        if self.max_int_rate < new_data.max_int_rate {
            self.max_int_rate = new_data.max_int_rate;
        }
        self.avg_days_contract_mat_sum += new_data.avg_days_contract_mat_sum;
        self.avg_days_residual_mat_sum += new_data.avg_days_residual_mat_sum;
        self.no_of_depositers += new_data.no_of_depositers;
        self.total_bal += new_data.total_bal;
    }
}
