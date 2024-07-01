use super::ConfigurationParameters;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::timestamp;
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let int_rt: f64 = account.roi;
    let prin_amt: f64 = account.prin_amt;
    let as_on_dt: i64 = timestamp(*config_params.as_on_date());

    if prin_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `principal amount` for account: `{}`.",
            account.deal_id,
        );
    }

    let int_amt: f64 = account.int_amt;

    if int_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest amount` for account: `{}`.",
            account.deal_id,
        );
    }

    let cf_dt: i64 = if let Some(dt) = account.val_date {
        timestamp(dt)
    } else {
        log_error!(
            log,
            "`value date` is not well-formatted for account: `{}`.",
            account.deal_id,
        );
        as_on_dt
    };

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

    new_cashflow(int_amt, prin_amt, cf_dt)
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
