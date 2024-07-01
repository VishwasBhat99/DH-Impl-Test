mod account_as_cashflows;
mod account_writer;
mod cashflow_appender;
mod structs;

use self::cashflow_appender::{append_data_txt, append_data_xl};
use self::structs::LMRBond;
use calamine::{open_workbook, Reader, Xlsx};
use cashflow_generator::account_writer::AccountWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, _: &Logger) {
    let mut writer = AccountWriter::new(&config_params.output_file_path(), log);

    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
    let mut counter = 0;
    let mut ref_file: Xlsx<_> = open_workbook(config_params.ref_file_path()).unwrap();
    let mut ref_map: HashMap<String, LMRBond> = HashMap::new();
    if let Some(Ok(reader)) = ref_file.worksheet_range(config_params.master_sheet_name()) {
        for row in reader.rows() {
            let master_fields = LMRBond {
                class_1: row[1].to_string(),
                class_2: row[2].to_string(),
                class_3: row[3].to_string(),
                tenure_classification: row[4].to_string(),
                sys_identifier: row[5].to_string(),
            };
            let isin = row[0].to_string();
            ref_map.insert(isin, master_fields);
        }
    }
    let file_type: Vec<&str> = config_params.input_file_path().split(".").collect();
    if file_type[1] == "txt" {
        let input_file = match new_buf_rdr(config_params.input_file_path()) {
            Ok(input_file) => input_file,
            Err(error) => panic!("{}", error),
        };
        let reader = BufReader::new(input_file);
        for line in reader.lines() {
            if counter == 0 {
                counter += 1;
                continue;
            }
            let acc_info: String = match line {
                Ok(acc_info) => acc_info,
                Err(error) => {
                    panic!("Cannot read line from input file: {:?}", error);
                }
            };
            let mut row: Vec<&str> = acc_info.split("|").collect();
            if row[0] == "HK_CE" {
                tot_rec += 1;
                tot_amt += row[10].to_string().parse::<f64>().unwrap_or(0.0);
                let account_data = append_data_txt(
                    row,
                    &ref_map,
                    config_params.as_on_date(),
                    config_params.ccy(),
                );
                if !account_data.isin.is_empty() {
                    writer.write(account_data);
                }
            }
        }
    } else if file_type[1] == "xlsx" || file_type[1] == "xls" {
        let mut input_file: Xlsx<_> = open_workbook(config_params.input_file_path()).unwrap();
        if let Some(Ok(reader)) = input_file.worksheet_range(config_params.input_sheet_name()) {
            for row in reader.rows() {
                if counter == 0 {
                    counter += 1;
                    continue;
                }
                if &row[0] == "HK_CE" {
                    tot_rec += 1;
                    tot_amt += row[10].to_string().parse::<f64>().unwrap_or(0.0);
                    let account_data = append_data_xl(
                        row,
                        &ref_map,
                        config_params.as_on_date(),
                        config_params.ccy(),
                    );
                    if !account_data.isin.is_empty() {
                        writer.write(account_data);
                    }
                }
            }
        }
    } else {
        panic!("Invalid Input-file-type: `{:?}`", file_type[1])
    }

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
