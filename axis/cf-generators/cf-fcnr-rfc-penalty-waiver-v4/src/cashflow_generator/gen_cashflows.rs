use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::CompData;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use rbdate::date_from_timestamp;
use rbdate::{incr_dt_by_days, num_days_start_to_end, timestamp};
use std::collections::HashMap;

pub fn generate_cashflows(
    account: &InputAccount,
    config_params: &ConfigurationParameters,
    out_acc: &mut AccountWithCashflows,
    config_map: HashMap<String, CompData>,
) -> Vec<Cashflow> {
    let amt: f64 = account.out_bal_amt;
    let int_amt = 0.0;
    let def_dt = NaiveDate::parse_from_str("01-01-1970", "%d-%m-%Y").unwrap();
    let mat_dt = timestamp(account.maturity_date.expect("Unable to read Maturity date"));

    let st_dt = match account.open_effective_date {
        Some(val) => val,
        None => def_dt,
    };
    let days_due = num_days_start_to_end(st_dt, *config_params.as_on_date());

    let comp_data = match config_map.get(&account.acct_crncy_code) {
        Some(val) => val,
        None => {
            return vec![new_cashflow(int_amt, amt, mat_dt)];
        }
    };

    if (days_due == 365 || days_due == 366)
        && config_params.rfc_fcnr_flag() == "FCNR"
        && account.out_bal_amt < comp_data.limit_amt
    {
        let mut cf_date: i64 = timestamp(*config_params.as_on_date());
        return vec![new_cashflow(int_amt, amt, cf_date)];
    }

    if days_due == 30
        && config_params.rfc_fcnr_flag() == "RFC"
        && account.out_bal_amt < comp_data.limit_amt
    {
        let mut cf_date: i64 = timestamp(incr_dt_by_days(st_dt, 30));
        if cf_date > mat_dt {
            cf_date = mat_dt
        }
        return vec![new_cashflow(int_amt, amt, cf_date)];
    }

    let mut cf_date: i64 = timestamp(incr_dt_by_days(st_dt, comp_data.days));
    if cf_date > mat_dt {
        cf_date = mat_dt
    }

    if account.out_bal_amt < comp_data.limit_amt && days_due < comp_data.days {
        return vec![new_cashflow(int_amt, amt, cf_date)];
    }

    if account.out_bal_amt < comp_data.limit_amt && days_due > comp_data.days {
        let mut cf_date: i64 = timestamp(incr_dt_by_days(*config_params.as_on_date(), 179));
        if cf_date > mat_dt {
            cf_date = mat_dt
        }
        return vec![new_cashflow(int_amt, amt, cf_date)];
    }
    vec![new_cashflow(int_amt, amt, mat_dt)]
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
