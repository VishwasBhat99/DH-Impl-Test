use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{date_from_timestamp, timestamp, NaiveDate};
use slog::Logger;
use statics::*;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_dt: i64;
    let as_on_dt: i64 = timestamp(as_on_dt);
    let prin_amt: f64;
    let int_amt: f64 = DEFAULT_FLOAT;

    prin_amt = if let Some(amt) = account.frwrd_delta {
        amt
    } else {
        log_error!(
            log,
            "`forward_delta` is not well-formatted for account: `{}`.",
            account.trade_id,
        );
        DEFAULT_FLOAT
    };

    if prin_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `principal amount` for account: `{}`.",
            account.trade_id,
        );
    }

    if let Some(dt) = account.delivery_dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`delivery date` is not well-formatted for account: `{}`.",
            account.trade_id,
        );
        cf_dt = as_on_dt;
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.trade_id,);
    }

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`.",
        account.trade_id,
        int_amt,
        prin_amt,
        date_from_timestamp(cf_dt),
    );

    vec![new_cashflow(int_amt, prin_amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
