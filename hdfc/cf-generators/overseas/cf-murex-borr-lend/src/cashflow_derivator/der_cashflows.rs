use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::timestamp;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;

pub fn derive_cashflows(
    account: &mut InputAccount,
    log: &Logger,
    as_on_date: &NaiveDate,
) -> Vec<Cashflow> {
    let cf_dt: i64;
    let as_on_dt: i64;
    let prin_amt: f64;
    let int_amt: f64;
    let int_rt: f64 = DEFAULT_FLOAT;

    prin_amt = account.prin_amt;

    if prin_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `principal amount` for account: `{}`.",
            account.deal_id,
        );
    }

    int_amt = account.int_amt;

    if int_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest amount` for account: `{}`.",
            account.deal_id,
        );
    }

    as_on_dt = timestamp(*as_on_date);

    if let Some(dt) = account.mat_dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.deal_id,
        );
        cf_dt = as_on_dt;
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.deal_id,
        );
    } else if int_rt > 100.00 {
        log_warn!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.deal_id,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.deal_id,);
    }

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.deal_id,
        int_amt,
        prin_amt,
        cf_dt,
        int_rt
    );

    vec![new_cashflow(int_amt, prin_amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
