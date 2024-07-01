use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataMap {
    pub track: String,
    pub data: HashMap<String, Bucketdata>,
}

#[derive(Debug, Clone)]
pub struct Bucketdata {
    pub one_to_twenty_eight_days: f64,
    pub twenty_nine_days_to_three_mon: f64,
    pub three_mon_to_six_mon: f64,
    pub six_mon_to_one_yr: f64,
    pub one_yr_to_three_yr: f64,
    pub three_yr_to_five_yr: f64,
    pub five_yr_to_seven_yr: f64,
    pub seven_yr_to_ten_yr: f64,
    pub ten_yr_to_fiveteen_yr: f64,
    pub over_fifteen: f64,
}

impl DataMap {
    pub fn new() -> DataMap {
        DataMap {
            track: "".to_string(),
            data: HashMap::new(),
        }
    }
}
impl Bucketdata {
    pub fn new() -> Bucketdata {
        Bucketdata {
            one_to_twenty_eight_days: 0.0,
            twenty_nine_days_to_three_mon: 0.0,
            three_mon_to_six_mon: 0.0,
            six_mon_to_one_yr: 0.0,
            one_yr_to_three_yr: 0.0,
            three_yr_to_five_yr: 0.0,
            five_yr_to_seven_yr: 0.0,
            seven_yr_to_ten_yr: 0.0,
            ten_yr_to_fiveteen_yr: 0.0,
            over_fifteen: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldStructure {
    pub notional: f64,
    pub coupons: f64,
    pub yields: f64,
    pub mod_duration: f64,
}

impl FieldStructure {
    pub fn new() -> FieldStructure {
        FieldStructure {
            notional: 0.0,
            coupons: 0.0,
            yields: 0.0,
            mod_duration: 0.0,
        }
    }
}
