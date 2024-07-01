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
    let cf_dt: i64 = timestamp(
        account
            .final_next_repricing_date
            .unwrap_or(*config_params.as_on_date()),
    );
    let int_amt: f64 = 0.0;
    let amt: f64 = account
        .out_bal_amt
        .to_string()
        .parse::<f64>()
        .unwrap_or(0.0)
        .abs();
    Ok(vec![new_cashflow(int_amt, amt, cf_dt)])
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
