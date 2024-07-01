use chrono::Datelike;
use chrono::NaiveDate;
use rbdate::{get_days_from_month, num_days_start_to_end};

pub struct GetDates {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub no_of_days: i64,
}

impl GetDates {
    pub fn new(as_on_date: NaiveDate) -> GetDates {
        let month = as_on_date.month();
        let year = as_on_date.year();
        let start_date: NaiveDate = NaiveDate::from_ymd(year, month, 1);
        let mut no_of_days: i64 = get_days_from_month(start_date);
        let end_date: NaiveDate = NaiveDate::from_ymd(year, month, no_of_days as u32);
        if as_on_date != end_date {
            no_of_days = num_days_start_to_end(start_date, as_on_date);
        }

        GetDates {
            start_date,
            end_date,
            no_of_days,
        }
    }
}
