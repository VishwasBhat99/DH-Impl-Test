#![cfg_attr(feature = "strict-build", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::expect_fun_call))]
#![cfg_attr(
    feature = "cargo-clippy",
    deny(
        clippy::result_unwrap_used,
        clippy::panicking_unwrap,
        clippy::option_unwrap_used
    )
)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::float_cmp))]
//! # RBDate
//!
//! RBDate provides a collection of libraries that make it easy to work with dates
// TODO: Change the line above when the purpose of the crate is defined better. ^
extern crate chrono;

#[allow(dead_code)]
#[allow(unused_imports)]
mod tests;

use chrono::Datelike;
use chrono::Duration;
pub use chrono::NaiveDate;
pub use chrono::NaiveDateTime;
use chrono::ParseError;
use chrono::Utc;
use std::cmp::min;
use std::convert::TryInto;

/// Adds `month_count` number of months to `date` and returns a new date.
///
/// This method doesn't check for integer overflows.
/// Please refer to `increment_date_by_months` for integer overflow safety.
pub fn increment_date_by_months_unchecked(date: NaiveDate, month_count: u16) -> NaiveDate {
    let next_month_0 = i64::from(date.month0()) + i64::from(month_count);
    let additional_years = (next_month_0 / 12) as i32;
    let next_month_0 = (next_month_0 % 12) as u32;
    let next_year = date.year() + (additional_years);
    let next_day = min(date.day(), last_day_of_month_0(next_year, next_month_0));

    NaiveDate::from_ymd_opt(next_year, next_month_0 + 1, next_day)
        .expect("Cannot create a valid NaiveDate")
}

/// Adds `month_count` number of months to `date` and returns a new date.
pub fn increment_date_by_months(date: NaiveDate, month_count: u16) -> NaiveDate {
    // Slight modification of: https://github.com/kosta/date-iterator/blob/master/src/calendar_duration.rs
    // Thank you @kosta
    let next_month_0 = i64::from(date.month0())
        .checked_add(i64::from(month_count))
        .expect("Month number out of range for generating next date.");
    let additional_years = next_month_0 / 12;
    let next_month_0 = (next_month_0 % 12) as u32;
    let additional_years = if additional_years >= i64::from(i32::max_value()) {
        panic!(
            "Date too far in the future. Date: {:?}, Month Count: {}",
            date, month_count
        );
    } else {
        additional_years as i32
    };
    let next_year = date
        .year()
        .checked_add(additional_years)
        .expect("Year out of range for generating next date.");
    let next_day = min(date.day(), last_day_of_month_0(next_year, next_month_0));

    NaiveDate::from_ymd_opt(next_year, next_month_0 + 1, next_day)
        .expect("Cannot create a new NaiveDate from given day-month-year.")
}

/// This function adds `months` to `date` and returns the new `NaiveDate`.
/// This function also checks for end of month
/// This method doesn't check for integer overflows.
pub fn incr_dt_by_mon_presrv_eom(date: NaiveDate, months: usize) -> Option<NaiveDate> {
    let mut days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let next_date;
    let next_month = date.month() + months as u32;
    let mut next_month_index = ((next_month % 12) as i32 - 1) as i32;
    if next_month_index < 0 {
        next_month_index = 11;
    }
    let additional_year = {
        if next_month % 12 != 0 {
            next_month / 12
        } else {
            (next_month / 12) - 1
        }
    };

    let next_year = date.year() + additional_year as i32;
    if is_leap_year(next_year) {
        days_per_month[1] = 29;
    }
    let days_in_that_month = days_per_month[(date.month() - 1) as usize];
    let next_day = {
        if date.day() == days_in_that_month {
            days_per_month[next_month_index as usize]
        } else {
            min(date.day(), days_per_month[next_month_index as usize])
        }
    };

    next_date = NaiveDate::from_ymd_opt(next_year, (next_month_index + 1) as u32, next_day);
    next_date
}

