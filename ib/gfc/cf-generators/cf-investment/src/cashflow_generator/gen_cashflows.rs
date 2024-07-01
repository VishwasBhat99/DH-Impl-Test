use std::convert::TryInto;

use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::prelude::*;
use chrono::{Datelike, Duration, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, num_days_start_to_end, timestamp};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut cf_dt: i64 = timestamp(account.maturity_date);
    let as_on_dt: i64;
    let mut int_rt: f64 = 0.0;
    let mut amt: f64 = 0.0;
    let mut cashflows: Vec<Cashflow> = Vec::new();
    if let val = account.book_value {
        amt = val
    } else {
        log_error!(
            log,
            "`oustanding balance` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
    }
    let coupon_rate = account.coupon_rate;
    let coupon_freq = &account.coupon_frequency;
    let coupon_basis = &account.coupon_basis;
    amt = account.book_value;
    let formated_last_coupon_date = &account.last_coupon.replace(".", "-").replace("/", "-");
    let mut last_coupon = rbdate::NaiveDate::parse_from_str(&formated_last_coupon_date, "%d-%m-%Y")
        .unwrap_or(*config_params.as_on_date());

    let mut next_coupon = account.next_coupon;

    if coupon_rate == 0.0 && coupon_freq == "" && coupon_basis != "" {
        int_rt = account.book_value - account.face_value;
        cf_dt = timestamp(account.maturity_date);
        let cf = new_cashflow(int_rt, amt, cf_dt);
        cashflows.push(cf);
    } else {
        let mat_date = account.maturity_date;
        let mut temp_coupon = account.next_coupon;
        let mut days_diff = num_days_start_to_end(last_coupon, next_coupon);
        while next_coupon <= mat_date {
            int_rt =
                ((account.book_value * account.coupon_rate * days_diff as f64) / 360.0) / 100.0;
            let mut cf: Cashflow = new_cashflow(int_rt, 0.0, timestamp(next_coupon));
            if next_coupon == mat_date {
                cf = new_cashflow(int_rt, amt, timestamp(next_coupon));
            }

            next_coupon = increment_date_by_months(next_coupon, 6);
            days_diff = num_days_start_to_end(temp_coupon, next_coupon);
            temp_coupon = next_coupon;
            cashflows.push(cf);
        }
    }
    cashflows
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}

fn increment_date_by_months(start_date: NaiveDate, months: i32) -> NaiveDate {
    let mut result_date = start_date;
    result_date = result_date
        .checked_add_signed(Duration::days((months * 30) as i64))
        .unwrap_or(result_date);
    let next_month = NaiveDate::from_ymd(result_date.year(), result_date.month() + 1, 1);
    let last_day_of_month = next_month - Duration::days(1);

    last_day_of_month
}
