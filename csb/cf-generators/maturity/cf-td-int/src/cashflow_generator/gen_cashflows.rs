use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::{Duration, NaiveDate};
use macros;
use rbdate;
use slog::Logger;
use statics::*;

pub fn generate_cashflows(
    as_on_date: &NaiveDate,
    account: &mut InputAccount,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let prin_amt = if account.is_overdue == "YES" {
        generate_interest_on_demand_liability(
            account.bal_os,
            account.over_int_rt,
            account.resid_days,
        )
    } else if *as_on_date > account.mat_dt {
        DEFAULT_FLOAT
    } else {
        generate_interest_on_demand_liability(account.bal_os, account.int_rt, account.resid_days)
    };

    let cf_date = if account.is_overdue == "YES" {
        *as_on_date
    } else if account.pay_freq == "X" {
        account.mat_dt
    } else if account.pay_freq == "M" {
        account.max_date + Duration::days(30)
    } else if account.pay_freq == "Q" {
        account.max_date + Duration::days(90)
    } else if account.pay_freq == "H" {
        account.max_date + Duration::days(180)
    } else if account.pay_freq == "M" {
        account.max_date + Duration::days(365)
    } else {
        account.max_date
    };

    log_debug!(
        log,
        "account: `{}`, print_amt: `{}`, cf_date: `{}`",
        account.acc_no,
        prin_amt,
        cf_date
    );

    Ok(vec![new_cashflow(DEFAULT_FLOAT, prin_amt, &cf_date)])
}

fn generate_interest_on_demand_liability(o_a: f64, i_r: f64, num_days: i64) -> f64 {
    (o_a * i_r * num_days as f64) / (365.0 * 100.0)
}

fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = rbdate::timestamp(*d);

    cf
}
