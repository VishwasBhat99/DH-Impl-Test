use std::default::Default;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub mis_dt: String,
    pub prod_cod: String,
    pub count: String,
    pub pos: String,
    pub fc_count: String,
    pub fc_pos: String,
}

#[derive(Debug, Clone, Default)]
pub struct ForeClosureAmounts {
    pub pos: f64,
    pub fc_pos: f64,
}

impl ForeClosureAmounts {
    pub fn new() -> ForeClosureAmounts {
        Default::default()
    }
    pub fn insert(&mut self, fc_pos: f64) {
        self.fc_pos = fc_pos;
    }
    pub fn add_amts(&mut self, fc_pos: f64) {
        self.fc_pos += fc_pos;
    }
}
