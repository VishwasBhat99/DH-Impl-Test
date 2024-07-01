use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook, Reader, Xlsx};
use health_report::HealthReport;
use rbdate::{datevalue_to_naive_date, get_month_end_date};
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let mut tot_acc_encntrd = 1;
    let mut acc_pro_suc = 0;

    let mut input_file: Xlsx<_> =
        open_workbook(&config_params.input_file()).expect("Unable to open `Input File`.");
    let sheet1 = input_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();

    if let Some(Ok(reader)) = input_file.worksheet_range(sheet1.as_str()) {
        for input_fields in reader.rows().skip(1) {
            tot_acc_encntrd += 1;
            let amt_14per = 0.14 * (input_fields[1].to_string().parse::<f64>().unwrap_or(0.0));
            let amt_2per = 0.02 * (input_fields[1].to_string().parse::<f64>().unwrap_or(0.0));
            let approval_date = datevalue_to_naive_date(&input_fields[2].to_string())
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y");
            let cf_date = get_month_end_date(
                datevalue_to_naive_date(&input_fields[2].to_string())
                    .unwrap_or(*config_params.as_on_date()),
            );
            write!(
                op_writer,
                "{}|{}|{}|{}|{}|{}|{}\n",
                input_fields[0],
                input_fields[1],
                amt_14per,
                amt_2per,
                approval_date,
                cf_date.format("%d-%m-%Y"),
                input_fields[3]
            );
        }
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
