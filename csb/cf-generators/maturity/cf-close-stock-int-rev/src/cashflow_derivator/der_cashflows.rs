use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp,increment_date_by_months};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let mat_dt: i64;
    let mut cf_dt: i64;
    let as_on_dt: i64;
    let mut prin_amt: f64 = 0.0;
    let mut int_amt: f64 = 0.0;
    let mut int_rt: f64 = 0.0;

    int_amt = account.int_amt;
    prin_amt = account.accrued_int;

    if prin_amt <= 0.0 {
        log_error!(
            log,
            "Negative or zero `principal amount` for account: `{}`.",
            account.inst_id,
        );
    }

    if int_amt <= 0.0 {
        log_error!(
            log,
            "Negative or zero `interest amount` for account: `{}`.",
            account.inst_id,
        );
    }
    int_rt = account.coupon;
    as_on_dt = timestamp(*config_params.as_on_date());

    // Case: CF Date is null/empty
    if let Some(dt) = account.nxt_coupon {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`cf date` is not well-formatted for account: `{}`.",
            account.inst_id,
        );
        cf_dt = timestamp(increment_date_by_months(date_from_timestamp(as_on_dt),180));
    }
    // Case: Negative oustanding balance
    if account.accrued_int < 0.0 {
        log_warn!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.inst_id,
        );
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.inst_id,
        );
    } else if int_rt > 100.00 {
        log_warn!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.inst_id,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_warn!(log, "`overdue` for account: `{}`.", account.inst_id,);
    }

    // Case: cf_dt > mat_dt
    if let Some(dt) = account.maturity_dt {
        mat_dt = timestamp(dt);
        if cf_dt > mat_dt {
            cf_dt = mat_dt;
            log_info!(
                log,
                "`cf date` is greater than `maturity date` for account: `{}`.",
                account.inst_id
            );
        }
    } else {
        log_info!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.inst_id
        );
    }

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.inst_id,
        int_amt,
        prin_amt,
        date_from_timestamp(cf_dt),
        int_rt,
    );

    new_cashflow(int_amt, prin_amt, cf_dt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
