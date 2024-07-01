use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, incr_dt_by_days, incr_dt_by_mon_presrv_eom_checked, timestamp};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let as_on_date = config_params.as_on_date();
    let amt: f64 = account.out_bal_amt;
    let int_amt = 0.0;
    let mat_dt = if let Some(dt) = account.maturity_date {
        timestamp(dt)
    } else {
        panic!(
            "Unable to get maturity date for account: `{}`",
            account.acid
        )
    };
    let def_dt = NaiveDate::parse_from_str("01-01-1970", "%d-%m-%Y").unwrap();
    // Case 1
    if account.waiver == "Y" && (account.maturity_modify == "" || account.maturity_modify == "-") {
        let mut cf_date = timestamp(incr_dt_by_days(*as_on_date, 179));
        if cf_date > mat_dt {
            cf_date = mat_dt;
        }
        return Ok(vec![new_cashflow(int_amt, amt, cf_date)]);
    }

    // Case 2
    if account.waiver == "Y25" && (account.maturity_modify == "" || account.maturity_modify == "-")
    {
        // Cashflow 1
        let mut cfs: Vec<Cashflow> = Vec::new();
        let mut cf_date = timestamp(incr_dt_by_days(*as_on_date, 179));
        let prin_amt = amt * 25.0 / 100.0;
        if cf_date > mat_dt {
            cf_date = mat_dt;
        }
        cfs.push(new_cashflow(int_amt, prin_amt, cf_date));

        // Cashflow 2
        let p_a = amt - prin_amt;
        cfs.push(new_cashflow(
            int_amt,
            p_a,
            timestamp(account.maturity_date.unwrap()),
        ));
        return Ok(cfs);
    }

    // Case 3
    if account.waiver == "Y25"
        && (account.maturity_modify == "M" || account.maturity_modify == "6M")
    {
        let mut cfs: Vec<Cashflow> = Vec::new();
        // Cashflow 1
        let mut cf_date = timestamp(incr_dt_by_days(*as_on_date, 179));
        let prin_amt = amt * 25.0 / 100.0;

        if cf_date > mat_dt {
            cf_date = mat_dt;
        }
        cfs.push(new_cashflow(int_amt, prin_amt, cf_date));

        // Cashflow 2
        let p_a = amt - prin_amt;
        let st_dt = match account.open_effective_date {
            Some(dt) => dt,
            None => {
                log_info!(
                    log,
                    "Unable to get the Account Start Date for account: {}",
                    account.acid
                );
                def_dt
            }
        };
        let mut cf_date = timestamp(incr_dt_by_mon_presrv_eom_checked(st_dt, 15).unwrap());
        if cf_date > mat_dt {
            cf_date = mat_dt;
        } else if cf_date < timestamp(*as_on_date) {
            cf_date = timestamp(*as_on_date);
        }
        cfs.push(new_cashflow(int_amt, p_a, cf_date));
        return Ok(cfs);
    }

    // Case 4
    if account.waiver == "N" && (account.maturity_modify == "M" || account.maturity_modify == "6M")
    {
        let mut cfs: Vec<Cashflow> = Vec::new();
        let st_dt = match account.open_effective_date {
            Some(dt) => dt,
            None => {
                log_info!(
                    log,
                    "Unable to get the Account Start Date for account: {}",
                    account.acid
                );
                def_dt
            }
        };
        let mut cf_date = timestamp(incr_dt_by_mon_presrv_eom_checked(st_dt, 15).unwrap());
        if cf_date > mat_dt {
            cf_date = mat_dt;
        }
        cfs.push(new_cashflow(int_amt, amt, cf_date));
        return Ok(cfs);
    }
    return Ok(vec![new_cashflow(
        int_amt,
        amt,
        timestamp(account.maturity_date.unwrap()),
    )]);
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
