use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{date_from_timestamp, timestamp, NaiveDate};
use slog::Logger;
use statics::*;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut prin_amt: f64 = account.flowamount;
    prin_amt = prin_amt.abs();

    let cashflow_date: i64 = if let Some(dt) = account.cf_dt {
        timestamp(dt)
    } else {
        timestamp(as_on_dt)
    };

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.trade_id,
        DEFAULT_FLOAT,
        prin_amt,
        date_from_timestamp(cashflow_date),
        DEFAULT_FLOAT
    );
    vec![new_cashflow(DEFAULT_FLOAT, prin_amt, cashflow_date)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
