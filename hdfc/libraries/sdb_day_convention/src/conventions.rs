#[derive(Copy, Clone, Debug)]
pub enum Conventions {
    ACTbyACT,
    ACTby365,
    ACTby360,
    Thirtyby360,
    AccruedThirtyby360,
}
#[derive(Debug, Default)]
pub struct Days {
    pub days_btw_dts: i64,
    pub day_in_yr: i64,
}

impl Days {
    pub fn new() -> Days {
        ::std::default::Default::default()
    }
}

pub fn new_days(days_btw_dts: i64, day_in_yr: i64) -> Days {
    let mut days = Days::new();
    days.days_btw_dts = days_btw_dts;
    days.day_in_yr = day_in_yr;
    days
}
