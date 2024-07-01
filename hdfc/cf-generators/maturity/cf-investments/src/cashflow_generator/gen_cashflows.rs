use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::cf_date_iterator::CFDateIterator;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, timestamp, NaiveDate};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
use statics::*;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut int_rt: f64 = DEFAULT_FLOAT;
    let mut amt: f64 = DEFAULT_FLOAT;

    if let Some(val) = account.os_cost_val {
        amt = val
    } else {
        log_error!(
            log,
            "`oustanding balance` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
    }

    if let Some(val) = account.int_rt {
        int_rt = val
    } else {
        log_error!(
            log,
            "`interest rate` is not well-formatted for account: `{}`.",
            account.deal_no,
        );
    }
    let conv = match account.int_basis {
        5 => Conventions::Thirtyby360,
        4 => Conventions::ACTby365,
        _ => Conventions::ACTby365,
    };

    let as_on_dt: i64 = timestamp(*config_params.as_on_date());

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

    let mat_dt = if let Some(dt) = account.mat_dt {
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
    if amt <= 0.0 {
        log_error!(
            log,
            "Negative or zero `outstanding balance` for account: `{}`.",
            account.deal_no,
        );
        let negative_o_a_cf = new_cashflow(0.0, amt, cf_dt);
        return vec![negative_o_a_cf];
    }

    // Case: Negative interest rate
    if int_rt < 0.0 {
        int_rt = int_rt.abs();
        log_error!(
            log,
            "Negative or zero `interest rate` for account: `{}`.",
            account.deal_no,
        );
    } else if int_rt > 100.00 {
        log_error!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.deal_no,
        );
        int_rt = 0.0;
    }

    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_error!(log, "`overdue` for account: `{}`.", account.deal_no,);
    }

    // Case: Interest payment frequency less than or equal to 0
    if account.int_pay_freq <= 0 {
        log_error!(
            log,
            "`interest_payment_frequency` is less than or equal to zero for account: `{}`.",
            account.deal_no
        );
        return vec![generate_bullet_cashflow(
            amt,
            as_on_dt,
            cf_dt,
            int_rt,
            conv,
            &account.deal_no,
            log,
        )];
    }

    if account.cmpnd_freq > 0 && account.cmpnd_freq < account.int_pay_freq {
        let compounding_interest_cfs =
            generate_compounding_interest_cfs(account, amt, as_on_dt, cf_dt, conv, int_rt, log);
        return compounding_interest_cfs;
    }

    let mut int_amt: f64 = 0.0;
    let i_a = compute_int_amt(as_on_dt, cf_dt, amt, int_rt, conv);
    if i_a.is_err() {
        log_error!(
            log,
            "Account: `{}`, error: `{}`",
            account.deal_no,
            i_a.err()
                .expect("Unexpected error occured while computing interest amount.")
        );
    } else {
        int_amt = i_a.expect("Unable to compute interest amount.");
    }

    log_debug!(
        log, 
        "account: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}` ",
        account.deal_no,
        int_amt,
        amt,
        date_from_timestamp(cf_dt),
        int_rt
        );

    generate_simple_interest_cfs(&account, amt, as_on_dt, cf_dt, conv, int_rt, log)
}

fn generate_simple_interest_cfs(
    a: &InputAccount,
    amt: f64,
    as_on_date: i64,
    maturity_date: i64,
    conv: Conventions,
    int_rt: f64,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();

    let default_principal_amount_for_cf = 0.0;
    let mut cf_start_date = as_on_date;

    let cf_date_iterator =
        CFDateIterator::new_from_account(a, &a.as_on_dt, &date_from_timestamp(maturity_date));

    for cashflow_date in cf_date_iterator {
        if let Ok(i_a) = compute_int_amt(cf_start_date, timestamp(cashflow_date), amt, int_rt, conv)
        {
            let cf = new_cashflow(
                i_a,
                default_principal_amount_for_cf,
                timestamp(cashflow_date),
            );
            cashflows.push(cf);
        }

        cf_start_date = timestamp(cashflow_date);
    }

    if let Some(cfs) = cashflows.last_mut() {
        cfs.prin_amt = amt
    } else {
        log_error!(
            log,
            "Cashflows vec has no value after account matured for account: `{}`.",
            a.deal_no
        )
    }

    cashflows
}

fn generate_compounding_interest_cfs(
    a: &InputAccount,
    amt: f64,
    as_on_date: i64,
    maturity_date: i64,
    conv: Conventions,
    int_rt: f64,
    log: &Logger,
) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();

    let start_date = if let Some(dt) = a.deal_dt {
        dt
    } else {
        log_error!(
            log,
            "`deal_date` is not well formatted for account: `{}`.",
            a.deal_no
        );
        date_from_timestamp(as_on_date)
    };
    let compound_interest_advance_by_months = a.cmpnd_freq as u16;
    let pay_cf_advance_by_months = a.int_pay_freq;
    let cf_date_iterator = CFDateIterator::new(
        pay_cf_advance_by_months,
        &start_date,
        &date_from_timestamp(maturity_date),
        &date_from_timestamp(as_on_date),
    );

    let mut last_compounding_start_date = start_date;

    for cashflow_date in cf_date_iterator {
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = amt;

        let mut should_break_loop = false;

        loop {
            if should_break_loop {
                break;
            }

            let mut next_cf_compounding_date = rbdate::increment_date_by_months_unchecked(
                last_compounding_start_date,
                compound_interest_advance_by_months,
            );

            if next_cf_compounding_date >= cashflow_date {
                next_cf_compounding_date = cashflow_date;
                should_break_loop = true;
            }

            if let Ok(new_i_amount) = compute_int_amt(
                timestamp(last_compounding_start_date),
                timestamp(next_cf_compounding_date),
                compounding_principal_amount,
                int_rt,
                conv,
            ) {
                interest_to_pay_amount = new_i_amount;
                compounding_principal_amount += new_i_amount;
                last_compounding_start_date = next_cf_compounding_date;
            }
        }

        let cf = new_cashflow(interest_to_pay_amount, 0.0, timestamp(cashflow_date));
        cashflows.push(cf);

        last_compounding_start_date = cashflow_date;
    }

    if let Some(cfs) = cashflows.last_mut() {
        cfs.prin_amt = amt;
    } else {
        log_error!(
            log,
            "Cashflows vec has no value after account matured for account: `{}`.",
            a.deal_no
        )
    };

    cashflows
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

fn generate_bullet_cashflow(
    prin_amt: f64,
    as_on_dt: i64,
    cf_dt: i64,
    int_rt: f64,
    conv: Conventions,
    acc_no: &String,
    log: &Logger,
) -> Cashflow {
    let mut int_amt: f64 = 0.0;
    let i_a = compute_int_amt(as_on_dt, cf_dt, prin_amt, int_rt, conv);
    if i_a.is_err() {
        log_error!(
            log,
            "Account: `{}`, error: {}.",
            acc_no,
            i_a.err()
                .expect("Unexpected error occured while computing interest amount.")
        );
    } else {
        int_amt = i_a.expect("Error while computing interest amount.");
    }
    new_cashflow(int_amt, prin_amt, cf_dt)
}
