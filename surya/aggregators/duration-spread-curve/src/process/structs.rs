use std::collections::HashMap;

#[derive(Debug)]
pub struct AggrData {
    pub data: HashMap<i64, AggrVal>,
}
impl AggrData {
    pub fn aggr_data(&mut self, amt: f64, bucket: i64, rate: f64, curveid: String) {
        self.data
            .entry(bucket)
            .and_modify(|val| val.calc_weighted_rate(amt, rate))
            .or_insert(AggrVal { amt, rate, curveid });
    }
}

#[derive(Debug)]
pub struct AggrVal {
    pub amt: f64,
    pub rate: f64,
    pub curveid: String,
}
impl AggrVal {
    pub fn calc_weighted_rate(&mut self, amt: f64, rate: f64) {
        self.rate = if (self.amt + amt) == 0.0 {
            0.0
        } else {
            (self.rate * self.amt + rate * amt) / (self.amt + amt)
        };
        self.amt += amt;
    }
}
