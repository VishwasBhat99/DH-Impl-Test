use crate::cashflow_derivator::account_reader::input_account::InputAccount;
use crate::cashflow_derivator::account_with_cashflows::Cashflow;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut prin_amt: f64 = 0.0;
    let mut int_amt: f64 = 0.0;

    let cashflow_date: i64 = if let Some(dt) = account.due_dt {
        rbdate::timestamp(dt)
    } else {
        rbdate::timestamp(*config_params.as_on_date())
    };
    if let Some(amt) = account.cf_amt {
        if account.component == "PRINCIPAL" {
            prin_amt = amt;
            if prin_amt <= 0.0 {
                log_error!(
                    log,
                    "Negative or Zero `principal amount` for account: `{}`.",
                    account.cust_no,
                );
            }
        } else if account.component == "INTEREST" {
            int_amt = amt;
            if int_amt <= 0.0 {
                log_error!(
                    log,
                    "Negative or Zero `interest amount` for account: `{}`.",
                    account.cust_no,
                );
            }
        } else if account.cf_amt.is_some() {
            log_error!(
                log,
                "`component` {} is not well-formatted for account: `{}`.",
                account.component,
                account.cust_no,
            );
        }
    } else {
        log_error!(
            log,
            "`CFAmt` is not well-formatted for account: `{}`.",
            account.cust_no,
        );
    }

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`",
        account.cust_no,
        int_amt,
        prin_amt,
        rbdate::date_from_timestamp(cashflow_date),
    );
    Ok(vec![new_cashflow(int_amt, prin_amt, cashflow_date)])
    //new_cashflow(int_amt, prin_amt, cashflow_date)
}

pub fn derive_adj_cashflows(
    account: &mut InputAccount,
    amount: &f64,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let cashflow_date: i64 = if let Some(dt) = account.mat_dt {
        rbdate::timestamp(dt)
    } else {
        rbdate::timestamp(*config_params.as_on_date())
    };

    log_debug!(
        log,
        "Adjustment Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.cust_no,
        0.0,
        amount,
        cashflow_date,
    );
    new_cashflow(0.0, *amount, cashflow_date)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
