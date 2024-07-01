#[derive(Debug, Copy, Clone)]
pub struct AggregateData {
    pub tot_prin_amt: f64,
    pub rate_amount_weighted: f64,
}

impl AggregateData {
    pub fn new() -> AggregateData {
        AggregateData {
            tot_prin_amt: 0.0,
            rate_amount_weighted: 0.0,
        }
    }

    pub fn add_data(&mut self, p_a: f64, r: f64) {
        self.tot_prin_amt += p_a;
        self.rate_amount_weighted += p_a * r;
    }

    pub fn add_from_builder(&mut self, other: AggregateData) {
        self.tot_prin_amt += other.tot_prin_amt;
        self.rate_amount_weighted += other.rate_amount_weighted;
    }

    pub fn values_divided_by(&mut self, multiplier: f64) {
        self.tot_prin_amt /= multiplier;
        self.rate_amount_weighted /= multiplier;
    }
}
