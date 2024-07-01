use statics::DEFAULT_FLOAT;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct NormalizeData {
    pub ccy_id: String,
    pub cust_grp_id: String,
    pub cust_grp_name: String,
    pub lcy_amt: f64,
    pub fcy_amt: f64,
}

impl NormalizeData {
    pub fn new() -> NormalizeData {
        NormalizeData {
            ccy_id: String::new(),
            cust_grp_id: String::new(),
            cust_grp_name: String::new(),
            lcy_amt: DEFAULT_FLOAT,
            fcy_amt: DEFAULT_FLOAT,
        }
    }

    pub fn insert(
        &mut self,
        ccy_id: String,
        cust_grp_id: String,
        cust_grp_name: String,
        lcy_amt: f64,
        fcy_amt: f64,
    ) {
        self.ccy_id = ccy_id;
        self.cust_grp_id = cust_grp_id;
        self.cust_grp_name = cust_grp_name;
        self.lcy_amt = lcy_amt;
        self.fcy_amt = fcy_amt;
    }

    pub fn add(&mut self, other: NormalizeData) {
        self.lcy_amt += other.lcy_amt;
        self.fcy_amt += other.fcy_amt;
    }

    pub fn add_cust_det(&mut self, cust_grp_id: String, cust_grp_name: String) {
        self.cust_grp_id = cust_grp_id;
        self.cust_grp_name = cust_grp_name;
    }
}

impl Display for NormalizeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}|{}|{}|{}|{}",
            self.ccy_id, self.cust_grp_id, self.cust_grp_name, self.fcy_amt, self.lcy_amt,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AggregatedKey {
    pub client_id: String,
    pub ccy_id: String,
}

impl AggregatedKey {
    pub fn new(client_id: String, ccy_id: String) -> Self {
        AggregatedKey { client_id, ccy_id }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustMaster {
    pub cust_id: String,
    pub cust_type: String,
    pub cust_name: String,
    pub ucc_id: String,
}

impl CustMaster {
    pub fn new(fields: &[&str]) -> Self {
        CustMaster {
            cust_id: fields[0].to_string(),
            cust_type: fields[1].to_string(),
            cust_name: fields[2].to_string(),
            ucc_id: fields[3].to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustMasterOutput {
    pub cust_id: String,
    pub cust_name: String,
}

impl CustMasterOutput {
    pub fn new(cust_master: &CustMaster) -> Self {
        CustMasterOutput {
            cust_id: cust_master.cust_id.to_string(),
            cust_name: cust_master.cust_name.to_string(),
        }
    }
}
