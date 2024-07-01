use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct DailyData {
    pub acc_num: String,
    pub out_bal: String,
    pub int_rt: String,
    pub int_posted: String,
    pub curr_status: String,
    pub class: String,
    pub acc_cls_dt: String,
    pub gl_cd: String,
}

impl DailyData {
    pub fn new() -> DailyData {
        DailyData {
            acc_num: "0".to_string(),
            out_bal: "0.0".to_string(),
            int_rt: "0.0".to_string(),
            int_posted: "0.0".to_string(),
            curr_status: "NA".to_string(),
            class: "na".to_string(),
            acc_cls_dt: "0".to_string(),
            gl_cd: "NA".to_string(),
        }
    }
}
