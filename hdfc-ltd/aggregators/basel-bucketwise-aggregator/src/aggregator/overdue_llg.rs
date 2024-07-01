use std::collections::HashMap;
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
) -> Option<i32> {
    for (period, residual_period_cat) in residual_period_map.iter() {
        if period.start_period <= residual_period && period.end_period > residual_period {
            return Some(*residual_period_cat);
        }
    }
    None // Return None if no matching period is found
}
