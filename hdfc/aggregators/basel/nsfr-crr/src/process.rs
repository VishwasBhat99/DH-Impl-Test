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

pub fn process_sfr(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
    let mut sfr_excel: Xlsx<_> =
        open_workbook(config_params.sfr_file_path()).expect("Cannot open SFR excel file.");
    let mut sfr_map: HashMap<NaiveDate, f64> = HashMap::new();
    if let Some(Ok(reader)) = sfr_excel.worksheet_range(&config_params.sfr_sheet_name()) {
        for row in reader.rows() {
            let date = rbdate::datevalue_to_naive_date(&row[0].to_string());
            let tot_crr_amt = row[config_params.amt_col()]
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            tot_rec += 1;
            tot_amt += tot_crr_amt;
            log_debug!(
                logger,
                "Date column values in SFR files: {:#?}",
                row[0].to_string()
            );
            if date.is_ok() {
                sfr_map.insert(date.expect("Unexpected unwrap error!!"), tot_crr_amt);
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
    let mut def_float = 0.0;
    let amt = sfr_map
        .get_mut(config_params.as_on_date())
        .unwrap_or(&mut def_float);
    let mut crr_final_amt = match config_params.denomination_type() {
        "CR" => (*amt * 10000000.0),
        "L" => (*amt * 100000.0),
        _ => {
            log_error!(logger, "Invalid Denomination type encountered!!");
            *amt * 1.0
        }
    };
    if config_params.is_negative() {
        crr_final_amt *= -1.0;
    }
    write!(
        output_file,
        "{}|{}|{}|{}|{}|{}\n",
        config_params.as_on_date().format("%d-%m-%Y"),
        config_params.country(),
        config_params.base_currency(),
        config_params.default_llg_code(),
        crr_final_amt,
        crr_final_amt
    )
    .expect("Unable to generate aggregated summary file.");

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(logger, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
