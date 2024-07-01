use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use slog::Logger;
use statics::DEFAULT_FLOAT;

pub fn derive_cashflows(
    account: &mut InputAccount,
    int_amt: f64,
    prin_amt: f64,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let mat_dt: i64;
    let mut cf_dt: i64;
    let as_on_dt: i64 = timestamp(*config_params.as_on_date());

    if let Some(dt) = account.cf_date {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`due date` is not well-formatted for account: `{}`.",
            account.account_id,
        );
        cf_dt = if let Some(m_dt) = account.maturity_date {
            timestamp(m_dt)
        } else {
            log_error!(
                log,
                "`maturity date` is not well-formatted for account: `{}`.",
                account.account_id,
            );
            as_on_dt
        };
    }

    let int_rt: f64 = account.int_rate;

    // Case: Negative oustanding balance
    if account.cf_principal_amount <= 0.0 {
        log_error!(
            log,
            "Negative or Zero `outstanding balance` for account: `{}`.",
            account.account_id,
        );
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_error!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.account_id,
        );
    } else if int_rt > 100.00 {
        log_error!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.account_id,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.account_id,);
    }

    // Case: cf_dt > mat_dt
    if let Some(m_d) = account.maturity_date {
        mat_dt = timestamp(m_d);
        if cf_dt > mat_dt {
            cf_dt = mat_dt;
            log_info!(
                log,
                "`due date` is greater than `maturity date` for account: `{}`.",
                account.account_id
            );
        }
    } else {
        log_info!(
            log,
            "`maturity date` not found for account: `{}`.",
            account.account_id
        );
    }

    new_cashflow(int_amt, prin_amt, cf_dt)
}

pub fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}

pub fn generate_adjustment_cashflow(
    a_w_cf: &AccountWithCashflows,
    total_prin: &mut f64,
    cfs: &mut Vec<Cashflow>,
    log: &Logger,
) {
    let mut last_cf_date = a_w_cf.maturity_date;
    loop {
        if *total_prin > a_w_cf.outstanding_bal {
            if let Some(prev_cashflow) = cfs.pop() {
                *total_prin -= prev_cashflow.prin_amt;
                last_cf_date = prev_cashflow.date;
            } else {
                *total_prin += a_w_cf.outstanding_bal;
                let adj_cashflow =
                    new_cashflow(DEFAULT_FLOAT, a_w_cf.outstanding_bal, last_cf_date);
                cfs.push(adj_cashflow);
            }
        } else {
            let diff = a_w_cf.outstanding_bal - *total_prin;
            let adj_cashflow = new_cashflow(DEFAULT_FLOAT, diff, last_cf_date);
            cfs.push(adj_cashflow);
            *total_prin += diff;
        }
        if *total_prin == a_w_cf.outstanding_bal {
            break;
        }
    }
    if cfs.len() == 0 && a_w_cf.outstanding_bal == 0.0 {
        log_warn!(
            log,
            "Account: {} with outstanding: 0.0 but with cashflow principal: {} encountered!!",
            a_w_cf.account_id,
            a_w_cf.cf_principal_amount
        );
        let empty_cf = new_cashflow(DEFAULT_FLOAT, DEFAULT_FLOAT, last_cf_date);
        cfs.push(empty_cf);
    }
}
