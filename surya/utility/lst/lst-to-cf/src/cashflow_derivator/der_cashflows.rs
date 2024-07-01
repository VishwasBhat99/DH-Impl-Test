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
    let cf_dt: i64;
    let as_on_dt: i64;
    let int_amt: f64 = 0.0;
    let prin_amt = account.amount;

    as_on_dt = timestamp(*config_params.as_on_date());

    cf_dt = if let Some(m_dt) = account.mat_dt {
        timestamp(m_dt)
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.acc_num,
        );
        as_on_dt
    };

    new_cashflow(int_amt, prin_amt, cf_dt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}
