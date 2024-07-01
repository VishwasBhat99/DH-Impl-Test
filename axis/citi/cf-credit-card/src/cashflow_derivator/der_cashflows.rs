use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{decr_dt_by_mon_presrv_eom, increment_date_by_months, timestamp};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;

pub fn derive_cashflows(
    account: &mut InputAccount,
    _config_params: &ConfigurationParameters,
    log: &Logger,
    convention: Conventions,
) -> Result<Vec<Cashflow>, String> {
    let mut ost_bal = account.outstanding_balance_inr;
    if ost_bal == 0.0 {
        return Ok(vec![new_cashflow(0.0, 0.0, 0)]);
    }
    let mut pending_installments = account.total_tenure - account.completed_tenure;
    if pending_installments <= 0 {
        log_error!(log,"Account:{} has pending installment:{} and outstanding amount:{}. Writing default cashflow.",account.account_number,pending_installments,ost_bal);
        return Ok(vec![new_cashflow(0.0, 0.0, 0)]);
    } else if ost_bal == 0.0 {
        log_error!(
            log,
            "Account:{} has ost_bal:{}. Writing default cashflow.",
            account.account_number,
            ost_bal
        );
        return Ok(vec![new_cashflow(0.0, 0.0, 0)]);
    }
    let mut cfs: Vec<Cashflow> = Vec::new();
    let cf_freq = match &account.installment_frequency[..] {
        "MONTHLY" => 1,
        "BI-MONTHLY" => 2,
        "QUARTERLY" => 3,
        "HALF YEARLY" => 6,
        "YEARLY" => 12,
        _ => {
            log_warn!(
                log,
                "Payment frequency '{}' is incorrect for account: {}. Using Default Pay Freq: Monthly",
                account.installment_frequency, account.account_number
            );
            1
        }
    };
    let mut last_cf_date = decr_dt_by_mon_presrv_eom(account.bill_due_day, cf_freq)
        .unwrap_or(NaiveDate::from_ymd(1970, 1, 1));
    while pending_installments > 0 {
        let cf_date = increment_date_by_months(last_cf_date, cf_freq as u16);
        let int_amt = cal_int_amt(
            last_cf_date,
            cf_date,
            ost_bal,
            account.rate_of_int,
            convention,
        );
        if ost_bal <= 0.0 {
            break;
        }
        let principal_amount = account.emi_amount - int_amt;
        if principal_amount > ost_bal {
            cfs.push(new_cashflow(int_amt, ost_bal, timestamp(cf_date)));
            ost_bal = 0.0;
            break;
        } else {
            cfs.push(new_cashflow(int_amt, principal_amount, timestamp(cf_date)));
        }
        last_cf_date = cf_date;
        pending_installments -= 1;
        ost_bal -= principal_amount;
    }
    //If ost_bal is pending after installments adjust it to the last cashflow.
    if ost_bal > 0.0 {
        let mut last_cf = cfs.pop().unwrap_or(new_cashflow(0.0, 0.0, 0));
        last_cf.prin_amt += ost_bal;
        cfs.push(last_cf);
    }
    Ok(cfs)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;
    cf
}

fn cal_int_amt(
    last_cf_date: NaiveDate,
    next_cf_date: NaiveDate,
    outstanding_amount: f64,
    int_rate: f64,
    convention: Conventions,
) -> f64 {
    let days = days_with_convn(last_cf_date, next_cf_date, &convention)
        .expect("Could not get days by convention.");
    let no_of_days = days.days_btw_dts as f64;
    let days_in_yr = days.day_in_yr as f64;
    (outstanding_amount * int_rate * no_of_days) / (days_in_yr * 100.0)
}
