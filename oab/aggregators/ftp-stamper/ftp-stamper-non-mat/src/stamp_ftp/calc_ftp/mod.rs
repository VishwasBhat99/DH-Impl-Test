use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use stamp_ftp::account_with_cashflows::AccountWithCashflows;
mod ftp_calculator;

pub fn calc_ftp(
    mut cf_data_out: AccountWithCashflows,
    method: i32,
    basecurve: i32,
    lst_adjustments: Vec<i32>,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> AccountWithCashflows {
    match method {
        1023 => {
            //Matched Term1 Method implementation
            let cf_data_out1 = ftp_calculator::calc_ftp_cflevel(
                cf_data_out,
                basecurve,
                lst_adjustments,
                config_params,
                log,
            );

            cf_data_out = cf_data_out1;
        }
        _ => {
            //Undefined method here
            log_error!(log, "Executing unimplemented method");
        }
    }

    cf_data_out
}
