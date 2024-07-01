use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::CompData;
use chrono::NaiveDate;
use rbdate::{incr_dt_by_days, num_days_start_to_end, timestamp, date_from_timestamp};
use std::collections::HashMap;

pub fn generate_cashflows(
    account: &InputAccount,
    as_on_date: NaiveDate,
    out_acc: &mut AccountWithCashflows,
    config_map: HashMap<String, CompData>,
) -> Vec<Cashflow> {
    let amt: f64 = account.amount;
    let int_amt = 0.0;
    let def_dt = NaiveDate::parse_from_str("01-01-1970", "%d-%m-%Y").unwrap();
    let mat_dt = timestamp(account.maturity_date.expect("Unable to read Maturity date"));

    let st_dt = match account.start_date {
        Some(val) => val,
        None => def_dt,
    };
    let days_due = num_days_start_to_end(st_dt, as_on_date);
    out_acc.days_passed_due = days_due;

    let comp_data = match config_map.get(&account.currency) {
        Some(val) => val,
        None => {
            out_acc.revised_mat_dt = mat_dt;
            return vec![new_cashflow(int_amt, amt, mat_dt)];
        }
    };
    let cf_date: i64 = timestamp(incr_dt_by_days(st_dt, comp_data.days));
    out_acc.revised_mat_dt = cf_date;

    if account.amount < comp_data.limit_amt && days_due < comp_data.days {
        return vec![new_cashflow(int_amt, amt, cf_date)];
    }
    out_acc.revised_mat_dt = mat_dt;
    vec![new_cashflow(int_amt, amt, mat_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
