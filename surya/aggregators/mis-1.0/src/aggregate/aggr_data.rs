#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub tot_prin_amt: f64,
    pub tot_prin_amt_lcy: f64,
    pub rt_prin_amt_weighted: f64,
}

impl Data {
    pub fn append_data(&mut self, new_data: Data) {
        self.tot_prin_amt += new_data.tot_prin_amt;
        self.tot_prin_amt_lcy += new_data.tot_prin_amt_lcy;
        self.rt_prin_amt_weighted += new_data.rt_prin_amt_weighted;
    }
}
