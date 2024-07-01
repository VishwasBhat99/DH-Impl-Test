use chrono::Datelike;
use chrono::NaiveDate;
use rbdate::num_days_start_to_end;

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
        let end_date: NaiveDate = match NaiveDate::from_ymd_opt(year, month, 31) {
            Some(date) => date,
            None => match NaiveDate::from_ymd_opt(year, month, 30) {
                Some(date) => date,
                None => match NaiveDate::from_ymd_opt(year, month, 29) {
                    Some(date) => date,
                    None => NaiveDate::from_ymd(year, month, 28),
                },
            },
        };
        let mut no_of_days: i64 = num_days_start_to_end(start_date, end_date);
        no_of_days += 1;

        GetDates {
            start_date,
            end_date,
            no_of_days,
        }
    }
}
