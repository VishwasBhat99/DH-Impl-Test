use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let mat_dt: i64;
    let mut cf_dt: i64;
    let as_on_dt: i64;
    let prin_amt: f64 = account.prin_amt;
    let int_amt: f64 = account.int_amt;
    let int_rt: f64 = account.intt_rate;

    if prin_amt <= 0.0 {
        log_debug!(
            log,
            "Negative or Zero `principal amount` for account: `{}`.",
            account.account_number,
        );
    }

    if int_amt <= 0.0 {
        log_debug!(
            log,
            "Negative or Zero `interest amount` for account: `{}`.",
            account.account_number,
        );
    }

    as_on_dt = timestamp(*config_params.as_on_date());

    if let Some(dt) = account.cf_dt {
        cf_dt = timestamp(dt);
    } else {
        log_debug!(
            log,
            "`cashflow date` is not well-formatted for account: `{}`.",
            account.account_number,
        );
        cf_dt = if let Some(m_dt) = account.mat_date {
            timestamp(m_dt)
        } else {
            log_error!(
                log,
                "`maturity date` is not well-formatted for account: `{}`.",
                account.account_number,
            );
            as_on_dt
        };
    }

    // Case: Negative oustanding balance
    if account.current_book_balance <= 0.0 {
        log_debug!(
            log,
            "Negative or Zero `outstanding balance` for account: `{}`.",
            account.account_number,
        );
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_debug!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.account_number,
        );
    } else if int_rt > 100.00 {
        log_debug!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.account_number,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_debug!(log, "`overdue` for account: `{}`.", account.account_number,);
    }

    // Case: cf_dt > mat_dt
    if let Some(m_d) = account.mat_date {
        mat_dt = timestamp(m_d);
        if cf_dt > mat_dt {
            cf_dt = mat_dt;
            log_debug!(
                log,
                "`cashflow date` is greater than `maturity date` for account: `{}`.",
                account.account_number
            );
        }
    } else {
        log_debug!(
            log,
            "`maturity date` not found for account: `{}`.",
            account.account_number
        );
    }

    new_cashflow(int_amt, prin_amt, cf_dt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
