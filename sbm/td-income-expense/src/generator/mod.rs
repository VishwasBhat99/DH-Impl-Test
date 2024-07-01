use self::config::read_config_file;
use crate::configuration_parameters::ConfigurationParameters;
use crate::generator::implementation::implement_income_expense;
use crate::generator::utility::{
    get_exchange_rate_map, get_income_master_map, get_pnl_bacid_map, get_td_daily_basis_map,
};
use crate::macros;
use chrono::{Datelike, NaiveDate, Duration};
use health_report::HealthReport;
use rbdate::decr_dt_by_mon_presrv_eom;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;

mod config;
mod implementation;
mod utility;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let config = read_config_file(config_params.config_file_path());

    let date_folder = config_params.as_on_date().format("%d%m%Y").to_string();
    let prev_month_ason = decr_dt_by_mon_presrv_eom(config_params.as_on_date(), 1)
        .expect("Cannot get previous month date");
    let prev_month_date_folder = prev_month_ason.format("%d%m%Y").to_string();

    let output_file = config.output_file_path.replace("{ddmmyyyy}", &date_folder);
    let mut writer = match buf_file_wrtr(&output_file, None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create Output file: `{}` on location `{}` : {:?}.",
                &output_file,
                env::current_exe()
                    .unwrap_or_else(|error| {
                        panic!("Unable to find current directory path: {}", error);
                    })
                    .display(),
                error
            );
        }
    };

    // AsOn Dates

    let year = config_params.as_on_date().year();
    let month = config_params.as_on_date().month();

    let first_day_of_month =
        NaiveDate::from_ymd_opt(year, month, 1).expect("Cannot get first date of month");
    let last_day_of_month = match month {
        12 => NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .expect("Cannot get first day of next month")
            .pred_opt()
            .expect("Cannot get last day of the month"),
        _ => NaiveDate::from_ymd_opt(year, month + 1, 1)
            .expect("Cannot get first day of next month")
            .pred_opt()
            .expect("Cannot get last day of the month"),
    };
    let second_day_of_month = first_day_of_month + Duration::days(1);
    let third_day_of_month = first_day_of_month + Duration::days(2);
    // BALM FC GSP

    log_debug!(log, "Reading BALM FC GSP File started");
    let balm_fc_file_path = config.balm_fc_gsp.replace("{ddmmyyyy}", &date_folder);
    let pnl_bacid_map = get_pnl_bacid_map(balm_fc_file_path, config.pnl_bacid_position, log);
    log_debug!(log, "Reading BALM FC GSP File ended");

    // PREVIOUS MONTH INCOME MASTER

    log_debug!(log, "Reading Previous Month Income Master File started");
    let prev_month_income_master_file = config
        .income_master_previous_month
        .replace("{ddmmyyyy}", &prev_month_date_folder);
    let prev_month_income_master_map = get_income_master_map(prev_month_income_master_file, log);
    log_debug!(log, "Reading Previous Month Income Master File ended");

    // CURRENT MONTH INCOME MASTER

    log_debug!(log, "Reading Current Month Income Master File started");
    let curr_month_income_master_file = config
        .income_master_current_month
        .replace("{ddmmyyyy}", &date_folder);
    let curr_month_income_master_map = get_income_master_map(curr_month_income_master_file, log);
    log_debug!(log, "Reading Current Month Income Master File ended");

    // EXCHANGE RATE

    log_debug!(log, "Reading Exchange Rate File started");
    let exchange_rate_file = config.exchange_rate.replace("{ddmmyyyy}", &date_folder);
    let currency_map = get_exchange_rate_map(config.local_currency, exchange_rate_file);
    log_debug!(log, "Reading Exchange Rate File ended");

    // TD DAILY FILES

    log_debug!(log, "Reading TD Daily Files started");

    let td_daily_basis_input_map = get_td_daily_basis_map(
        config.td_daily_files,
        first_day_of_month,
        last_day_of_month,
        log,
    );

    log_debug!(log, "Reading TD Daily Files ended");

    // BALM FC TD

    log_debug!(log, "Processing BALM FC TD File started");

    let balm_fc_td_file = config.balm_fc_td.replace("{ddmmyyyy}", &date_folder);
    implement_income_expense(
        &balm_fc_td_file,
        &mut tot_rec,
        &mut skp_rec,
        log,
        &pnl_bacid_map,
        first_day_of_month,
        last_day_of_month,
        second_day_of_month,
        third_day_of_month,
        &curr_month_income_master_map,
        &prev_month_income_master_map,
        &td_daily_basis_input_map,
        &currency_map,
        &mut writer,
    );

    log_debug!(log, "Processing BALM FC TD File ended");

    // BALM FC TD CLOSED

    log_debug!(log, "Processing BALM FC TD Closed File started");
    let balm_fc_td_file_closed = config.balm_fc_td_closed.replace("{ddmmyyyy}", &date_folder);
    implement_income_expense(
        &balm_fc_td_file_closed,
        &mut tot_rec,
        &mut skp_rec,
        log,
        &pnl_bacid_map,
        first_day_of_month,
        last_day_of_month,
        second_day_of_month,
        third_day_of_month,
        &curr_month_income_master_map,
        &prev_month_income_master_map,
        &td_daily_basis_input_map,
        &currency_map,
        &mut writer,
    );

    log_debug!(log, "Processing BALM FC TD Closed File ended");

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, 0.0, 0.0, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&output_file);
}
