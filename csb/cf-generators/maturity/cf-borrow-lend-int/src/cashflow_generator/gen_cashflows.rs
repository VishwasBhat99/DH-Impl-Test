use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp, NaiveDate};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_strt_dt: i64;
    let cf_end_dt: i64;
    let as_on_dt: i64;
    let mut int_rt: f64 = account.int_rt;

    as_on_dt = timestamp(*config_params.as_on_date());
    let as_on_date_naive = *config_params.as_on_date();
    let borrowing_dt = if let Some(dt) = account.borrowing_dt {
        dt
    } else {
        log_error!(
            log,
            "`borrowing_date` is not well-formatted for account: `{}`.",
            account.deal_num,
        );
        as_on_date_naive
    };
    let residual_days: i64 = get_residual_days(as_on_date_naive, borrowing_dt);
    let amt: f64 = account.spread.abs();
    if let Some(dt) = account.maturity_dt {
        cf_end_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.deal_num
        );
        cf_end_dt = as_on_dt;
    }

    // Case: Account Openining Date is null/empty
    if let Some(dt) = account.borrowing_dt {
        cf_strt_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`borrowing start date` is not well-formatted for account: `{}`.",
            account.deal_num
        );
        cf_strt_dt = as_on_dt;
    }

    // Case: Negative oustanding balance
    if amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.deal_num,
        );
        let negative_o_a_cf = new_cashflow(0.0, amt, cf_end_dt);
        return vec![negative_o_a_cf];
    }

    // Case: Negative interest rate
    if int_rt < 0.0 {
        int_rt = int_rt.abs();
        log_error!(
            log,
            "Negative `interest rate` for account: `{}`.",
            account.deal_num,
        );
    } else if int_rt > 100.00 {
        log_error!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.deal_num,
        );
        int_rt = 0.0;
    }

    // Case: cf_dt < as_on_dt
    if cf_end_dt < as_on_dt {
        log_error!(log, "`overdue` for account: `{}`.", account.deal_num,);
    }

    let int_amt: f64 = match compute_int_amt(
        cf_strt_dt,
        cf_end_dt,
        amt,
        int_rt,
        *config_params.convention(),
    ) {
        Ok(amt) => amt,
        Err(error) => {
            log_error!(log, "{}", error);
            0.0
        }
    };

    log_debug!(
        log,
        "account: `{}`, interest amount: `{}`, principal amount: `{}`, \
         cashflow date: `{:?}`, interest rate: `{}` ",
        account.deal_num,
        int_amt,
        amt,
        account.maturity_dt,
        int_rt
    );

    vec![new_cashflow(int_amt, amt, cf_end_dt)]
}

pub fn compute_int_amt(
    st_dt: i64,
    end_dt: i64,
    ost: f64,
    int_rt: f64,
    conv: Conventions,
) -> Result<f64, String> {
    let st_dt = date_from_timestamp(st_dt);
    let end_dt = date_from_timestamp(end_dt);
    let int_basis = match days_with_convn(st_dt, end_dt, &conv) {
        Ok(days) => days,
        Err(error) => return Err(format!("Unable to get `{:?}` convention : {}", conv, error)),
    };
    let int_amt =
        (ost * int_basis.days_btw_dts as f64 * int_rt) / (int_basis.day_in_yr as f64 * 100.00);
    Ok(int_amt)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}

fn get_residual_days(as_on_dt: NaiveDate, borrowing: NaiveDate) -> i64 {
    rbdate::num_days_start_to_end(borrowing, as_on_dt) + 1
}
