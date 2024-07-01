use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use slog::Logger;

pub fn derive_cashflows(
    account: &InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_dt: i64;
    let as_on: i64;
    let int_rt = 0.0;

    as_on = timestamp(as_on_dt);

    // Case: cf_date is null/empty
    if let Some(dt) = account.cf_dt {
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
    if account.mtm_amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.deal_no,
        );
        let negative_o_a_cf = new_cashflow(account.cf_int_amt, account.mtm_amt, cf_dt);

        log_debug!(
            log,
            "Acount: `{}`, interest amount: `0.0`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}`.",
            account.deal_no,
            account.mtm_amt,
            date_from_timestamp(cf_dt),
            int_rt,
        );

        return vec![negative_o_a_cf];
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on {
        log_info!(log, "`overdue` for account: `{}`.", account.deal_no,);
    }

    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.deal_no,
        account.cf_int_amt,
        account.mtm_amt,
        date_from_timestamp(cf_dt),
    );

    vec![new_cashflow(account.cf_int_amt, account.mtm_amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}
