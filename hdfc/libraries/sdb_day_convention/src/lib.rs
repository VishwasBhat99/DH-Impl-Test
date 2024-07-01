//! Day Count Convention
extern crate chrono;
extern crate rbdate;
pub mod conventions;

mod test;

use chrono::Datelike;
pub use conventions::{new_days, Conventions, Days};
use rbdate::{NaiveDate, incr_dt_by_mon_presrv_eom_checked};
use rbdate::{decr_dt_by_mon_presrv_eom, incr_dt_by_mon_presrv_eom};

/// This function returns `number of days between the dates` and `total number of days in the year according to the convention passed to the function`.<br/><br/>
/// `This function supports the following conventions:`<br/>
///     1. `ACTbyACT`: Consider actual number of days in each month and consider the actual number of days in that year.<br/>
///     2. `ACTby365`: Consider actual number of days in each month and each year contains 365 days. including leap years.<br/>
///     3. `ACTby360`: Consider actual number of days in each month and each year is a 360-day year.<br/>
///     4. `Thirtyby360` : Consider each month is a 30 day month and each year is a 360-day year.<br/><br/>
/// `Usage`: The user of this function can use `number of days between the dates` and `total number of days in the year` to calculate the interest for a period.<br/><br/>
/// `Error`: This function may throw the following errors:-<br/>
///         1. `Start date cannot be greater than end date` : If start date is after the end date.<br/><br/>
pub fn days_with_convn(
    st_dt: NaiveDate,
    end_dt: NaiveDate,
    convention: &Conventions,
) -> Result<Days, String> {
    if st_dt > end_dt {
        return Err(format!(
            "Start Date is after End Date, Start Date: {}, End Date: {}",
            st_dt, end_dt
        ));
    }
    let days = match convention {
        Conventions::ACTbyACT => {
            let days = rbdate::num_days_start_to_end(st_dt, end_dt);
            let next_yr_date = incr_dt_by_mon_presrv_eom_checked(end_dt, 12).expect("Cannot get next date using incr date eom function.");
            let dy_yr = rbdate::num_days_start_to_end(end_dt, next_yr_date);
            new_days(days, dy_yr)
        }
        Conventions::ACTby365 => {
            let days = rbdate::num_days_start_to_end(st_dt, end_dt);
            let dy_yr = 365;
            new_days(days, dy_yr)
        }
        Conventions::ACTby360 => {
            let days = rbdate::num_days_start_to_end(st_dt, end_dt);
            let dy_yr = 360;
            new_days(days, dy_yr)
        }
        Conventions::Thirtyby360 => {
            let dy_yr = 360;
            let prev_dt = st_dt;
            let mut month_to_incr = 1;
            let mut next_dt = incr_dt_by_mon_presrv_eom(prev_dt, month_to_incr)
                .expect("Cannot get next date using incr date eom function.");
            let mut days: i64 = 0;
            loop {
                if next_dt <= end_dt {
                    days += 30;
                } else {
                    break;
                }
                month_to_incr += 1;
                next_dt = incr_dt_by_mon_presrv_eom(prev_dt, month_to_incr)
                    .expect("Cannot get next date using incr date eom function.");
            }
            new_days(days, dy_yr)
        }
        _ => {
            return Err(format!(
                "Cannot compute for convention: {:?},Use a different convention.",
                convention
            ))
        }
    };
    Ok(days)
}

/// The function calculates the `number of accrued days between the given start and end dates according to the convention and frequency passed to the function`.<br/><br/>
/// `This function supports the following conventions:`<br/>
///     1. `AccruedThirtyby360` : Consider each month is a 30 day month and each year is a 360-day year.<br/><br/>
/// The function returns the `number of accrued days`.<br/><br/>

pub fn accrued_days_with_convn(
    st_dt: NaiveDate,
    end_dt: NaiveDate,
    freq: usize,
    convention: &Conventions,
) -> Result<Days, String> {
    let dy_yr = 360;
    if st_dt > end_dt {
        return Ok(new_days(0, dy_yr));
    }
    let days = match convention {
        Conventions::AccruedThirtyby360 => {
            let mut accrued_days = 0;
            let mut num_mon = 0;
            accrued_days = day_count_30_360_days(st_dt, end_dt);
            while day_count_30_360_days(st_dt, decr_dt_by_mon_presrv_eom(end_dt, num_mon).unwrap())
                > 0
            {
                num_mon += freq;
            }
            accrued_days =
                day_count_30_360_days(st_dt, decr_dt_by_mon_presrv_eom(end_dt, num_mon).unwrap());
            //Note: The function returns the absolute value of the accrued days.
            new_days((accrued_days - 1).abs(), dy_yr)
        }
        _ => return Err(format!("Not correct convention: {:?}", convention)),
    };
    Ok(days)
}

fn day_count_30_360_days(st_dt: NaiveDate, end_dt: NaiveDate) -> i64 {
    let st_month = st_dt.month() as i64;
    let end_month = end_dt.month() as i64;
    let st_year = st_dt.year() as i64;
    let end_year = end_dt.year() as i64;
    let month_diff = (end_year - st_year) * 12 - st_month + end_month;
    let st_day = st_dt.day() as i64;
    let end_day = end_dt.day() as i64;
    let mut day_count = month_diff * 30 + end_day - st_day;
    if st_day == 31 && end_day != 31 {
        day_count += 1;
    }
    day_count
}

/// This function returns the `interest amount` for a given prinicipal amount, interest rate, day convention and start day to end day period.
/// `This function supports the following conventions:`
///     1. `ACTbyACT`: Consider actual number of days in each month and consider the actual number of days in that year.
///     2. `ACTby365`: Consider actual number of days in each month and each year contains 365 days. including leap years.
///     3. `ACTby360`: Consider actual number of days in each month and each year is a 360-day year.
///     4. `Thirtyby360` : Consider each month is a 30 day month and each year is a 360-day year.
/// `Usage`: The user of this function can use `interest amount`, calculated based on day convention, to report the interest for a period.
/// `Error`: This function may throw the following errors:-
///         1. `Start date cannot be greater than end date` : If start date is after the end date.
///         2. `Could not fetch time period` : If days_with_convn() function does not work

pub fn get_int_amt(
    st_dt: NaiveDate,
    end_dt: NaiveDate,
    convention: &Conventions,
    prin_amt: f64,
    int_rt: f64,
) -> Result<f64, String> {
    if st_dt > end_dt {
        return Err(format!(
            "Start Date is after End Date, Start Date: {}, End Date: {}",
            st_dt, end_dt
        ));
    }
    let time_period = match days_with_convn(st_dt, end_dt, convention) {
        Ok(val) => val,
        Err(_) => {
            return Err(format!(
            "Could not fetch time period for start date: {}, end date: {} and day convention: {:?}",
            st_dt, end_dt, convention
        ))
        }
    };
    let int_amt = ((prin_amt * int_rt) / 100.0)
        * ((time_period.days_btw_dts as f64) / (time_period.day_in_yr as f64));
    Ok(int_amt)
}
