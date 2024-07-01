use chrono::{Datelike, NaiveDate};

pub fn is_first_day_of_month(date: NaiveDate) -> bool {
    if date.day() == 1 {
        true
    } else {
        false
    }
}
