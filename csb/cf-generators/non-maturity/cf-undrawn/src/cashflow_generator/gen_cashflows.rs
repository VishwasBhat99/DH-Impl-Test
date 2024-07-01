use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut prin_amt: f64 = 0.0;
    let int_amt: f64 = 0.0;
    let cf_dt: i64;

    let as_on_dt = timestamp(*config_params.as_on_date());
    if config_params.amt_field_for().to_uppercase() == "NSFR" {
        prin_amt = account.care_funded;
    } else if config_params.amt_field_for().to_uppercase() == "LCR" {
        prin_amt = account.ccod_undrawn_lcr;
    }
    else {
        info!(log, "prinicipal amount for cashflow is taken as '0.0' as 'NIL' is given as amt-field-value.");
    }
    if let Some(dt) = account.maturity_dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
        cf_dt = as_on_dt;
    }
    Ok(vec![new_cashflow(int_amt, prin_amt, cf_dt)])
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
