use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use sdb_day_convention::{days_with_convn, Conventions, Days};
use slog::Logger;

pub fn derive_cashflows(
    account: &InputAccount,
    as_on_dt: NaiveDate,
    convention: Conventions,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_dt: i64;
    let as_on: i64;
    let int_rt = 0.0;

    let amt = account.settle_amt_1st_leg;
    as_on = timestamp(as_on_dt);

    // Case: cf_date is null/empty
    if let Some(dt) = account.value_dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`value date` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
        cf_dt = as_on;
    }

    // Case: Negative oustanding balance
    if amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.deal_no,
        );
        let negative_o_a_cf = new_cashflow(account.accrued_interest, amt, cf_dt);

        log_debug!(
            log,
            "Acount: `{}`, interest amount: `0.0`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}`.",
            account.deal_no,
            amt,
            date_from_timestamp(cf_dt),
            int_rt,
        );

        return vec![negative_o_a_cf];
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on {
        log_info!(log, "`overdue` for account: `{}`.", account.deal_no,);
        return vec![new_cashflow(0.0, amt, as_on)];
    }

    let days = days_with_convn(as_on_dt, account.value_dt.unwrap(), &convention)
        .expect("Failed to calculate days with convention");
    let int_amt = calculate_interest_amount(amt, account.repo_rate, days);

    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.deal_no,
        int_amt,
        amt,
        date_from_timestamp(cf_dt),
    );
    vec![new_cashflow(int_amt, amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}

fn calculate_interest_amount(original_balance: f64, interest_rate: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_year = days.day_in_yr as f64;
    (original_balance * interest_rate * num_days as f64) / (days_in_year * 100.0)
}
