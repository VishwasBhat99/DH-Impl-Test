use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::date_from_timestamp;
use slog::Logger;
use statics::*;

pub fn derive_cashflows(
    account: &InputAccount,
    cf_amt: f64,
    cf_dt: i64,
    log: &Logger,
) -> Vec<Cashflow> {
    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.cf_sub_type,
        DEFAULT_FLOAT,
        cf_amt,
        date_from_timestamp(cf_dt),
    );

    vec![new_cashflow(DEFAULT_FLOAT, cf_amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}
