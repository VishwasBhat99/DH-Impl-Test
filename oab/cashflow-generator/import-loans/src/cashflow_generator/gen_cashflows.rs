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
    log_debug!(log, "Account processing: `{}`.", account.account_id);

    // Case 1: When CFPAmt or Outstanding Balance is negative
    if account.outstanding_bal <= DEFAULT_FLOAT {
        log_error!(
            log,
            "Negative or Zero `outstanding balance` for account: `{}`.",
            account.account_id,
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }

    // Case 2: When CF Date < AsOn Date (Overdue)
    if account.maturity_date <= *as_on_date {
        return Ok(vec![new_cashflow(0.0, account.outstanding_bal, as_on_date)]);
    }

    //Case 3: When CF Date is null/empty
    // Handled in InputAccount struct

    // Case 4: When CF Date > Maturity Date
    if account.start_date > account.maturity_date {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.account_id,
            "Account start date is after maturity date, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }

    // To avoid problems in interest calculation
    if account.int_rate < DEFAULT_FLOAT || account.int_rate > 100.0 {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.account_id,
            "Account interest rate is `INVALID`, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, &account.maturity_date);
        return Ok(vec![cf]);
    }

    // To avoid problems in CF Date calculation
    if account.int_repayment_frequency <= DEFAULT_INT {
        if account.int_repayment_frequency < DEFAULT_INT {
            log_warn!(
                log,
                "For account: '{}'. Error: {}",
                account.account_id,
                "Account interest payment frequency is INVALID, hence generated interest and principal cashflow at maturity";
            );
        }
        let cf = generate_bullet_cf(account, day_convention);
        return Ok(vec![cf]);
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

    let o_a = a.outstanding_bal;
    let i_r = a.int_rate;

    let default_principal_amount_for_cf = 0.0;
    let mut last_cf_date = get_last_pay_date(a.start_date, as_on_date, a.int_repayment_frequency);
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

fn generate_bullet_cf(a: &InputAccount, convention: &Conventions) -> Cashflow {
    let start_date = a.start_date;
    let mat_date = a.maturity_date;
    if a.int_repayment_frequency <= 0 {
        let days = days_with_convn(start_date, a.maturity_date, convention).unwrap();
        let i_a = interest_amount(a.outstanding_bal, a.int_rate, days);
        let cf = new_cashflow(i_a, a.outstanding_bal, &mat_date);

        cf
    } else {
        let mut last_compounding_start_date = start_date;
        let compound_interest_advance_by_months = a.int_repayment_frequency as usize;
        let mut interest_to_pay_amount = 0.0;
        let mut compounding_principal_amount = a.outstanding_bal;
        loop {
            let mut next_cf_compounding_date = rbdate::incr_dt_by_mon_presrv_eom(
                last_compounding_start_date,
                compound_interest_advance_by_months,
            )
            .unwrap();
            if next_cf_compounding_date >= a.maturity_date {
                next_cf_compounding_date = a.maturity_date;
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
            if last_compounding_start_date >= a.maturity_date {
                break;
            }
        }
        let cf = new_cashflow(interest_to_pay_amount, a.outstanding_bal, &a.maturity_date);

        cf
    }
}

fn generate_matured_cf(a: &InputAccount, date: &NaiveDate) -> Cashflow {
    new_cashflow(DEFAULT_FLOAT, a.outstanding_bal, date)
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
