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
) -> Result<Vec<Cashflow>, String> {
    log_debug!(log, "Account processing: `{}`.", account.account_number);
    let maturity_date = account
        .next_repr_date
        .expect("Cannot parse Maturity Date")
        .min(account.maturity_date.expect("Cannot parse 'Maturity Date'"));
    if maturity_date <= *as_on_date {
        return Ok(vec![new_cashflow(0.0, account.amount, as_on_date)]);
    }
    // MARK: Bullet Cashflow Generation
    if account
        .next_payment_date
        .expect("Cannot parse Next Payment Date")
        > maturity_date
        || account.amount <= DEFAULT_FLOAT
        || account.int_rate <= DEFAULT_FLOAT
        || account.int_rate > 100.0
        || account.int_pay_freq <= DEFAULT_INT
    {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.account_number,
            "Account open date is after maturity date, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account, as_on_date);
        return Ok(vec![cf]);
    }

    // Interest Cashflow Generation
    let day_convention = match &account.int_calc_basis[..] {
        "ACT/ACT" => Conventions::ACTbyACT,
        "ACT/365" => Conventions::ACTby365,
        "ACT/360" => Conventions::ACTby360,
        "30/360" => Conventions::Thirtyby360,
        _ => panic!("Incorrect day convention parameter passed:- Must be one of ACT/ACT, ACT/365, ACT/360, 30/360")
    };
    let interest_cfs = generate_interest_cfs(account, as_on_date, &day_convention);
    Ok(interest_cfs)
}

fn generate_interest_cfs(
    account: &InputAccount,
    as_on_date: &NaiveDate,
    convention: &Conventions,
) -> Vec<Cashflow> {
    let mut cashflows = Vec::new();

    let o_a = account.amount;
    let i_r = account.int_rate;

    let default_principal_amount_for_cf = 0.0;
    let mut last_cf_date = *as_on_date;
    let cf_date_iterator = CFDateIterator::new_from_account(account);

    let days = days_with_convn(
        *as_on_date,
        account
            .next_payment_date
            .expect("Cannot parse Next Payment Date"),
        convention,
    )
    .unwrap();
    let i_a = interest_amount(o_a, i_r, days);
    let cf = new_cashflow(
        i_a,
        default_principal_amount_for_cf,
        &account
            .next_payment_date
            .expect("Cannot parse Next Payment Date"),
    );
    cashflows.push(cf);
    last_cf_date = account.next_payment_date.unwrap();

    for cashflow_date in cf_date_iterator {
        let days = days_with_convn(last_cf_date, cashflow_date, convention).unwrap();
        if &days.days_btw_dts == &0 {
            break;
        }
        let i_a = interest_amount(o_a, i_r, days);
        let cf = new_cashflow(i_a, default_principal_amount_for_cf, &cashflow_date);
        cashflows.push(cf);

        last_cf_date = cashflow_date;
    }

    cashflows
        .last_mut()
        .expect("Cashflows matured without generating any cashflows.")
        .principal_amount = o_a;

    cashflows
}
fn generate_matured_cf(a: &InputAccount, date: &NaiveDate) -> Cashflow {
    new_cashflow(DEFAULT_FLOAT, a.amount, date)
}

fn interest_amount(o_a: f64, i_r: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_yr = days.day_in_yr as f64;
    (o_a * i_r * num_days as f64) / (days_in_yr * 100.0)
}

fn new_cashflow(i_a: f64, p_a: f64, d: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = rbdate::timestamp(*d);

    cf
}
