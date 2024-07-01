#[derive(Debug, Copy, Clone)]

pub struct Data {
    pub tot_amt: f64,
    pub weighted_int_rate_sum: f64,
    pub count: i64,
    pub max_amt: f64,
    pub min_amt: f64,
    pub max_int_rate: f64,
    pub min_int_rate: f64,
    pub int_rate_sum: f64,
}

impl Data {
    pub fn append_data(&mut self, new_data: Data) {
        self.tot_amt += new_data.tot_amt;
        self.weighted_int_rate_sum += new_data.weighted_int_rate_sum;
        self.count += new_data.count;
        if self.min_amt > new_data.min_amt {
            self.min_amt = new_data.min_amt;
        }
        if self.max_amt < new_data.max_amt {
            self.max_amt = new_data.max_amt;
        }
        if self.min_int_rate > new_data.min_int_rate {
            self.min_int_rate = new_data.min_int_rate;
        }
        if self.max_int_rate < new_data.max_int_rate {
            self.max_int_rate = new_data.max_int_rate;
        }
        self.int_rate_sum += new_data.int_rate_sum;
    }
}
