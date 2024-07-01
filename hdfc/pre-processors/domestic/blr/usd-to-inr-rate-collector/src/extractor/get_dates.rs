use chrono::Datelike;
use rbdate::*;

pub struct Dates {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Dates {
    pub fn new(as_on_date: NaiveDate) -> Self {
        Self {
            start_date: NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 1),
            end_date: as_on_date,
        }
    }
}
