use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::timestamp;
use slog::Logger;
use statics::*;

pub fn derive_cashflows(account: &mut InputAccount, log: &Logger) -> Vec<Cashflow> {
    let int_amt: f64 = DEFAULT_FLOAT;
    let prin_amt: f64 = account.non_index_cf_amt.abs();

    let cf_dt: i64 = if let Some(dt) = account.indexation_dt {
        timestamp(dt)
    } else {
        DEFAULT_INT
    };

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `0.0`.",
        account.trade_id,
        int_amt,
        prin_amt,
        cf_dt
    );

    vec![new_cashflow(int_amt, prin_amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
