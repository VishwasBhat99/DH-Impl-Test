extern crate chrono;
extern crate rbdate;

mod tests;

use chrono::ParseError;
use rbdate::*;

pub fn get_tenor_days(
    start_date: Result<NaiveDate, ParseError>,
    end_date: Result<NaiveDate, ParseError>,
) -> i64 {
    if let Ok(start_date) = start_date {
        if let Ok(end_date) = end_date {
            num_days_start_to_end(start_date, end_date)
        } else {
            0
        }
    } else {
        0
    }
}

pub fn get_tenor_desc<'a>(tenor: i64, value_date: NaiveDate) -> &'a str {
    if tenor < get_num_days_from_add(value_date, 0, 0, 7) {
        "below 7 days"
    } else if tenor >= get_num_days_from_add(value_date, 0, 0, 7)
        && tenor <= get_num_days_from_add(value_date, 0, 0, 14)
    {
        "7 - 14 days"
    } else if tenor >= get_num_days_from_add(value_date, 0, 0, 15)
        && tenor <= get_num_days_from_add(value_date, 0, 0, 29)
    {
        "15 - 29 days"
    } else if tenor >= get_num_days_from_add(value_date, 0, 0, 30)
        && tenor <= get_num_days_from_add(value_date, 0, 0, 45)
    {
        "30 - 45 days"
    } else if tenor >= get_num_days_from_add(value_date, 0, 0, 46)
        && tenor <= get_num_days_from_add(value_date, 0, 0, 60)
    {
        "46 - 60 days"
    } else if tenor >= get_num_days_from_add(value_date, 0, 0, 61)
        && tenor <= get_num_days_from_add(value_date, 0, 0, 90)
    {
        "61 - 90 days"
    } else if tenor >= get_num_days_from_add(value_date, 0, 0, 91)
        && tenor <= get_num_days_from_add(value_date, 0, 6, 0)
    {
        "91 days - 6 months"
    } else if tenor >= get_num_days_from_add(value_date, 0, 6, 1)
        && tenor < get_num_days_from_add(value_date, 0, 9, 0)
    {
        "6 months 1 day - 9 months"
    } else if tenor >= get_num_days_from_add(value_date, 0, 9, 1)
        && tenor < get_num_days_from_add(value_date, 1, 0, 0)
    {
        "9 months 1 day - 1 year"
    } else if tenor == get_num_days_from_add(value_date, 1, 0, 0) {
        "1 year"
    } else if tenor >= get_num_days_from_add(value_date, 1, 0, 1)
        && tenor < get_num_days_from_add(value_date, 2, 0, 0)
    {
        "1 year 1 day - 2 years"
    } else if tenor >= get_num_days_from_add(value_date, 2, 0, 1)
        && tenor < get_num_days_from_add(value_date, 3, 0, 0)
    {
        "2 years 1 day - 3 years"
    } else if tenor >= get_num_days_from_add(value_date, 3, 0, 1)
        && tenor < get_num_days_from_add(value_date, 5, 0, 0)
    {
        "3 years 1 day - 5 years"
    } else if tenor >= get_num_days_from_add(value_date, 5, 0, 1)
        && tenor < get_num_days_from_add(value_date, 10, 0, 0)
    {
        "5 years 1 day - 10 years"
    } else {
        "above 10 years"
    }
}

pub fn get_tenor_months(
    start_date_opt: Result<NaiveDate, ParseError>,
    end_date_opt: Result<NaiveDate, ParseError>,
) -> i64 {
    if let Ok(start_date) = start_date_opt {
        if let Ok(end_date) = end_date_opt {
            (num_days_start_to_end(start_date, end_date) as f64 * 12.0 / 365.0).round() as i64
        } else {
            0
        }
    } else {
        0
    }
}
