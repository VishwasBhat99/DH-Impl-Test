use std::{collections::HashMap, path::StripPrefixError};
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct ResidualPeriod {
    pub start_period: i32,
    pub end_period: i32,
}

impl ResidualPeriod {
    pub fn new(start_period: String, end_period: String) -> ResidualPeriod {
        ResidualPeriod {
            start_period: start_period.trim().parse::<i32>().unwrap_or(0),
            end_period: end_period.trim().parse::<i32>().unwrap_or(0),
        }
    }
}

pub fn get_overdue_llg(
    residual_period: i32,
    residual_period_map: &HashMap<ResidualPeriod, i32>,
) -> i32 {
    for (period, residual_period_cat) in residual_period_map.iter() {
        if period.start_period <= residual_period && period.end_period > residual_period {
            return *residual_period_cat;
        }
    }
    panic!("Could not get overdue_llg!");
}
