#[derive(Debug, Clone, PartialEq)]
pub struct OutputData {
    pub curr: String,
    pub amt: f64,
}

impl OutputData {
    pub fn new(curr: String, amt: f64) -> OutputData {
        OutputData {
            curr: curr.to_string(),
            amt: amt.to_string().trim().parse::<f64>().unwrap_or(0.0).abs(),
        }
    }
}

impl OutputData {
    pub fn append_data(&mut self, new_data: Self) {
        self.amt += new_data.amt;
    }
}
