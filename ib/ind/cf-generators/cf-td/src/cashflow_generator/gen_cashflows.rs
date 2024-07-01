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
    if account.mat_dt.unwrap_or(*as_on_date) <= *as_on_date {
        return Ok(vec![new_cashflow(0.0, account.curr_bal, as_on_date)]);
    }
    // MARK: Bullet Cashflow Generation
    // If account start date is after account maturity data use account start date for cashflow generation
    if account.acct_open_dt > account.mat_dt {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.customer_no,
            "Account open date is after maturity date, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }
    if account.curr_bal < DEFAULT_FLOAT {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.customer_no,
            "Account current balance is less than 0, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }
    if account.curr_bal == 0.0 {
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }
    if account.int_rate <= DEFAULT_FLOAT || account.int_rate > 100.0 {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.customer_no,
            "Account interest rate is `INVALID`, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, &account.mat_dt.unwrap());
        return Ok(vec![cf]);
    }
    if account.int_repay_freq <= DEFAULT_INT {
        if account.int_repay_freq < DEFAULT_INT {
            log_warn!(
                log,
                "For account: '{}'. Error: {}",
                account.customer_no,
                "Account interest payment frequency is INVALID, hence generated interest and principal cashflow at maturity";
            );
        }
        let cf = generate_bullet_cf(account, day_convention);
        return Ok(vec![cf]);
    }
    // MARK: Compounding Cashflow Generation
    if account.term_int_comp_freq > 0 && account.term_int_comp_freq < account.int_repay_freq {
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

    let o_a = a.curr_bal;
    let i_r = a.int_rate;

    let default_principal_amount_for_cf = 0.0;
    let mut last_cf_date = get_last_pay_date(a.acct_open_dt.unwrap(), as_on_date, a.int_repay_freq);
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
    let start_date = a.acct_open_dt.unwrap();
    let mat_dt = a.mat_dt;
    let compound_interest_advance_by_months = a.term_int_comp_freq as usize;
    let pay_cf_advance_by_months = a.int_repay_freq;
    let i_r = a.int_rate;

    let cf_date_iterator = CFDateIterator::new(
        pay_cf_advance_by_months,
        &start_date,
        &mat_dt.unwrap(),
        *as_on_date,
    );

    let mut last_pay_date = get_last_pay_date(start_date, as_on_date, a.int_repay_freq);
    for cashflow_date in cf_date_iterator {
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = a.curr_bal;
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
        .prin_amt = a.curr_bal;

    cashflows
}

fn generate_bullet_cf(a: &InputAccount, convention: &Conventions) -> Cashflow {
    let start_date = a.acct_open_dt.unwrap();
    let mat_date = a.mat_dt;
    if a.term_int_comp_freq <= 0 {
        let days = days_with_convn(start_date, a.mat_dt.unwrap(), convention).unwrap();
        let i_a = interest_amount(a.curr_bal, a.int_rate, days);
        let cf = new_cashflow(i_a, a.curr_bal, &mat_date.unwrap());

        cf
    } else {
        let mut last_compounding_start_date = start_date;
        let compound_interest_advance_by_months = a.term_int_comp_freq as usize;
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = a.curr_bal;
        loop {
            let mut next_cf_compounding_date = rbdate::incr_dt_by_mon_presrv_eom(
                last_compounding_start_date,
                compound_interest_advance_by_months,
            )
            .unwrap();
            if next_cf_compounding_date >= a.mat_dt.unwrap() {
                next_cf_compounding_date = a.mat_dt.unwrap();
            }
            let days = days_with_convn(
                last_compounding_start_date,
                next_cf_compounding_date,
                convention,
            )
            .unwrap();
            let new_i_amount = interest_amount(compounding_principal_amount, a.int_rate, days);
            interest_to_pay_amount += new_i_amount;
            compounding_principal_amount += new_i_amount;
            last_compounding_start_date = next_cf_compounding_date;
            if last_compounding_start_date >= a.mat_dt.unwrap() {
                break;
            }
        }
        let cf = new_cashflow(interest_to_pay_amount, a.curr_bal, &a.mat_dt.unwrap());

        cf
    }
}

fn generate_matured_cf(a: &InputAccount, date: &NaiveDate) -> Cashflow {
    new_cashflow(DEFAULT_FLOAT, a.curr_bal, date)
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
