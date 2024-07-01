use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::DateParser;
use rbdate::{
    date_from_timestamp, get_month_end_date, increment_date_by_months, num_days_start_to_end,
    timestamp,incr_dt_by_mon_presrv_eom,
};
use sdb_day_convention::{days_with_convn, Conventions};
use slog::Logger;
use std::process;
pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    
    let date_parser = DateParser::new("%Y-%m-%d".to_string(), true);
    let start_date =
        rbdate::date_from_timestamp(timestamp(config_params.as_on_date().succ())).to_string();
    let mut mat_date = "".to_string();
    if let Some(dt) = account.maturity_date {
        mat_date = rbdate::date_from_timestamp(timestamp(dt)).to_string();
    }
    let deposit_amount = account.initial_deposit_amount.unwrap_or(0.0);
    let broken_quat_int = account.broken_quat_int.unwrap_or(0.0);
    let mut int_rt = account.int_rate.unwrap_or(0.0);
    let mut total_int = account.interest_accrued;
    if account
        .int_payment_fq
        .to_string()
        .replace(" ", "")
        .to_lowercase()
        == "compoundedquarterly"
    {
        let start_date = *config_params.as_on_date();
        let mut initial_val = deposit_amount + total_int - broken_quat_int;
        let mat_date = account.maturity_date.expect("could not find date");
        let month = start_date.to_string()[5..7].parse::<f32>().unwrap_or(0.0);
        let mut quaterly_int = 0.0;
        let mut end_date;
        if (month <= 3.0) {
            end_date =
                get_month_end_date(increment_date_by_months(start_date, (3.0 - month) as u16));
        } else if (month > 3.0 && month <= 6.0) {
            end_date =
                get_month_end_date(increment_date_by_months(start_date, (6.0 - month) as u16));
        } else if (month > 6.0 && month <= 9.0) {
            end_date =
                get_month_end_date(increment_date_by_months(start_date, (9.0 - month) as u16));
        } else {
            end_date =
                get_month_end_date(increment_date_by_months(start_date, (12.0 - month) as u16));
        }
        let mut no_of_quaterly_days = num_days_start_to_end(start_date, end_date);
        quaterly_int = initial_val * int_rt * (no_of_quaterly_days as f64) / 36500.0;
        total_int = total_int + quaterly_int;
        initial_val = initial_val + broken_quat_int;
        while incr_dt_by_mon_presrv_eom(end_date, 3).expect("Couldnt find date") < mat_date {
            let prev_end = end_date;
            end_date = incr_dt_by_mon_presrv_eom(end_date, 3).expect("Couldnt find date");
            no_of_quaterly_days = num_days_start_to_end(prev_end, end_date);
            initial_val = initial_val + quaterly_int;
            quaterly_int = initial_val * int_rt * (no_of_quaterly_days as f64) / 36500.0;
            total_int = total_int + quaterly_int;
        }
        no_of_quaterly_days = num_days_start_to_end(end_date, mat_date);
        initial_val = initial_val + quaterly_int;
        quaterly_int = initial_val * int_rt * (no_of_quaterly_days as f64) / 36500.0;
        total_int = total_int + quaterly_int;
    } else {
        let no_of_days =
            num_days_start_to_end(date_parser.parse(&start_date),date_parser.parse(&mat_date));
        let fut_int = (no_of_days as f64 + 1.0)* deposit_amount * int_rt / 36500.0;
        total_int = total_int + fut_int;
    }
    Ok(vec![new_cashflow(
        total_int,
        deposit_amount,
        timestamp(date_parser.parse(&mat_date)),
    )])
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
