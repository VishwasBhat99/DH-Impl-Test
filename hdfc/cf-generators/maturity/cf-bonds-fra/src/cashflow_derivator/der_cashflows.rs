use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{date_from_timestamp, timestamp, NaiveDate};
use slog::Logger;
use statics::*;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    is_nd: bool,
    log: &Logger,
) -> Vec<Cashflow> {

    let prin_amt: f64 = account.fwdmtm_in_inr;
    let cashflow_date: i64 = if let Some(dt) = account.settlement_dt {
        timestamp(dt)
    } else {
        timestamp(as_on_dt)
    };
    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`.",
        account.trade_id,
        DEFAULT_FLOAT,
        prin_amt,
        date_from_timestamp(cashflow_date)
    );
    vec![new_cashflow(DEFAULT_FLOAT, prin_amt.abs(), cashflow_date)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
