use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Vec<Cashflow> {
    let cf_dt: i64;
    let as_on_dt: i64;
    let mut int_rt: f64 = 0.0;
    let mut amt: f64 = 0.0;
    let lst_int_pd_dt: i64;

    if let Some(val) = account.amt {
        amt = val
    } else {
        log_error!(
            log,
            "`oustanding balance` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
    }

    if let Some(val) = account.int_rt {
        int_rt = val
    } else {
        log_error!(
            log,
            "`interest rate` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
    }

    as_on_dt = timestamp(*config_params.as_on_date());

    if let Some(dt) = account.st_dt {
        lst_int_pd_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`start date` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
        lst_int_pd_dt = as_on_dt;
    }

    // Case: cf_date is null/empty
    if let Some(dt) = account.mat_dt {
        cf_dt = timestamp(dt);
    } else {
        log_error!(
            log,
            "`maturity date` is not well-formatted for account: `{}`.",
            account.acc_no,
        );
        cf_dt = as_on_dt;
    }

    // Case: Negative oustanding balance
    if amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.acc_no,
        );
        let negative_o_a_cf = new_cashflow(0.0, amt, cf_dt);

        log_debug!(
            log,
            "Acount: `{}`, interest amount: `0.0`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}`.",
            account.acc_no,
            amt,
            date_from_timestamp(cf_dt),
            int_rt,
        );

        return vec![negative_o_a_cf];
    }

    // Case: Negative interest rate
    if int_rt < 0.0 {
        int_rt = int_rt.abs();
        log_error!(
            log,
            "Negative `interest rate` for account: `{}`.",
            account.acc_no,
        );
    } else if int_rt > 100.00 {
        log_error!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.acc_no,
        );
        int_rt = 0.0;
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.acc_no,);
    }

    let mut int_amt: f64 = 0.0;
    if let Ok(i_a) = compute_int_amt(
        lst_int_pd_dt,
        cf_dt,
        amt,
        int_rt,
        *config_params.convention(),
    ) {
        int_amt = i_a;
    }

    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}`.",
        account.acc_no,
        int_amt,
        amt,
        date_from_timestamp(cf_dt),
        int_rt,
    );

    vec![new_cashflow(int_amt, amt, cf_dt)]
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
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
