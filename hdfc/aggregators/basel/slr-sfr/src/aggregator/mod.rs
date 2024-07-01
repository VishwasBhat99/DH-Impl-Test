mod currency;

use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::io::Write;

#[derive(Debug)]
pub struct Data {
    pub ndtl_data: f64,
    pub excess_data: f64,
}

pub fn aggregate_sfr(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.currency_conversion_file_path(),
    );

    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
    let mut sfr_excel: Xlsx<_> =
        open_workbook(config_params.sfr_file_path()).expect("Cannot open SFR excel file.");
    let mut sfr_map: HashMap<NaiveDate, Data> = HashMap::new();
    if let Some(Ok(reader)) = sfr_excel.worksheet_range(&config_params.sfr_sheet_name()) {
        for row in reader.rows() {
            let date = rbdate::datevalue_to_naive_date(&row[0].to_string());
            let excess_amount = row[config_params.amt_col()]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let net_bal_with_banks = row[config_params.amt_col() - 2]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let cash_in_hand = row[config_params.amt_col() - 3]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let average_excess_with_rbi = row[config_params.amt_col() - 4]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let excess_slr_rptd =
                excess_amount - net_bal_with_banks - cash_in_hand - average_excess_with_rbi;
            let ndtl_amount = row[config_params.ndtl_amt_col()]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);

            tot_rec += 1;
            tot_amt += ndtl_amount;
            
            let sfr_data = Data {
                ndtl_data: ndtl_amount,
                excess_data: excess_slr_rptd,
            };
            log_debug!(
                logger,
                "Date column values in SFR files: {:#?}",
                row[0].to_string()
            );
            if date.is_ok() {
                sfr_map.insert(date.unwrap(), sfr_data);
            }
        }
    }
    log_debug!(diag_logger, "{:#?}", sfr_map);

    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut ndtl_output_file = match buf_file_wrtr(config_params.ndtl_output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create ndtl output file: `{}` on location `{}`: {}.",
                config_params.ndtl_output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut default_data = Data {
        ndtl_data: 0.0,
        excess_data: 0.0,
    };
    let sfr_data = sfr_map
        .get_mut(config_params.as_on_date())
        .unwrap_or(&mut default_data);
    let mut native_data = match config_params.denomination_type() {
        "CR" => (sfr_data.excess_data * 10000000.0),
        "L" => (sfr_data.excess_data * 100000.0),
        _ => {
            log_error!(logger, "Invalid Denomination type encountered!!");
            0.0
        }
    };
    if native_data < 0.0 {
        native_data = 0.0;
    }
    let consol_data =
        currency_converter.get_consol_data(config_params.base_currency(), &native_data);

    write!(
        output_file,
        "{}|{}|{}|{}|{}|{}\n",
        config_params.as_on_date().format("%d-%m-%Y"),
        config_params.country(),
        config_params.base_currency(),
        config_params.default_llg_code(),
        native_data,
        consol_data
    )
    .expect("Unable to generate aggregated summary file.");
    for (date, data) in sfr_map.drain() {
        if data.ndtl_data != 0.0 {
            let ndtl_data = match config_params.denomination_type() {
                "CR" => (data.ndtl_data * 10000000.0),
                "L" => (data.ndtl_data * 100000.0),
                _ => {
                    log_error!(logger, "Invalid Denomination type encountered!!");
                    0.0
                }
            };
            write!(
                ndtl_output_file,
                "{}|{}\n",
                date.format("%d-%m-%Y"),
                ndtl_data
            )
            .expect("Unable to generate aggregated summary file.");
        }
    }
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
