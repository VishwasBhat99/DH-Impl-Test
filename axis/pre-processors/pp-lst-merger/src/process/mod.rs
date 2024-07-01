use self::io::*;
use self::structs::*;
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;
use std::time::{Duration, SystemTime};
mod io;
mod structs;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_log: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let input_file_path = config_params.input_file_path().to_string();
    let mut config_file_excel = open_workbook_auto(config_params.config_file())
        .expect("Unable to open Configuration file.");
    let output_file_path = config_params.output_file();
    let mut output_file = get_writer(output_file_path);
    let concat_fields = config_params.concat_fields().to_owned();
    let asondate = *config_params.as_on_date();
    if let Some(Ok(reader)) = config_file_excel.worksheet_range(config_params.config_sheet_name()) {
        for row in reader.rows().skip(1) {
            let start_timer = SystemTime::now();
            let principal_file_name = format!("{}{}.lst", input_file_path, row[1]);
            let interest_file_name = format!("{}{}.lst", input_file_path, row[2]);
            let mut lst_map: HashMap<String, LstOp> = HashMap::new();

            //Principal file records.
            match new_buf_rdr(&principal_file_name) {
                Ok(principal_file) => {
                    for (line_num, lines) in principal_file.lines().enumerate() {
                        acc_enc += 1;
                        let line = match lines {
                            Ok(line) => line,
                            Err(error) => panic!(
                                "Unable to read file `{}` at line number: `{}` : {}",
                                principal_file_name,
                                line_num + 1,
                                error
                            ),
                        };
                        if !line.is_empty() {
                            let fields: Vec<&str> = line.split('|').collect();
                            let fields_length = fields.len();
                            let mut concat_key: String = "".to_string();
                            for index in &concat_fields {
                                //Check for index validity.
                                if index >= &1 && index <= &fields_length {
                                    let pos = &(index - 1);
                                    let key_val = format!("{}_", fields[pos.to_owned()]);
                                    concat_key.push_str(&key_val);
                                } else {
                                    log_error!(
                                        logger,
                                        "Invalid concat index!! The column number:{:?} could not be fetched from file:{}.",
                                        index,
                                        principal_file_name
                                    );
                                    panic!("Invalid concat index!! Could not get the concat field at position: {} from the file: {}.",index,principal_file_name);
                                }
                            }
                            concat_key.pop();
                            lst_map
                                .entry(concat_key)
                                .and_modify(|data| {
                                    data.append_principal_data(
                                        fields[3].to_string().parse::<f64>().unwrap_or(0.0),
                                        fields[5].to_string().parse::<f64>().unwrap_or(0.0),
                                    )
                                })
                                .or_insert_with(|| get_new_value(fields, "P"));
                        } else {
                            log_error!(
                                logger,
                                "Line number:{} is empty in file:{}",
                                line_num + 1,
                                principal_file_name
                            );
                        }
                    }
                }
                Err(error) => {
                    log_info!(
                        logger,
                        "Could not get the file`{}`, Error: {}.",
                        principal_file_name,
                        error
                    )
                }
            };
            //Interest file records.
            match new_buf_rdr(&interest_file_name) {
                Ok(interest_file) => {
                    for (line_num, lines) in interest_file.lines().enumerate() {
                        acc_enc += 1;
                        let line = match lines {
                            Ok(line) => line,
                            Err(error) => panic!(
                                "Unable to read file `{}` at line number: `{}` : {}",
                                interest_file_name,
                                line_num + 1,
                                error
                            ),
                        };
                        if !line.is_empty() {
                            let fields: Vec<&str> = line.split('|').collect();
                            let fields_length = fields.len();

                            let mut concat_key: String = "".to_string();
                            for index in &concat_fields {
                                //Check for index validity.
                                if index >= &1 && index <= &fields_length {
                                    let pos = &(index - 1);
                                    let key_val = format!("{}_", fields[pos.to_owned()]);
                                    concat_key.push_str(&key_val);
                                } else {
                                    log_error!(
                                        logger,
                                        "Invalid concat index!! The column number:{:?} could not be fetched from file:{}.",
                                        index,
                                        interest_file_name
                                    );
                                    panic!("Invalid concat index!! Could not get the concat field at position: {} from the file: {}.",index,interest_file_name);
                                }
                            }
                            concat_key.pop();
                            // To exclude overdue cashflows
                            if config_params.is_exclude_overdue_interest_cashflow {
                                let cf_date = NaiveDate::parse_from_str(fields[8], "%d-%m-%Y")
                                    .unwrap_or_else(|_| asondate);
                                if cf_date <= asondate {
                                    log_debug!(
                                        logger,
                                        "Skipping record at line {} as cashflow date is before or equal to the asondate, and the exclude overdue interest flag is true",
                                        line_num + 1
                                    );
                                    continue;
                                }
                            }
                            lst_map
                                .entry(concat_key)
                                .and_modify(|data| {
                                    data.append_interest_data(
                                        fields[3].to_string().parse::<f64>().unwrap_or(0.0),
                                        fields[5].to_string().parse::<f64>().unwrap_or(0.0),
                                    )
                                })
                                .or_insert_with(|| get_new_value(fields, "I"));
                            acc_succ += 1;
                        } else {
                            log_error!(
                                logger,
                                "Line number:{} is empty in file:{}",
                                line_num + 1,
                                interest_file_name
                            );
                        }
                    }
                }
                Err(error) => {
                    log_info!(
                        logger,
                        "Could not get the file`{}`, Error: {}.",
                        interest_file_name,
                        error
                    )
                }
            }
            for (key, data) in lst_map.drain() {
                let mut wtd_int_rate = 0.0;
                //Calculate the total weighted int rate. Use interest avg wtd rate only if principal wtd rate is 0.
                if data.prin_amt != 0.0 {
                    wtd_int_rate = data.p_int_rate / data.prin_amt;
                } else if data.int_amt != 0.0 {
                    wtd_int_rate = data.i_int_rate / data.int_amt;
                } else {
                    log_info!(logger,"Cannot calculate weighted total interest rate for concat key:{}. Principal amt:{}, interest amt:{}",key,data.prin_amt,data.int_amt);
                }
                writeln!(
                    output_file,
                    "{}|{}|{}|{}|{}|{}|{}",
                    data.first_to_amt,
                    data.prin_amt,
                    data.int_amt,
                    data.ccyid,
                    wtd_int_rate,
                    data.amt_to_last,
                    key
                )
                .expect("Unable to write to output file.");
            }
            let end_timer = SystemTime::now();
            let duration = end_timer
                .duration_since(start_timer)
                .unwrap_or(Duration::new(0, 0));
            log_debug!(
                logger,
                "Time taken to process record {} in config file is:{:?}",
                row[0],
                duration
            );
        }
    }
    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file());
}
