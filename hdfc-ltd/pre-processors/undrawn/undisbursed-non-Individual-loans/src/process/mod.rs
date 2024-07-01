use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook, Reader, Xlsx};
use health_report::HealthReport;
use rbdate::datevalue_to_naive_date;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file_path());
    let mut tot_acc_encntrd = 5;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    let mut input_file: Xlsx<_> =
        open_workbook(&config_params.input_file()).expect("Unable to open `Input File`.");
    let sheet1 = input_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();

    if let Some(Ok(reader)) = input_file.worksheet_range(sheet1.as_str()) {
        for input_fields in reader.rows().skip(5) {
            tot_acc_encntrd += 1;
            let new_approval_dt = rbdate::increment_date_by_months(
                datevalue_to_naive_date(&input_fields[14].to_string())
                    .expect("Could Not Find Date"),
                *config_params.no_of_months(),
            );
            if new_approval_dt >= *config_params.as_on_date() {
                acc_pro_suc += 1;
                write!(
                    op_writer,
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||||\n",
                    input_fields[0],
                    input_fields[1],
                    input_fields[2],
                    input_fields[3],
                    input_fields[4],
                    input_fields[5],
                    input_fields[6],
                    input_fields[7],
                    input_fields[8],
                    input_fields[9],
                    datevalue_to_naive_date(&input_fields[10].to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y"),
                    datevalue_to_naive_date(&input_fields[11].to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y"),
                    input_fields[12],
                    input_fields[13],
                    datevalue_to_naive_date(&input_fields[14].to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y"),
                    input_fields[15],
                    input_fields[16],
                );
            }
        }
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
