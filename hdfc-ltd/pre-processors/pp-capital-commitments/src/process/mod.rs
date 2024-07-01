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
    let mut tot_acc_encntrd = 4;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    let mut input_file: Xlsx<_> =
        open_workbook(&config_params.input_file()).expect("Unable to open `Input File`.");
    let first_sheet_name = input_file
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();

    if let Some(Ok(reader)) = input_file.worksheet_range(first_sheet_name.as_str()) {
        for input_fields in reader.rows().skip(4) {
            tot_acc_encntrd += 1;
            acc_pro_suc += 1;
            if input_fields[0].to_string() == "" {
                break;
            }
            write!(
                op_writer,
                "||{}|{}|{}|{}|{}||||{}|{}|{}|{}|{}|{}||||||\n",
                input_fields[0],
                input_fields[2],
                input_fields[1],
                input_fields[4],
                input_fields[3],
                config_params.as_on_date().format("%d-%m-%Y"),
                config_params.as_on_date().format("%d-%m-%Y"),
                input_fields[5],
                input_fields[6],
                config_params.as_on_date().format("%d-%m-%Y"),
                config_params.currency(),
            );
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