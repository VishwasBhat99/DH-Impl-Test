extern crate csv;
extern crate serde;
use calamine::{open_workbook_auto, Reader};

use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;

use self::mat_cal::mat_dt_cal;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::time::SystemTime;
mod mat_cal;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_timer = SystemTime::now();

    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let month_long = vec![
        "JANRUARY".to_string(),
        "FEBRUARY".to_string(),
        "MARCH".to_string(),
        "APRIL".to_string(),
        "MAY".to_string(),
        "JUNE".to_string(),
        "JULY".to_string(),
        "AUGUST".to_string(),
        "SEPTEMBER".to_string(),
        "OCTOBER".to_string(),
        "NOVEMBER".to_string(),
        "DECEMBER".to_string(),
    ];
    let month_short = vec![
        "JAN END".to_string(),
        "FEB END".to_string(),
        "MAR END".to_string(),
        "APR END".to_string(),
        "MAY END".to_string(),
        "JUN END".to_string(),
        "JUL END".to_string(),
        "AUG END".to_string(),
        "SEP END".to_string(),
        "OCT END".to_string(),
        "NOV END".to_string(),
        "DEC END".to_string(),
    ];
    let mut output_writer = BufWriter::new(output_file);
    let master_file_extension = Path::new(config_param.master_file_path())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("txt");
    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    //Mapping master File reading started
    log_debug!(log, "Mapping master File reading started");
    if master_file_extension == "xlsx" || master_file_extension == "xls" {
        let mut master_excel = open_workbook_auto(config_param.master_file_path())
            .expect("Unable to open Mapping Master File.");

        if let Some(Ok(reader)) = master_excel.worksheet_range(config_param.master_sheet_name()) {
            for row in reader.rows().skip(0) {
                let mut master_vec: Vec<String> = Vec::new();
                for data in row {
                    master_vec.push(data.to_string().trim().to_string());
                }
                let gl_code = row[0].to_string();
                master_map.insert(gl_code, master_vec);
            }
        }
    } else {
        let master_file = match new_buf_rdr(config_param.master_file_path()) {
            Ok(file) => file,
            Err(_error) => panic!(
                "Could not found master_file: `{}`",
                config_param.master_file_path(),
            ),
        };

        for (line_num, lines) in master_file.lines().enumerate().skip(1) {
            let master_line = match lines {
                Ok(master_line) => master_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.master_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let master_fields = master_line
                .split('|')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            let gl_code = master_fields[0].to_string();
            master_map.insert(gl_code, master_fields);
        }
    }
    log_debug!(log, "Master File Reading Completed");

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found input_file: `{}`",
            config_param.input_file_path(),
        ),
    };

    //input file reading started
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let input_fields: Vec<String> = input_line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();
        if !master_map.contains_key(&input_fields[0]) {
            log_debug!(
                log,
                "Acccount skipped `{}` becasue MaterFile does not contains this value",
                input_fields[0]
            );
            continue;
        }
        succ_rec += 1;
        let master_data = master_map.get(&input_fields[0]).unwrap();
        let mut output_line = "".to_string();
        let master_line = master_data.join("|");
        //Add the master fields in output file
        output_line.push_str(&master_line);
        output_line.push_str("|");
        let input_line = input_fields.join("|");
        output_line.push_str(&input_line);
        output_line.push_str("|");
        //Derivation of maturity date on the basis of 4th field (bucket) of master file
        let bucket = master_data[3].as_str();
        let reporting_date_days = master_data[12].parse::<i64>().unwrap_or(0);
        let mat_date = mat_dt_cal(
            &month_long,
            &month_short,
            config_param.as_on_date(),
            bucket,
            reporting_date_days,
        )
        .format("%d-%m-%Y")
        .to_string();
        output_line.push_str(&mat_date);

        writeln!(output_writer, "{}", output_line).expect("output_line can not be written");
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total process duration.");
    log_debug!(
        log,
        "Total Duration for preprocess the data: {:?}.",
        duration
    );
    info!(
        diag_log,
        "Total Duration for preprocess the data: {:?}.", duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_param.output_file_path());
}
