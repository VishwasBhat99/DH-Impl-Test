use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{decr_dt_by_mon_presrv_eom,  incr_dt_by_mon_presrv_eom_checked, timestamp};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String>  {
    let intt_rt: f64 = account.intt_rate;
    let mut cashflows: Vec<Cashflow> = Vec::new();

    // Case: Negative oustanding balance
    if account.current_book_balance <= 0.0 {
        log_debug!(
            log,
            "Negative or Zero `outstanding balance` for account: `{}`.",
            account.account_number,
        );
    }

    // Case: Negative interest rate
    if intt_rt <= 0.0 {
        log_debug!(
            log,
            "Negative or Zero `interest rate` for account: `{}`.",
            account.account_number,
        );
    } else if intt_rt > 100.00 {
        log_debug!(
            log,
            "`interest rate` is greater than 100% for account: `{}`.",
            account.account_number,
        );
    }
    
    let date_freq = match account.interest_pay_freq.to_uppercase().as_str() {
        "QUARTERLY" => 3,
        "HALF-YEARLY" => 6,
        "YEARLY" => 12,
        _ => 1,
    };
    let int_freq = match account.interest_pay_freq.to_uppercase().as_str() {
        "QUARTERLY" => 4,
        "HALF-YEARLY" => 2,
        "YEARLY" => 1,
        _ => 12,
    };

    let org_date = account.org_date.unwrap_or(*config_params.as_on_date());
    let cf_start_date = incr_dt_by_mon_presrv_eom_checked(org_date, 36).unwrap_or_default();
    let mut prev_cf_date = decr_dt_by_mon_presrv_eom(cf_start_date, date_freq).unwrap_or_default();

    let acc_int_rate = intt_rt / (100*int_freq) as f64;

    let book_bal = account.current_book_balance;
    let total_instalments = account.orig_term as i32;
    let emi_amt = get_emi_amt(acc_int_rate, total_instalments, book_bal);
    let mut period = 1;
    let mut remaining_book_bal = book_bal;
    let mut is_bal_zero = false;

    if total_instalments <= 0 {
        let cf_dt = timestamp(cf_start_date);
        let cf = new_cashflow(0.0, book_bal, cf_dt);
        cashflows.push(cf);
    }
    while period <= total_instalments {

        let cf_date = incr_dt_by_mon_presrv_eom_checked(prev_cf_date, date_freq).unwrap_or_default();
        let cf_dt = timestamp(cf_date);
        let int_amt = get_interest(acc_int_rate, period, book_bal, emi_amt);
        let mut prin_amt = emi_amt - int_amt;
        if prin_amt >= remaining_book_bal {
            prin_amt = remaining_book_bal;
            is_bal_zero = true;
        }
        remaining_book_bal -= prin_amt;
        let cf = new_cashflow(int_amt, prin_amt, cf_dt);
        cashflows.push(cf);
        if is_bal_zero {
            break;
        }

        if prin_amt >= remaining_book_bal {
            break;
        }
        period+=1;
        prev_cf_date = cf_date;
    }

    Ok(cashflows)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
}

fn get_emi_amt(rate: f64, total_instlmnts: i32, mut book_bal: f64) -> f64 {
    book_bal *= -1.0;
    let emi = rate / ((1.0 + rate).powi(total_instlmnts) - 1.0) * -(book_bal * (1.0 + rate).powi(total_instlmnts));
    emi
}

fn get_interest(rate: f64, period: i32, mut book_bal: f64, emi_amt: f64) -> f64 {
    book_bal *= -1.0;
    let interest = get_fv(rate, period - 1, emi_amt, book_bal) * rate;
    interest
}

fn get_fv(rate: f64, no_of_period: i32, emi_amt: f64, book_bal: f64) -> f64 {
    let fv = -(emi_amt * ((1.0 + rate).powi(no_of_period) - 1.0) / rate + book_bal * (1.0 + rate).powi(no_of_period));
    fv
}
