use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use macros;
use rbdate::{timestamp, NaiveDate};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> Vec<Cashflow> {
    let prin_amt: f64 = account.book_value;
    let int_amt: f64 = account.int_amt;
    let int_rt: f64 = account.int_rate;

    if prin_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `principal amount` for account: `{}`.",
            account.deal_no,
        );
    }

    if int_amt <= 0.0 {
        log_warn!(
            log,
            "Negative or Zero `interest amount` for account: `{}`.",
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

    let cf_dt = if let Some(dt) = account.repo_mat_dt {
        dt
    } else {
        log_error!(
            log,
            "`maturity_date` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
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

    vec![new_cashflow(int_amt, prin_amt, timestamp(cf_dt))]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}