/// This function adds `months` to `date` and returns the new `NaiveDate`.
/// This function also checks for end of month
/// This method checks for integer overflows.
pub fn incr_dt_by_mon_presrv_eom_checked(date: NaiveDate, months: usize) -> Option<NaiveDate> {
    let mut days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let next_date;
    let next_month = date
        .month()
        .checked_add(months as u32)
        .expect("Month number out of range for generating next date.");
    let mut next_month_index = ((next_month % 12) as i32 - 1) as i32;
    if next_month_index < 0 {
        next_month_index = 11;
    }
    let additional_year = {
        if next_month % 12 != 0 {
            next_month / 12
        } else {
            (next_month / 12) - 1
        }
    };

    let next_year = date
        .year()
        .checked_add(additional_year as i32)
        .expect("Year out of range for generating next date.");
    let days_in_that_month = days_per_month[(date.month() - 1) as usize];
    if is_leap_year(next_year) {
        days_per_month[1] = 29;
    }
    let next_day = {
        if date.day() == days_in_that_month {
            days_per_month[next_month_index as usize]
        } else {
            min(date.day(), days_per_month[next_month_index as usize])
        }
    };

    next_date = NaiveDate::from_ymd_opt(next_year, (next_month_index + 1) as u32, next_day);
    next_date
}

/// This function subtract `months` from `date` and returns the new `NaiveDate`.
/// This function also checks for end of month
/// This method doesn't check for integer overflows.
pub fn decr_dt_by_mon_presrv_eom(date: NaiveDate, months: usize) -> Option<NaiveDate> {
    let mut days_array = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut prev_year = date.year();
    let mut prev_month = date.month() as i32 - months as i32;
    if prev_month < 0 {
        prev_year -= (months / 12) as i32;
        prev_month = date.month() as i32 - (months % 12) as i32;
    }
    if prev_month <= 0 {
        prev_year -= 1;
        prev_month += 12;
    }
    if is_leap_year(prev_year) {
        days_array[1] = 29;
    }
    let prev_day = if date.day() == days_array[(date.month() - 1) as usize] {
        days_array[(prev_month - 1) as usize]
    } else {
        min(date.day(), days_array[(prev_month - 1) as usize])
    };
    NaiveDate::from_ymd_opt(prev_year, prev_month as u32, prev_day)
}

/// Returns the number of days between the `start` and `end` dates.
///
/// This method assumes the `start` date is greater than the `end` date. Otherwise the function will return -1.
pub fn num_days_start_to_end(start: NaiveDate, end: NaiveDate) -> i64 {
    if start > end {
        -1
    } else {
        i64::from(end.num_days_from_ce() - start.num_days_from_ce())
    }
}

/// Returns a timestamp that corresponds to the start of the `date`.
pub fn timestamp(date: NaiveDate) -> i64 {
    date.and_hms(0, 0, 0).timestamp()
}

