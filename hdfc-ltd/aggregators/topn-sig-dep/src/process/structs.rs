#[derive(Debug, Default, PartialEq)]
pub struct CustDet {
    pub curr: String,
    pub amt: f64,
}

impl CustDet {
    pub fn new() -> CustDet {
        CustDet {
            curr:String::new(),
            amt: 0.0,
        }
    }

    pub fn update_amt(&mut self, custdata: &mut CustDet) {
        self.amt += custdata.amt;
    }
}
