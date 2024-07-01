#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub tot_prin_amt_org: f64,
    pub tot_prin_amt_lcy: f64,
    pub tot_prin_amt_hcy: f64,
    pub rt_prin_amt_weighted: f64,
}

impl Data {
    pub fn append_data(&mut self, new_data: Data) {
        self.tot_prin_amt_org += new_data.tot_prin_amt_org;
        self.tot_prin_amt_lcy += new_data.tot_prin_amt_lcy;
        self.tot_prin_amt_hcy += new_data.tot_prin_amt_hcy;
        self.rt_prin_amt_weighted += new_data.rt_prin_amt_weighted;
    }

    pub fn values_multiplied_by(&mut self, multiplier: f64, is_consolidated: bool) {
        if is_consolidated {
            self.tot_prin_amt_lcy *= multiplier;
        } else {
            self.tot_prin_amt_hcy *= multiplier;
        }
        self.tot_prin_amt_org *= multiplier;
        self.rt_prin_amt_weighted *= multiplier;
    }

    pub fn print(&self) -> String {
        format!(
            "{:.2}|{:.2}|{:.2}",
            self.tot_prin_amt_lcy,
            self.tot_prin_amt_hcy,
            if self.tot_prin_amt_org == 0.0 {
                0.0
            } else {
                self.rt_prin_amt_weighted / self.tot_prin_amt_org
            }
        )
    }
}