pub fn date_from_timestamp(t: i64) -> NaiveDate {
    // TODO: Obviously wasteful!
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

/// A DateParser parses Dates from Strings.
///
/// The `date_format` property specifies the format of the String that represents a date.
///
/// If a date object cannot be parsed, the `use_today_on_error` specifies whether the `parse` method should return
/// today's date or `panic` instead.
pub struct DateParser {
    date_format: String,
    use_today_on_error: bool,
}

impl DateParser {
    /// Creates a new date parser.
    ///
    /// Use the `parse` method to parse a date from a String.
    pub fn new(date_format: String, use_today_on_error: bool) -> DateParser {
        DateParser {
            date_format,
            use_today_on_error,
        }
    }
}

impl DateParser {
    /// Parses the `string` into a date.
    ///
    /// If the `DateParser`'s `use_today_on_error` property is set to true, and the parser cannot parse the
    /// `string`, then you're returned today's date instead.
    pub fn parse(&self, string: &str) -> NaiveDate {
        match NaiveDate::parse_from_str(string, &self.date_format) {
            Ok(parsed_date) => parsed_date,
            Err(error) => {
                if self.use_today_on_error {
                    Utc::now().naive_utc().date();
                }

                panic!(
                    "Could not parse date string. String: '{}', Expected format: '{}'. Parse error: '{}'",
                     string, self.date_format, error
                );
            }
        }
    }

    /// Parses the `string` into an `Option<NaiveDate>`.
    pub fn parse_opt(&self, string: &str) -> Option<NaiveDate> {
        match NaiveDate::parse_from_str(string, &self.date_format) {
            Ok(parsed_date) => Some(parsed_date),
            Err(_error) => None,
        }
    }

    pub fn parse_datetime(&self, string: &str) -> NaiveDateTime {
        match NaiveDateTime::parse_from_str(string, &self.date_format) {
            Ok(datetime) => datetime,
            Err(error) => {
                panic!(
                        "Could not parse date string. String: '{}', Expected format: '{}'. Parse error: '{}'",
                        string, self.date_format, error
                    );
            }
        }
    }
}

/// Outputs the current (local) time in the format `2001-07-08T00:34:60.026490+09:30`
pub fn current_time_utc() -> String {
    format!("{}", chrono::Local::now().format("%+"))
}

fn last_day_of_month_0(year: i32, month_0: u32) -> u32 {
    last_day_of_month(year, month_0 + 1)
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day()
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

/// This function will return NaiveDate corresponding to the given date value
pub fn datevalue_to_naive_date(date: &str) -> Result<NaiveDate, String> {
    if let Ok(timestamp) = date.parse::<f64>() {
        Ok(date_from_timestamp(((timestamp as i64) - 25569) * 86400))
    } else {
        Err(format!("Error in parsing {} as NaiveDate", date))
    }
}

/// This function will return maximum number of days in a month
pub fn get_days_from_month(date: NaiveDate) -> i64 {
    let month = date.month();
    let year = date.year();
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

/// This function will return last date of the month
pub fn get_month_end_date(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(
        date.year(),
        date.month(),
        last_day_of_month(date.year(), date.month()),
    )
}

/// This function will return number of days from given duration
pub fn get_num_days_from_add(start_date: NaiveDate, year: i32, month: u32, day: i64) -> i64 {
    let mut days_per_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut year = start_date.year() + year;
    let mut month = start_date.month() + month;

    if month > 12 {
        month -= 12;
        year += 1;
    }

    if is_leap_year(year) {
        days_per_month[1] = 29;
    }

    let next_day = min(start_date.day(), days_per_month[month as usize - 1]);

    let mut end_date = NaiveDate::from_ymd(year, month, next_day);
    let day_diff_factor = start_date.day() - next_day;
    let mut extra_days = day;
    if day_diff_factor > 0 {
        extra_days += day_diff_factor as i64;
    }
    end_date = date_from_timestamp(timestamp(end_date) + (86400 * extra_days));

    num_days_start_to_end(start_date, end_date)
}

/// increment date by number of days.
pub fn incr_dt_by_days(date: NaiveDate, days: i64) -> NaiveDate {
    let dt = date + Duration::days(days);
    dt
}

/// decrement date by number of days.
pub fn dcr_dt_by_days(date: NaiveDate, days: i64) -> NaiveDate {
    let dt = date - Duration::days(days);
    dt
}

/// decrement date by number of years.
pub fn incr_dt_by_yrs(date: NaiveDate, years: i64) -> NaiveDate {
    let yrs = date.year();
    NaiveDate::from_ymd_opt(
        (yrs as i64 + years).try_into().unwrap(),
        date.month(),
        date.day(),
    )
    .unwrap_or(get_month_end_date(
        NaiveDate::from_ymd_opt((yrs as i64 + years).try_into().unwrap(), date.month(), 01)
            .unwrap_or_else(|| {
                panic!(
                    "Error parsing date from DD:MM:YYYY -> 01:{:?}:{:?}",
                    date.month(),
                    yrs as i64 + years
                )
            }),
    ))
}

pub fn timestamp_to_naivedatetime(timestamp: i64) -> NaiveDateTime {
    let datetime = NaiveDateTime::from_timestamp(timestamp, 0);

    datetime
}

pub fn naivedatetime_to_timestamp(datetime: NaiveDateTime) -> i64 {
    datetime.timestamp()
}

pub fn timestamp_to_naivedate(timestamp: i64) -> NaiveDate {
    let datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    datetime.date()
}
// This function returns `true` if the given date is a month end date, and `false` otherwise.
pub fn is_month_end_date(date: NaiveDate) -> bool {
    let month_end_date = get_month_end_date(date);
    if date == month_end_date {
        true
    } else {
        false
    }
}
