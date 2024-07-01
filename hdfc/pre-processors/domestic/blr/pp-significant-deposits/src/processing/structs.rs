use std::fmt;
#[derive(Debug)]
pub struct UcicDet {
    pub ucic_id: String,
    pub ucic_name: String,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct DepositsDet {
    pub ca_bal: f64,
    pub sa_bal: f64,
    pub td_bal: f64,
    pub tot_bal: f64,
}

impl DepositsDet {
    pub fn new() -> DepositsDet {
        DepositsDet {
            ..Default::default()
        }
    }
    pub fn update_deposistsdet(&mut self, dep_data: DepositsDet) {
        self.ca_bal += dep_data.ca_bal;
        self.sa_bal += dep_data.sa_bal;
        self.td_bal += dep_data.td_bal;
        self.tot_bal += dep_data.tot_bal;
    }
}

#[derive(Debug, Clone)]
pub struct TopNDepDet {
    pub ucic_id: String,
    pub ucic_name: String,
    pub ca_bal: f64,
    pub sa_bal: f64,
    pub td_bal: f64,
    pub tot_bal: f64,
}

impl TopNDepDet {
    pub fn new() -> TopNDepDet {
        TopNDepDet {
            ucic_id: String::new(),
            ucic_name: String::new(),
            ca_bal: 0.0,
            sa_bal: 0.0,
            td_bal: 0.0,
            tot_bal: 0.0,
        }
    }

    pub fn update_topndepdet(&mut self, topn_data: &mut TopNDepDet) {
        self.ca_bal += topn_data.ca_bal;
        self.sa_bal += topn_data.sa_bal;
        self.td_bal += topn_data.td_bal;
        self.tot_bal += topn_data.tot_bal;
    }
}

impl fmt::Display for TopNDepDet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.ucic_id,
            self.ucic_name,
            self.sa_bal,
            self.sa_bal,
            self.ca_bal,
            self.ca_bal,
            self.td_bal,
            self.td_bal,
            self.tot_bal,
            self.tot_bal
        )
    }
}
