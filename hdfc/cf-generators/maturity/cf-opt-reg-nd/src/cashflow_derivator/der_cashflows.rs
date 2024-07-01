use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{date_from_timestamp, timestamp, NaiveDate};
use slog::Logger;
use statics::*;

use super::req_fields;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    required_fields: req_fields::ReqFieldNames,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_amt_fld = required_fields.cf_amt.trim();
    let cf_date_fld = required_fields.cf_date.trim();

    info!(log, "The value of cf_amt in req field file: {}", cf_amt_fld);
    info!(log, "The value of cf_date in req field file: {}", cf_date_fld);

    let mut prin_amt = DEFAULT_FLOAT;
    let mut cf_dt = DEFAULT_INT;

    match cf_amt_fld {
        "settlement_amount" => {
            prin_amt = account.settlement_amount.parse().unwrap_or(DEFAULT_FLOAT);
        },
        "call_amt" => {
            prin_amt = account.call_amt;
        },
        "put_amt" => {
            prin_amt = account.put_amt;
        },
        "prem_amt" => {
            prin_amt = account.prem_amt;
        },
        "setld_prem_amt" => {
            prin_amt = account.setld_prem_amt
        },
        "unsetld_prem_amt" => {
            prin_amt = account.unsetld_prem_amt;
        },
        _ => {
            prin_amt = account.fwdmtm_settle_ccy;
        }
    }

    prin_amt = prin_amt.abs();

    match cf_date_fld {
        "trade_dt" => {
            cf_dt = if let Some(dt) = account.trade_dt {
                timestamp(dt)
            } else {
                timestamp(as_on_dt)
            };
        },
        "st_dt" => {
            cf_dt = if let Some(dt) = account.st_dt {
                timestamp(dt)
            } else {
                timestamp(as_on_dt)
            };
        },
        "ex_dt" => {
            cf_dt = if let Some(dt) = account.ex_dt {
                timestamp(dt)
            } else {
                timestamp(as_on_dt)
            };
        },
        "prem_setld_dt" => {
            cf_dt = if let Some(dt) = account.prem_setld_dt {
                timestamp(dt)
            } else {
                timestamp(as_on_dt)
            };
        },
        "settlementdate" => {
            let dt = NaiveDate::parse_from_str(&account.settlementdate, "%d-%m-%Y");
    
            if dt.is_err() {
                cf_dt = timestamp(as_on_dt)
            } else {
                cf_dt = timestamp(dt.unwrap());
            }
        },
        _ => {
            cf_dt = if let Some(dt) = account.del_dt {
                timestamp(dt)
            } else {
                timestamp(as_on_dt)
            };
        }
    }

    info!(log, "The value taken for cf_amt in cashflow: {}", prin_amt);
    info!(log, "The value taken for cf_date in cashflow: {}", cf_dt);

    log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.trade_id,
        DEFAULT_FLOAT,
        prin_amt,
        date_from_timestamp(cf_dt),
        DEFAULT_FLOAT
    );
    vec![new_cashflow(DEFAULT_FLOAT, prin_amt, cf_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
