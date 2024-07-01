use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::timestamp;
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    _config_params: &ConfigurationParameters,
    log: &Logger,
) -> Cashflow {
    let mut prin_amt: f64 = 0.0;
    let mut int_amt: f64 = 0.0;
    let cf_dt = timestamp(account.cf_date);
    if account.cf_type.to_uppercase() == "PRINCIPAL" {
        prin_amt = account.cf_amount;
        if prin_amt <= 0.0 {
            log_error!(
                log,
                "Negative or Zero `principal amount` for account: `{}`.",
                account.deal_number,
            );
        }
    } else if account.cf_type.to_uppercase() == "INT" {
        int_amt = account.cf_amount;
        if int_amt <= 0.0 {
            log_error!(
                log,
                "Negative or Zero `interest amount` for account: `{}`.",
                account.deal_number,
            );
        }
    } else {
        log_error!(
            log,
            "`cf_type` is not well-formatted for account: `{}`.",
            account.deal_number,
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
