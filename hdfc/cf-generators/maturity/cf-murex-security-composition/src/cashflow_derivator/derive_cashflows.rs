use cashflow_derivator::{
    account_reader::input_account::InputAccount, account_with_cashflows::Cashflow,
};
use chrono::NaiveDate;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use slog::Logger;

pub fn derive_cashflows(account: &InputAccount, as_on_dt: NaiveDate, log: &Logger) -> Cashflow {
    let int_rt = 0.0;
    let as_on_dt: i64 = timestamp(as_on_dt);

    let call_dt: i64 = if let Some(dt) = account.call_dt {
        timestamp(dt)
    } else {
        timestamp(NaiveDate::from_ymd(2099, 01, 01))
    };

    let put_dt: i64 = if let Some(dt) = account.put_dt {
        timestamp(dt)
    } else {
        timestamp(NaiveDate::from_ymd(2099, 01, 01))
    };

    let mat_dt = if let Some(dt) = account.cf_dt {
        timestamp(dt)
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
        as_on_dt
    };

    let cf_dt: i64 = if call_dt <= put_dt && call_dt < mat_dt && call_dt > as_on_dt {
        call_dt
    } else if put_dt <= call_dt && put_dt < mat_dt && put_dt > as_on_dt {
        put_dt
    } else {
        mat_dt
    };

    // Case: Negative oustanding balance
    if account.prin_amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.deal_no,
        );
        let negative_o_a_cf = new_cashflow(account.int_amt, account.prin_amt, cf_dt);

        log_debug!(
            log,
            "Acount: `{}`, interest amount: `0.0`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}`.",
            account.deal_no,
            account.prin_amt,
            date_from_timestamp(cf_dt),
            int_rt,
        );

        return negative_o_a_cf;
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.deal_no,);
    }

    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.deal_no,
        account.int_amt,
        account.prin_amt,
        date_from_timestamp(cf_dt),
    );

    new_cashflow(account.int_amt, account.prin_amt, cf_dt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}
