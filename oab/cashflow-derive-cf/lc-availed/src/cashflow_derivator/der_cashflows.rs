use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use slog::Logger;
use statics::DEFAULT_FLOAT;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let as_on_dt: i64 = timestamp(*config_params.as_on_date());

    let int_amt: f64 = DEFAULT_FLOAT;
    let prin_amt: f64 = account.outstanding_bal;
    let mut cf_dt: i64 = timestamp(account.maturity_date);

    // Case: Negative oustanding balance
    if account.outstanding_bal <= 0.0 {
        log_error!(
            log,
            "Negative or Zero `outstanding balance` for account: `{}`.",
            account.account_id,
        );
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.account_id,);
    } else {
        cf_dt = timestamp(account.maturity_date);
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
