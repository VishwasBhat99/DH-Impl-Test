use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

pub fn get_aggregation_date_limit(as_on_date: &NaiveDate) -> i64 {
    let thirty_years_from_now = as_on_date
        .and_hms(0,0,0) + Duration::weeks(1560);

    thirty_years_from_now.timestamp()
}

pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    // TODO: Obviously wasteful!
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}