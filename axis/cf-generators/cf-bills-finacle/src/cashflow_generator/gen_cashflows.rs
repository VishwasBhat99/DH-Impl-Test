use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let cf_dt: i64;
    let as_on_dt = timestamp(*config_params.as_on_date());
    let prin_val = account.bill_liab;
    //Interest is 0.

    if let Some(dt) = account.due_date {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`due date` is not well-formatted for account: `{}`.",
            account.account_number,
        );
        cf_dt = as_on_dt;
    }

    log_debug!(
        log,
        "account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}` ",
        account.account_number,
        0,
        prin_val,
        account.due_date
    );

    Ok(vec![new_cashflow(0.0, prin_val, cf_dt)])
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
