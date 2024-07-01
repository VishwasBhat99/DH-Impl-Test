use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::cf_date_iterator::CFDateIterator;
use macros;
use rbdate;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use sdb_day_convention::conventions::Days;
use sdb_day_convention::days_with_convn;
use slog::Logger;
use statics::{DEFAULT_FLOAT, DEFAULT_INT};

pub fn generate_cashflows(
    as_on_date: &NaiveDate,
    account: &mut InputAccount,
    log: &Logger,
    day_convention: &Conventions,
) -> Result<Vec<Cashflow>, String> {
    if account.mat_dt <= *as_on_date {
        return Ok(vec![new_cashflow(0.0, account.bal_os, as_on_date)]);
    }
    // MARK: Bullet Cashflow Generation
    // If account start date is after account maturity data use account start date for cashflow generation
    if account.acc_open_dt > account.mat_dt {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.acc_no,
            "Account open date is after maturity date, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }
    if account.bal_os < DEFAULT_FLOAT {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.acc_no,
            "Account current balance is less than 0, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }
    if account.bal_os == 0.0 {
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }
    if account.int_rt <= DEFAULT_FLOAT || account.int_rt > 100.0 {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.acc_no,
            "Account interest rate is `INVALID`, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, &account.mat_dt);
        return Ok(vec![cf]);
    }
    if account.pay_freq <= DEFAULT_INT {
        if account.pay_freq < DEFAULT_INT {
            log_warn!(
                log,
                "For account: '{}'. Error: {}",
                account.acc_no,
                "Account interest payment frequency is INVALID, hence generated interest and principal cashflow at maturity";
            );
        }
        let cf = generate_bullet_cf(account, day_convention);
        return Ok(vec![cf]);
    }
    // MARK: Compounding Cashflow Generation
    if account.comp_freq > 0 && account.comp_freq < account.pay_freq {
        let compounding_interest_cfs =
            generate_compounding_interest_cfs(account, as_on_date, day_convention);
        return Ok(compounding_interest_cfs);
    }
    // MARK: Simple Interest Cashflow Generation
    let simple_interest_cfs = generate_simple_interest_cfs(account, as_on_date, day_convention);
    Ok(simple_interest_cfs)
}

fn generate_simple_interest_cfs(
    a: &InputAccount,
    as_on_date: &NaiveDate,
    convention: &Conventions,
) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();

    let o_a = a.bal_os;
    let i_r = a.int_rt;

    let default_principal_amount_for_cf = 0.0;
    let mut last_cf_date = get_last_pay_date(a.acc_open_dt, as_on_date, a.pay_freq);
    let cf_date_iterator = CFDateIterator::new_from_account(a, *as_on_date);

    for cashflow_date in cf_date_iterator {
        let days = days_with_convn(last_cf_date, cashflow_date, convention).unwrap();
        let i_a = interest_amount(o_a, i_r, days);
        let cf = new_cashflow(i_a, default_principal_amount_for_cf, &cashflow_date);
        cashflows.push(cf);

        last_cf_date = cashflow_date;
    }

    cashflows
        .last_mut()
        .expect("Cashflows matured without generating any cashflows.")
        .prin_amt = o_a;

    cashflows
}

fn generate_compounding_interest_cfs(
    a: &InputAccount,
    as_on_date: &NaiveDate,
    convention: &Conventions,
) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();
    let start_date = a.acc_open_dt;
    let mat_dt = a.mat_dt;
    let compound_interest_advance_by_months = a.comp_freq as usize;
    let pay_cf_advance_by_months = a.pay_freq;
    let i_r = a.int_rt;

    let cf_date_iterator =
        CFDateIterator::new(pay_cf_advance_by_months, &start_date, &mat_dt, *as_on_date);

    let mut last_pay_date = get_last_pay_date(start_date, as_on_date, a.pay_freq);
    for cashflow_date in cf_date_iterator {
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = a.bal_os;
        let mut should_break_loop = false;
        loop {
            if should_break_loop {
                break;
            }
            let mut next_cf_compounding_date = rbdate::incr_dt_by_mon_presrv_eom(
                last_pay_date,
                compound_interest_advance_by_months,
            )
            .unwrap();

            if next_cf_compounding_date >= cashflow_date {
                next_cf_compounding_date = cashflow_date;
                should_break_loop = true;
            }

            let days =
                days_with_convn(last_pay_date, next_cf_compounding_date, convention).unwrap();
            let new_i_amount = interest_amount(compounding_principal_amount, i_r, days);

            interest_to_pay_amount += new_i_amount;
            compounding_principal_amount += new_i_amount;
            last_pay_date = next_cf_compounding_date;
        }

        let cf = new_cashflow(interest_to_pay_amount, 0.0, &cashflow_date);
        cashflows.push(cf);
    }

    cashflows
        .last_mut()
        .expect("Cashflows vec has no value after account matured.")
        .prin_amt = a.bal_os;

    cashflows
}

fn generate_bullet_cf(a: &InputAccount, convention: &Conventions) -> Cashflow {
    let start_date = a.acc_open_dt;
    let mat_date = a.mat_dt;
    if a.comp_freq <= 0 {
        let days = days_with_convn(start_date, a.mat_dt, convention).unwrap();
        let i_a = interest_amount(a.bal_os, a.int_rt, days);
        let cf = new_cashflow(i_a, a.bal_os, &mat_date);

        cf
    } else {
        let mut last_compounding_start_date = start_date;
        let compound_interest_advance_by_months = a.comp_freq as usize;
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = a.bal_os;
        loop {
            let mut next_cf_compounding_date = rbdate::incr_dt_by_mon_presrv_eom(
                last_compounding_start_date,
                compound_interest_advance_by_months,
            )
            .unwrap();
            if next_cf_compounding_date >= a.mat_dt {
                next_cf_compounding_date = a.mat_dt;
            }
            let days = days_with_convn(
                last_compounding_start_date,
                next_cf_compounding_date,
                convention,
            )
            .unwrap();
            let new_i_amount = interest_amount(compounding_principal_amount, a.int_rt, days);
            interest_to_pay_amount += new_i_amount;
            compounding_principal_amount += new_i_amount;
            last_compounding_start_date = next_cf_compounding_date;
            if last_compounding_start_date >= a.mat_dt {
                break;
            }
        }
        let cf = new_cashflow(interest_to_pay_amount, a.bal_os, &a.mat_dt);

        cf
    }
}

fn generate_matured_cf(a: &InputAccount, date: &NaiveDate) -> Cashflow {
    new_cashflow(DEFAULT_FLOAT, a.bal_os, date)
}

fn interest_amount(o_a: f64, i_r: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (o_a * i_r * num_days as f64) / (days_in_yr * 100.0)
}

fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = rbdate::timestamp(*d);

    cf
}

fn get_last_pay_date(acc_st_dt: NaiveDate, as_on_date: &NaiveDate, pay_freq: i64) -> NaiveDate {
    let mut next_date;
    let mut st_dt = acc_st_dt;
    let mut last_pay_dt = acc_st_dt;
    loop {
        next_date = rbdate::incr_dt_by_mon_presrv_eom(st_dt, pay_freq as usize).unwrap();
        if next_date > *as_on_date {
            break;
        }
        st_dt = next_date;
        last_pay_dt = next_date;
    }

    last_pay_dt
}