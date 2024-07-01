use account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::NaiveDate;
use rbdate::{date_from_timestamp, timestamp};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_dt: i64 = 0;
    let as_on_dt: i64;
    let mut int_amt: f64 = 0.0;
    let mut amt: f64 = 0.0;

    amt = account.bal.to_string().parse::<f64>().unwrap_or(0.0).abs();
    int_amt = account.int_accrued_amt.to_string().parse::<f64>().unwrap_or(0.0);

    as_on_dt = timestamp(*config_params.as_on_date());

    if account.next_reset_dt.is_none() && account.lm_exp.is_none() {
        cf_dt = as_on_dt;
    } else if account.next_reset_dt.is_none() && !account.lm_exp.is_none() {
        if let Some(dt) = account.lm_exp {
            cf_dt = timestamp(dt);
        }
    } else if !account.next_reset_dt.is_none() && account.lm_exp.is_none() {
        if let Some(dt) = account.next_reset_dt {
            cf_dt = timestamp(dt);
        }
    } else {
        let mut cf_dt_reset = 0;
        let mut cf_dt_exp = 0;
        if let Some(dt) = account.next_reset_dt {
            cf_dt_reset = timestamp(dt);
        }
        if let Some(dt) = account.lm_exp {
            cf_dt_exp = timestamp(dt);
        }
        if cf_dt_exp < cf_dt_reset {
            cf_dt = cf_dt_exp;
        } else {
            cf_dt = cf_dt_reset;
        }
    }
    Ok(vec![new_cashflow(int_amt, amt, cf_dt)])
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
