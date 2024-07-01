use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_dt: i64;
    let as_on: i64;
    let mut amt: f64 = 0.0;

    if let Some(val) = account.undrawn_amount {
        amt = val
    } else {
        log_error!(
            log,
            "`oustanding balance` is not well-formatted for account: `{}`.",
            account.account_number,
        );
    }

    as_on = timestamp(as_on_dt);

    // Case: cf_date is null/empty
    if let Some(dt) = account.exp_end_date {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.account_number,
        );
        cf_dt = as_on;
    }

    // Case: Negative oustanding balance
    if amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.account_number,
        );
        let negative_o_a_cf = new_cashflow(0.0, amt, cf_dt);

        log_debug!(
            log,
            "Acount: `{}`, interest amount: `0.0`, principal amount: `{}`, cashflow date: `{:?}`.",
            account.account_number,
            amt,
            date_from_timestamp(cf_dt),
        );

        return vec![negative_o_a_cf];
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on {
        log_info!(log, "`overdue` for account: `{}`.", account.account_number,);
    }

    //here int amt zero because there are no interest cashflows for Unutilized portion.
    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.account_number,
        0.0,
        amt,
        date_from_timestamp(cf_dt),
    );

    vec![new_cashflow(0.0, amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}
