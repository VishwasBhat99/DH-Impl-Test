use super::input_records::InputRecord;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct AggregatedKey {
    pub as_on: String,
    pub src_id: String,
    pub is_src_tb: String,
    pub llg_id: i64,
    pub ccy_id: String,
}

impl AggregatedKey {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.as_on, self.src_id, self.is_src_tb, self.llg_id, self.ccy_id,
        )
    }

    pub fn new() -> Self {
        AggregatedKey {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, rec: InputRecord, src_id: &str, is_src_tb: &str) {
        self.as_on = rec.as_on;
        self.src_id = src_id.to_string();
        self.is_src_tb = is_src_tb.to_string();
        self.llg_id = rec.llg_id;
        self.ccy_id = rec.ccy_id;
    }
}

#[derive(Debug, Default, Clone)]
pub struct AggregatedValue {
    pub bal: f64,
    pub w_int_rt: f64,
}

impl AggregatedValue {
    pub fn print(&self) -> String {
        format!(
            "{}|{}",
            self.bal,
            if self.bal == 0.0 {
                0.0
            } else {
                self.w_int_rt / self.bal
            }
        )
    }

    pub fn new() -> Self {
        AggregatedValue {
            ..Default::default()
        }
    }

    pub fn add(&mut self, rec: InputRecord) {
        self.bal += rec.bal;
        self.w_int_rt += rec.bal * rec.rate;
    }
}

#[derive(Debug, Default)]
pub struct AggregatedMap {
    pub store: HashMap<AggregatedKey, AggregatedValue>,
}

impl AggregatedMap {
    pub fn new() -> Self {
        AggregatedMap {
            store: HashMap::new(),
        }
    }
}
