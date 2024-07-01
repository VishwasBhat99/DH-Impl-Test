use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::Datelike;
use macros;
use rbdate;
use rbdate::get_days_from_month;
use rbdate::get_month_end_date;
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use sdb_day_convention::conventions::Days;
use sdb_day_convention::days_with_convn;
use slog::Logger;
use statics::DEFAULT_FLOAT;

pub fn generate_cashflows(
    as_on_date: &NaiveDate,
    account: &mut InputAccount,
    log: &Logger,
    day_convention: &Conventions,
) -> Result<Vec<Cashflow>, String> {
    log_debug!(log, "Account processing: `{}`.", account.account_number);

    if account.maturity_date <= *as_on_date {
        return Ok(vec![generate_matured_cf(
            account.current_book_balance,
            as_on_date,
        )]);
    }

    // MARK: Bullet Cashflow Generation
    // If the account start date is after the account maturity date, use the account start date for cashflow generation
    if account.account_start_date > account.maturity_date {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.account_number,
            "Account open date is after maturity date, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account.current_book_balance, as_on_date);
        return Ok(vec![cf]);
    }

    if account.current_book_balance < DEFAULT_FLOAT {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.account_number,
            "Account current balance is less than 0, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account.current_book_balance, as_on_date);
        return Ok(vec![cf]);
    }

    if account.current_book_balance == 0.0 {
        let cf = generate_matured_cf(account.current_book_balance, as_on_date);
        return Ok(vec![cf]);
    }

    if account.int_rate <= DEFAULT_FLOAT || account.int_rate > 100.0 {
        log_warn!(
            log,
            "For account: '{}'. Error: {}",
            account.account_number,
            "Account interest rate is `INVALID`, hence interest cashflows are not generated";
        );
        let cf = generate_matured_cf(account.current_book_balance, &account.maturity_date);
        return Ok(vec![cf]);
    }

    // MARK: Simple Interest Cashflow Generation
    let simple_interest_cfs = generate_simple_interest_cfs(account, as_on_date, day_convention);
    Ok(simple_interest_cfs)
}

fn generate_simple_interest_cfs(
    account: &InputAccount,
    as_on_date: &NaiveDate,
    convention: &Conventions,
) -> Vec<Cashflow> {
    let original_balance = account.current_book_balance;
    let interest_payment_frequency = if account.int_pay_freq == 4 { 1 } else { 3 };
    let (mut first_cf_date, is_month_end) = get_month_end_for_ason(*as_on_date);
    let end_cf_date = account.maturity_date;

    if is_month_end
        && incr_dt_by_mon_presrv_eom_checked(first_cf_date, interest_payment_frequency - 1)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to calculate the next cashflow date for account: {}",
                    account.account_number
                )
            })
            >= end_cf_date
    {
        let days = days_with_convn(*as_on_date, end_cf_date, convention)
            .expect("Failed to calculate days with convention");

        let int_amt =
            calculate_interest_amount(account.current_book_balance, account.int_rate, days);
        return vec![new_cashflow(
            int_amt,
            account.current_book_balance,
            &end_cf_date,
        )];
    }
    let mut cashflows = Vec::new();
    if is_month_end && interest_payment_frequency == 3 {
        first_cf_date = incr_dt_by_mon_presrv_eom_checked(first_cf_date, 2).unwrap();
    }
    let days = days_with_convn(*as_on_date, first_cf_date, &convention)
        .expect("Failed to calculate days with convention");
    let interest_amount =
        calculate_interest_amount(account.current_book_balance, account.int_rate, days);
    let cf = new_cashflow(interest_amount, 0.0, &first_cf_date);
    cashflows.push(cf);

    let mut last_cf_date = first_cf_date;
    while last_cf_date <= end_cf_date {
        let mut cashflow_date =
            incr_dt_by_mon_presrv_eom_checked(last_cf_date, interest_payment_frequency)
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to calculate the next cashflow date for account: {}",
                        account.account_number
                    )
                });
        cashflow_date = get_month_end_date(cashflow_date);
        if cashflow_date <= end_cf_date {
            let days = days_with_convn(last_cf_date, cashflow_date, &convention)
                .expect("Failed to calculate days with convention");
            let interest_amount =
                calculate_interest_amount(account.current_book_balance, account.int_rate, days);
            let cf = new_cashflow(interest_amount, 0.0, &cashflow_date);
            cashflows.push(cf);
            last_cf_date = cashflow_date;
        } else {
            break;
        }
    }

    if last_cf_date < end_cf_date {
        let days = days_with_convn(last_cf_date, end_cf_date, &convention)
            .expect("Failed to calculate days with convention");
        let interest_amount =
            calculate_interest_amount(account.current_book_balance, account.int_rate, days);
        let cf = new_cashflow(interest_amount, 0.0, &end_cf_date);
        cashflows.push(cf);
    }

    cashflows
        .last_mut()
        .expect("Cashflows matured without generating any cashflows.")
        .principal_amount = original_balance;
    cashflows
}

fn generate_matured_cf(current_balance: f64, date: &NaiveDate) -> Cashflow {
    new_cashflow(DEFAULT_FLOAT, current_balance, date)
}

fn calculate_interest_amount(original_balance: f64, interest_rate: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_year = days.day_in_yr as f64;
    (original_balance * interest_rate * num_days as f64) / (days_in_year * 100.0)
}

fn new_cashflow(interest_amount: f64, principal_amount: f64, date: &NaiveDate) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = interest_amount;
    cf.principal_amount = principal_amount;
    cf.date = rbdate::timestamp(*date);
    cf
}

fn get_month_end_for_ason(date: NaiveDate) -> (NaiveDate, bool) {
    let days_in_month = get_days_from_month(date);

    if date.day() == days_in_month as u32 {
        // Given date is the end of the month, return the next month end
        (
            incr_dt_by_mon_presrv_eom_checked(date, 1).unwrap_or_else(|| {
                panic!("Failed to calculate the next month end for date: {}", date)
            }),
            true,
        )
    } else {
        // Given date is not the end of the month, return the current month end
        (get_month_end_date(date), false)
    }
}
