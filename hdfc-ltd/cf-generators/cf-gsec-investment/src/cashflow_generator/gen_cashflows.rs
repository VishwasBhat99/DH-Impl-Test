use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::Cashflow;
use chrono::Duration;
use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;

pub fn generate_cashflows(
    account: &mut InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> Result<Vec<Cashflow>, String> {
    let mut cf_vec: Vec<Cashflow> = Vec::new();
    let mat_date = account.maturity_date.unwrap();
    let mut start_date = account
        .last_coupon_date
        .unwrap_or(*config_params.as_on_date())
        + Duration::days(1);
    let end_date = account.next_coupon_date.unwrap_or(mat_date);
    let face_value = account.face_value.unwrap();
    let coupon_rate = account.coupon_rate;
    let mut cf_date = end_date;
    let mut succ_int_amt = face_value * (coupon_rate / 100.0) * (rbdate::num_days_start_to_end(start_date, cf_date) / 30) as f64 * 30.0 / 360.0;

    while cf_date < mat_date {
        let no_mon = rbdate::num_days_start_to_end(start_date, cf_date) / 30;
        succ_int_amt = face_value * (coupon_rate / 100.0) * no_mon as f64 * 30.0 / 360.0;
        cf_vec.push(new_cashflow(succ_int_amt, 0.0, rbdate::timestamp(cf_date)));
        start_date = cf_date + Duration::days(1);
        let freq: &str = &account.coupon_frequency.to_string().trim().to_uppercase();
        cf_date = match freq {
            "ANNUALY" => rbdate::increment_date_by_months(cf_date, 12),
            "SEMI" => rbdate::increment_date_by_months(cf_date, 6),
            "QUARTERLY" => rbdate::increment_date_by_months(cf_date, 3),
            _ => {
                log_error!(
                    log,
                    "`Taking default End date : '{}' for coupon frequency : `{}` .",
                    mat_date,
                    freq
                );
                mat_date
            }
        };
    }

    if *config_params.amount_type() == *"book_val" {
        cf_vec.push(new_cashflow(
            succ_int_amt,
            account.book_value,
            rbdate::timestamp(mat_date),
        ));
    } else {
        cf_vec.push(new_cashflow(
            succ_int_amt,
            account.market_value,
            rbdate::timestamp(mat_date),
        ));
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
