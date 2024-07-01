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
    let as_on_dt: i64 = timestamp(*config_params.as_on_date());
    let mut prin_amt: f64 = 0.0;
    let mut int_amt: f64 = 0.0;
    let mut int_rt: f64 = 0.0;

    if let Some(amt) = account.cf_amt {
        if account.component == "PRINCIPAL" {
            prin_amt = amt;
            if prin_amt <= 0.0 {
                log_error!(
                    log,
                    "Negative or Zero `principal amount` for account: `{}`.",
                    account.reference,
                );
            }
        } else if account.component == "MAIN_INT" {
            int_amt = amt;
            if int_amt <= 0.0 {
                log_error!(
                    log,
                    "Negative or Zero `interest amount` for account: `{}`.",
                    account.reference,
                );
            }
        } else if account.cf_amt.is_some() {
            log_error!(
                log,
                "`component` is not well-formatted for account: `{}`.",
                account.reference,
            );
        }
    } else {
        log_error!(
            log,
            "`CFAmt` is not well-formatted for account: `{}`.",
            account.reference,
        );
    }

    if let Some(val) = account.norm_int_rt {
        int_rt = val;
    } else {
        log_error!(
            log,
            "`interest rate` is not well-formatted for account: `{}`.",
            account.reference,
        );
    }

    if let Some(dt) = account.due_dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`due date` is not well-formatted for account: `{}`.",
            account.reference,
        );
        cf_dt = if let Some(m_dt) = account.mat_dt {
            timestamp(m_dt)
        } else {
            log_error!(
                log,
                "`maturity date` is not well-formatted for account: `{}`.",
                account.reference,
            );
            as_on_dt
        };
    }

    // Case: Negative oustanding balance
    if account.prin_ost_bal <= 0.0 {
        log_error!(
            log,
            "Negative or Zero `outstanding balance` for account: `{}`.",
            account.reference,
        );
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_error!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.reference,
        );
    } else if int_rt > 100.00 {
        log_error!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.reference,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.reference,);
    }

    // Case: cf_dt > mat_dt
    if let Some(m_d) = account.mat_dt {
        mat_dt = timestamp(m_d);
        if cf_dt > mat_dt {
            cf_dt = mat_dt;
            log_info!(
                log,
                "`due date` is greater than `maturity date` for account: `{}`.",
                account.reference
            );
        }
    } else {
        log_info!(
            log,
            "`maturity date` not found for account: `{}`.",
            account.reference
        );
    }

    new_cashflow(int_amt, prin_amt, cf_dt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
