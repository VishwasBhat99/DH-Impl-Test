use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let cf_dt: i64 = timestamp(account.mat_dt);
    let as_on_dt: i64;
    let int_rt: f64 = 0.0;
    let amt: f64 = account.tot_mk_val_of_col;
    as_on_dt = timestamp(*config_params.as_on_date());

    // Case: Negative oustanding balance
    if amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.acc_id,
        );
        let negative_o_a_cf = new_cashflow(0.0, amt, cf_dt);
        return Ok(vec![negative_o_a_cf]);
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_error!(log, "`overdue` for account: `{}`.", account.acc_id,);
    }

    let int_amt: f64 = 0.0;

    log_debug!(
        log, 
            "account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}` ",
            account.acc_id,
            int_amt,
            amt,
            account.mat_dt,int_rt
    );

    Ok(vec![new_cashflow(int_amt, amt, cf_dt)])
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}
