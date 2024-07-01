use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::DateParser;
use rbdate::{date_from_timestamp, timestamp};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
use std::process;
pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    if account.leg_type == "PAYMENT" {
        Ok(vec![new_cashflow(
            0.0,
            account.outstanding_notional_pay_leg_lcy.unwrap_or(0.0),
            rbdate::timestamp(account.pay_payment_date.expect("Could Not Find  Date")),
        )])
    } else {
        Ok(vec![new_cashflow(
            0.0,
            account.outstanding_notional_rec_leg_lcy.unwrap_or(0.0),
            rbdate::timestamp(account.rec_payment_date.expect("Could Not Find  Date")),
        )])
    }
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
