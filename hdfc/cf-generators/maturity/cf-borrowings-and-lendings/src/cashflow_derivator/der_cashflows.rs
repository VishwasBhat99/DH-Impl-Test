use super::cf_date_picker::CashflowDates;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{timestamp, NaiveDate};
use slog::Logger;
use statics::*;
use std::collections::HashMap;

pub fn derive_cashflows(
    account: &mut InputAccount,
    mut cf_dates: HashMap<String, CashflowDates>,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let prin_amt: f64;
    let int_amt: f64;
    let mut int_rt: f64 = DEFAULT_FLOAT;
    let mut cashflow_date: i64 = DEFAULT_INT;

    prin_amt = if let Some(amt) = account.orgballcy {
        amt
    } else {
        log_error!(
            log,
            "`principal` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
        DEFAULT_FLOAT
    };

    if prin_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `principal amount` for account: `{}`.",
            account.deal_no,
        );
    }

    int_amt = if let Some(amt) = account.int_amt {
        amt
    } else {
        log_error!(
            log,
            "`interest amount` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
        DEFAULT_FLOAT
    };

    if int_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest amount` for account: `{}`.",
            account.deal_no,
        );
    }

    if let Some(val) = account.couprt {
        int_rt = val;
    } else {
        log_warn!(
            log,
            "`interest rate` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
    }

    // Case: Negative interest rate
    if int_rt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.deal_no,
        );
    } else if int_rt > 100.00 {
        log_warn!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.deal_no,
        );
    }

    if cf_dates.contains_key(&account.deal_no) {
        if let Some(dates) = cf_dates.get_mut(&account.deal_no) {
            if let Some(dt) = account.mat_dt {
                dates.mat_dt = dt;
            }
            let cf_dt = if dates.call_dt <= dates.put_dt
                && dates.call_dt < dates.mat_dt
                && dates.call_dt > as_on_dt
            {
                dates.call_dt
            } else if dates.put_dt <= dates.call_dt
                && dates.put_dt < dates.mat_dt
                && dates.put_dt > as_on_dt
            {
                dates.put_dt
            } else {
                dates.mat_dt
            };
            log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.deal_no,
        int_amt,
        prin_amt,
        cf_dt,
        int_rt
    );
            cashflow_date = timestamp(cf_dt);
        }
        return vec![new_cashflow(int_amt, prin_amt, cashflow_date)];
    } else {
        let cf_dt = if let Some(dt) = account.mat_dt {
            dt
        } else {
            as_on_dt
        };

        log_debug!(
        log,
        "Account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{}`, interest rate: `{}`.",
        account.deal_no,
        int_amt,
        prin_amt,
        cf_dt,
        int_rt
    );

        cashflow_date = timestamp(cf_dt);

        return vec![new_cashflow(int_amt, prin_amt, cashflow_date)];
    }
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
