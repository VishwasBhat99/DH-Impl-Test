use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::{Datelike, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{
    date_from_timestamp, get_days_from_month, get_month_end_date,
    incr_dt_by_mon_presrv_eom_checked, timestamp,
};
use sdb_day_convention::{days_with_convn, Conventions, Days};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_dt: NaiveDate;
    let as_on_dt: i64;
    let mut int_rt: f64 = 0.0;
    let mut amt: f64 = 0.0;
    let mut lst_int_pd_dt: i64 = 0;

    if let Some(val) = account.amt {
        amt = val
    } else {
        log_error!(
            log,
            "`oustanding balance` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
    }

    if let Some(val) = account.int_rt {
        int_rt = val
    } else {
        log_error!(
            log,
            "`interest rate` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
    }

    // Case: cf_date is null/empty
    if let Some(dt) = account.mat_dt {
        cf_dt = dt;
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
        cf_dt = *config_params.as_on_date();
    }

    // Case: Negative oustanding balance
    if amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.acc_no,
        );
        let negative_o_a_cf = new_cashflow(0.0, amt, &cf_dt);
        return vec![negative_o_a_cf];
    }

    // Case: Negative interest rate
    if int_rt < 0.0 {
        int_rt = int_rt.abs();
        log_error!(
            log,
            "Negative `interest rate` for account: `{}`.",
            account.acc_no,
        );
    } else if int_rt > 100.00 {
        log_error!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.acc_no,
        );
        int_rt = 0.0;
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < *config_params.as_on_date() {
        log_info!(log, "`overdue` for account: `{}`.", account.acc_no,);
        return vec![new_cashflow(0.0, amt, config_params.as_on_date())];
    }

    let (mut first_cf_date, is_ason_month_end) =
        get_month_end_for_ason(*config_params.as_on_date());
    if is_ason_month_end
        && (incr_dt_by_mon_presrv_eom_checked(*config_params.as_on_date(), 2).unwrap())
            >= account.mat_dt.unwrap()
    {
        let days = days_with_convn(
            *config_params.as_on_date(),
            account.mat_dt.unwrap(),
            config_params.convention(),
        )
        .expect("Failed to calculate days with convention");
        let int_amt = compute_int_amt(account.amt.unwrap(), account.int_rt.unwrap(), days);
        return vec![new_cashflow(int_amt, amt, &account.mat_dt.unwrap())];
    }
    let end_cf_date = account.mat_dt.unwrap();
    if is_ason_month_end {
        first_cf_date = incr_dt_by_mon_presrv_eom_checked(first_cf_date, 2).unwrap();
    }
    let mut cashflows = Vec::new();
    let days = days_with_convn(
        *config_params.as_on_date(),
        first_cf_date,
        config_params.convention(),
    )
    .expect("Failed to calculate days with convention");
    let i_a = compute_int_amt(account.amt.unwrap(), account.int_rt.unwrap(), days);
    let cf = new_cashflow(i_a, 0.0, &first_cf_date);
    cashflows.push(cf);
    let mut prev_cf_date = first_cf_date;
    while prev_cf_date <= end_cf_date {
        let mut cashflow_date =
            incr_dt_by_mon_presrv_eom_checked(prev_cf_date, 3).unwrap_or_else(|| {
                panic!(
                    "Failed to calculate the next cashflow date for account: {}",
                    account.acc_no
                )
            });
        cashflow_date = get_month_end_date(cashflow_date);
        if cashflow_date <= end_cf_date {
            let days = days_with_convn(prev_cf_date, cashflow_date, config_params.convention())
                .expect("Failed to calculate days with convention");
            let i_a = compute_int_amt(account.amt.unwrap(), account.int_rt.unwrap(), days);
            let cf = new_cashflow(i_a, 0.0, &cashflow_date);
            cashflows.push(cf);

            prev_cf_date = cashflow_date;
        } else {
            break;
        }
    }
    if prev_cf_date < end_cf_date {
        let days = days_with_convn(prev_cf_date, end_cf_date, config_params.convention())
            .expect("Failed to calculate days with convention");
        let i_a = compute_int_amt(account.amt.unwrap(), account.int_rt.unwrap(), days);
        let cf = new_cashflow(i_a, 0.0, &end_cf_date);
        cashflows.push(cf);
    }
    cashflows
        .last_mut()
        .expect("Cashflows matured without generating any cashflows.")
        .prin_amt = account.amt.unwrap();

    cashflows
}

fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = rbdate::timestamp(*d);
    cf
}

fn get_month_end_for_ason(date: NaiveDate) -> (NaiveDate, bool) {
    let days_in_month = get_days_from_month(date);

    if date.day() == days_in_month as u32 {
        // Given date is the end of the month, return the next month end
        (
            incr_dt_by_mon_presrv_eom_checked(date, 1).unwrap_or_else(|| {
                panic!("Failed to calculate the next month end for date: {}", date)
            }),
            true,
        )
    } else {
        // Given date is not the end of the month, return the current month end
        (get_month_end_date(date), false)
    }
}

fn compute_int_amt(o_a: f64, i_r: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (o_a * i_r * num_days as f64) / (days_in_yr * 100.0)
}
