#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Schedules {
    pub idx: i64,
    pub acc_no: String,
    pub lnacrsdtl_intrnl_acc_no: i64,
    pub sr_no: i64,
    pub ccy: String,
    pub amt: f64,
    pub freq: String,
    pub from_dt: String,
    pub no_of_instals: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Cashflows {
    pub ccy: String,
    pub amt: f64,
    pub freq: String,
    pub from_dt: String,
}

impl Cashflows {
    pub fn new() -> Self {
        Cashflows {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, other: Schedules) {
        self.ccy = other.ccy;
        self.amt = other.amt;
        self.freq = other.freq;
        self.from_dt = other.from_dt;
    }

    pub fn print(&self) -> String {
        format!("{}:{}:{}:{},", self.ccy, self.amt, self.freq, self.from_dt)
    }
}
