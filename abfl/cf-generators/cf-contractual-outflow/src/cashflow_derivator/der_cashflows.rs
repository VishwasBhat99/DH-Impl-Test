use crate::cashflow_derivator::account_reader::input_account::InputAccount;
use crate::cashflow_derivator::account_with_cashflows::Cashflow;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use chrono::Duration;
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let prin_amt: f64 = account.incr_amt + account.projected_outflow;
    let int_amt: f64 = 0.0;
    let thirty_days_after_ason = config_params.as_on_date().to_owned() + Duration::days(30);
    let cashflow_date: i64 = rbdate::timestamp(thirty_days_after_ason);
    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`",
        account.lob,
        int_amt,
        prin_amt,
        rbdate::date_from_timestamp(cashflow_date),
    );
    Ok(vec![new_cashflow(int_amt, prin_amt, cashflow_date)])
    //new_cashflow(int_amt, prin_amt, cashflow_date)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
