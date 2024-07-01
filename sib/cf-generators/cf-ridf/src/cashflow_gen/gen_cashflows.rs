use crate::configuration_parameters::ConfigurationParameters;
use cashflow_gen::account_appender::create_account_with_cashflows;
use cashflow_gen::account_reader::input_account::InputAccount;
use cashflow_gen::account_with_cashflows::Cashflow;
use cashflow_gen::account_writer::AccountWithCashflowsWriter;
use chrono::Datelike;
use macros;
use rbdate::*;
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
use std::time::SystemTime;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    writer: &mut AccountWithCashflowsWriter,
) -> (f64, f64, usize) {
    let total_prin_amt = 0.0;
    let mut cashflow: Vec<Cashflow> = Vec::new();
    let mut cf_date = account.date_of_mobilization.unwrap_or_default();
    let due_dt = account.due_date.unwrap_or_default();
    let os_bal = account.os_bal;
    let mut next_date = get_quarter_end_date(account.date_of_mobilization.unwrap_or_default());

    while cf_date < due_dt {
        let (date_diff, days_in_yr) =
            get_date_diff(cf_date, next_date, config_params.day_convention());

        if next_date > due_dt {
            break;
        }
        cf_date = next_date;
        let int_amt =
            ((account.deposit * account.rate_of_interest / 100.0) / (days_in_yr)) * date_diff;
        let cf = new_cashflow(int_amt, 0.0, timestamp(cf_date));
        if cf_date >= *config_params.as_on_date() {
            cashflow.push(cf);
        }
        next_date = get_quarter_end_date(incr_dt_by_days(cf_date, 1));
    }

    if due_dt > *config_params.as_on_date() {
        let (date_diff, days_in_yr) =
            get_date_diff(cf_date, due_dt, config_params.day_convention());
        let last_int_amt =
            ((account.deposit * account.rate_of_interest / 100.0) / days_in_yr) * date_diff;
        let last_cf = new_cashflow(last_int_amt, os_bal, timestamp(due_dt));
        cashflow.push(last_cf);
    }

    let (account_with_cashflows, int_amt, prin_amt, num_of_cfs) = log_measurements!(
        log,
        [format!(
            "Type: CreateAccWithCFs, Identifier: {}",
            account.transche_desc
        )],
        create_account_with_cashflows(account.clone(), total_prin_amt, cashflow.to_owned())
    );
    info!(
        log,
        "generated account with cash-flow:{:?}", account_with_cashflows
    );
    log_measurements!(
        log,
        [format!(
            "Type: WriteAccWithCFs, Identifier: {}",
            account_with_cashflows.tranche_desc
        )],
        writer.write(account_with_cashflows)
    );
    (int_amt, prin_amt, num_of_cfs)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}

fn get_date_diff(
    start_date: NaiveDate,
    end_date: NaiveDate,
    convention: &Conventions,
) -> (f64, f64) {
    let days = days_with_convn(start_date, end_date, convention).unwrap();
    (days.days_btw_dts as f64, days.day_in_yr as f64)
}

pub fn get_quarter_end_date(date: NaiveDate) -> NaiveDate {
    if vec![1, 2, 3].contains(&date.month()) {
        NaiveDate::from_ymd_opt(date.year(), 3, 31).unwrap_or(date)
    } else if vec![4, 5, 6].contains(&date.month()) {
        NaiveDate::from_ymd_opt(date.year(), 6, 30).unwrap_or(date)
    } else if vec![7, 8, 9].contains(&date.month()) {
        NaiveDate::from_ymd_opt(date.year(), 9, 30).unwrap_or(date)
    } else {
        NaiveDate::from_ymd_opt(date.year(), 12, 31).unwrap_or(date)
    }
}
