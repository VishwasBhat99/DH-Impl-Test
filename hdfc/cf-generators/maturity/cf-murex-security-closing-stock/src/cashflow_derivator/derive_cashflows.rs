use std::f32::consts::E;

use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use macros;
use rbdate::timestamp;
use sdb_day_convention::{days_with_convn, Conventions, Days};
use slog::Logger;

pub fn derive_cashflows(
    account: &InputAccount,
    as_on_dt: NaiveDate,
    log: &Logger,
    convention: &Conventions,
) -> Vec<Cashflow> {
    let cf_dt: NaiveDate;
    let int_rt = account.rt;
    
    // Case: cf_date is null/empty
      
if let Some(m_dt) = account.mat_dt {
    if let Some(nxt_c_dt) = account.nxt_call_dt {
        if let Some(nxt_p_dt) = account.nxt_put_dt {
           
        //all three dates are available
        if m_dt < nxt_c_dt && m_dt < nxt_p_dt
            {
                cf_dt = m_dt;

            } 
            else 
            {
                if nxt_c_dt < m_dt && nxt_c_dt < nxt_p_dt {
                    cf_dt = nxt_c_dt;
                }
                else {
                    cf_dt = nxt_p_dt;
                  
                }
           }
        }
        //if put date is not available and mat date and calll date available
        else {
            if m_dt < nxt_c_dt {
                cf_dt = m_dt;
            } else {
                cf_dt = nxt_c_dt;
            }
        }
    }
    else {
    //if call date empty but mat date and put date are available
    if let Some(nxt_p_dt) = account.nxt_put_dt {
        if m_dt < nxt_p_dt {
            cf_dt = m_dt;
            } else {
                cf_dt = nxt_p_dt;
            }
            
        }
        else {
            cf_dt=m_dt;
        }
    }
} else 
{
    //if mat date is empty but call and put are available
    if let Some(nxt_c_dt) = account.nxt_call_dt {
        if let Some(nxt_p_dt) = account.nxt_put_dt 
        {
            if nxt_c_dt < nxt_p_dt {
                cf_dt = nxt_c_dt;
                } else {
                    cf_dt = nxt_p_dt;
                }
        }
        else {
             cf_dt = nxt_c_dt;
        }
    }
    else
    { 
         //only put date exists
        if let Some(nxt_p_dt) = account.nxt_put_dt 
        { 
            cf_dt = nxt_p_dt;
        }
        else
         //all three empty
        {
            log_error!(
                log,
                 "`maturity_date` is not well-formatted for account: `{}`.",
                account.deal_no,
            );
            cf_dt = as_on_dt;
        }
    }
}

    
    // Case: Negative oustanding balance
    if account.cf_prin_amt < 0.0 {
        log_error!(
            log,
            "Negative `outstanding balance` for account: `{}`.",
            account.deal_no,
        );
        let negative_o_a_cf = new_cashflow(0.0, account.cf_prin_amt, timestamp(cf_dt));

        log_debug!(
            log,
            "Acount: `{}`, interest amount: `0.0`, principal amount: `{}`, cashflow date: `{:?}`, interest rate: `{}`.",
            account.deal_no,
            account.cf_prin_amt,
            cf_dt,
            int_rt,
        );

        return vec![negative_o_a_cf];
    }
    let days = days_with_convn(as_on_dt, cf_dt, convention)
        .expect("Failed to calculate days with convention");
    let int_amt = calculate_interest_amount(account.cf_prin_amt, account.rt, days);
    // Case: cf_dt < as_on_dt
    if cf_dt < as_on_dt {
        log_info!(log, "`overdue` for account: `{}`.", account.deal_no,);
    }

    log_debug!(
        log,
        "Acount: `{}`, interest amount: `{}`, principal amount: `{}`, cashflow date: `{:?}`",
        account.deal_no,
        int_amt,
        account.cf_prin_amt,
        cf_dt,
    );

    vec![new_cashflow(int_amt, account.cf_prin_amt, timestamp(cf_dt))]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.int_amt = i_a;
    cf.prin_amt = p_a;
    cf.date = d;

    cf
}

fn calculate_interest_amount(original_balance: f64, interest_rate: f64, days: Days) -> f64 {
    let num_days = days.days_btw_dts;
    let days_in_year = days.day_in_yr as f64;
    (original_balance * interest_rate * num_days as f64) / (days_in_year * 100.0)
}
