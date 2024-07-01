use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use configuration_parameters::ConfigurationParameters;
use rbdate::get_month_end_date;
use rbdate::{increment_date_by_months, timestamp};
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
    per_slabs: &Vec<f64>,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_vec: Vec<Cashflow> = Vec::new();
    let succ_cf_dt = account.first_cf_dt.expect("Could Not Find Date");
    let cf_amount = account.sanc_amt.expect("Could Not Find Date");
    let mut count = 1;
    let mut first_cf = 1;
    for per in per_slabs {
        if increment_date_by_months(succ_cf_dt, count) <= *config_params.as_on_date() {
            cf_vec.push(new_cashflow(
                0.0,
                0.0,
                timestamp(increment_date_by_months(succ_cf_dt, count)),
            ));
            count += 1;
        } else {
            if first_cf == 1 {
                cf_vec.push(new_cashflow(
                    0.0,
                    cf_amount * (per / 2.0),
                    timestamp(*config_params.as_on_date()),
                ));
                cf_vec.push(new_cashflow(
                    0.0,
                    cf_amount * (per / 2.0),
                    timestamp(get_month_end_date(*config_params.as_on_date())),
                ));
                first_cf += 1;
                if timestamp(*config_params.as_on_date())
                    != timestamp(get_month_end_date(*config_params.as_on_date()))
                {
                    count += 1;
                }
                continue;
            }
            cf_vec.push(new_cashflow(
                0.0,
                cf_amount * per,
                timestamp(increment_date_by_months(succ_cf_dt, count)),
            ));
            count += 1;
        }
    }
    Ok(cf_vec)
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;

    cf
}
