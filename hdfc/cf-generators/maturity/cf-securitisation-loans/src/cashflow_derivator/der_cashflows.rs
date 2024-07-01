use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let mat_dt: i64;
    let mut cf_dt: i64;
    let as_on_dt: i64 = timestamp(*config_params.as_on_date());
    let mut prin_amt: f64 = 0.0;
    let mut int_amt: f64 = 0.0;
    let mut int_rt: f64 = 0.0;

    if let Some(amt) = account.int_portion {
        int_amt = amt;
    } else {
        log_error!(
            log,
            "`interest portion` is not well-formatted for account: `{}`.",
            account.acc_id,
        );
    }

    if let Some(amt) = account.prin_pay {
        prin_amt = amt;
    } else {
        log_error!(
            log,
            "`principal payment` is not well-formatted for account: `{}`.",
            account.acc_id,
        );
    }

    if prin_amt <= 0.0 {
        log_error!(
            log,
            "Negative or zero `principal amount` for account: `{}`.",
            account.acc_id,
        );
    }

    if int_amt < 0.0 {
        log_error!(
            log,
            "Negative or zero `interest amount` for account: `{}`.",
            account.acc_id,
        );
    }

    if let Some(val) = account.int_rt {
        int_rt = val;
    } else {
        log_warn!(
            log,
            "`interest rate` is not well-formatted for account: `{}`.",
            account.acc_id,
        );
    }

    // Case: CF Date is null/empty
    if let Some(dt) = account.dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`cf date` is not well-formatted for account: `{}`.",
            account.acc_id,
        );
        cf_dt = as_on_dt;
    }

    // Case: Negative oustanding balance
    if let Some(bal) = account.pout_bal {
        if bal < 0.0 {
            log_warn!(
                log,
                "Negative `outstanding balance` for account: `{}`.",
                account.acc_id,
            );
        }
    } else {
        log_error!(
            log,
            "`outstanding balance` is not well-formatted for account: `{}`.",
            account.acc_id,
        );
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.acc_id,
        );
    } else if int_rt > 100.00 {
        log_warn!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.acc_id,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.acc_id,);
    }

    // Case: cf_dt > mat_dt
    if let Some(dt) = account.c_dt {
        mat_dt = timestamp(dt);
        if cf_dt > mat_dt {
            cf_dt = mat_dt;
            log_info!(
                log,
                "`cf date` is greater than `maturity date` for account: `{}`.",
                account.acc_id
            );
        }
    } else {
        log_info!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.acc_id
        );
    }

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.acc_id,
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
