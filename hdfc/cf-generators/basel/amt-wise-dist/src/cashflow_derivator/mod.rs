use slog::Logger;
mod account;
mod account_appender;
mod account_writer;
mod convert_datatype;
mod get_output;
mod io;
mod structs;

use self::get_output::get_output_map;
use self::io::{get_cf_writer, get_txt_writer};
use self::structs::*;
use crate::cashflow_derivator::account_appender::create_account;
use crate::cashflow_derivator::convert_datatype::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_generator_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut tot_amt = 0.0;

    let op_path = config_params.output_file_path().to_string();
    let output_path = format!("{}.txt", &config_params.output_file_path());
    let mut output_file = get_txt_writer(&output_path);
    let mut writer = get_cf_writer(&op_path, log);

    let ret_input_file = match File::open(&config_params.ret_input_file_path()) {
        Ok(ret_input_file) => ret_input_file,
        Err(error) => panic!("{}", error),
    };
    let ret_reader = BufReader::new(ret_input_file);
    let non_ret_input_file = match File::open(&config_params.non_ret_input_file_path()) {
        Ok(non_ret_input_file) => non_ret_input_file,
        Err(error) => panic!("{}", error),
    };
    let non_ret_reader = BufReader::new(non_ret_input_file);

    let mut output_map: HashMap<KeyID, AccsData> = HashMap::new();
    for line in ret_reader.lines() {
        match line {
            Ok(input_line) => {
                let input_fields = input_line.split('|').collect::<Vec<&str>>();
                tot_amt += str_to_flt(input_fields[19]);
                tot_rec += 1;
                output_map = get_output_map(output_map, input_fields, &log);
            }
            Err(error) => {
                skp_rec += 1;
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    for line in non_ret_reader.lines() {
        match line {
            Ok(input_line) => {
                let input_fields = input_line.split('|').collect::<Vec<&str>>();
                tot_amt += str_to_flt(input_fields[19]);
                tot_rec += 1;
                output_map = get_output_map(output_map, input_fields, &log);
            }
            Err(error) => {
                skp_rec += 1;
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    for (keypair, data) in output_map.drain() {
        let account = create_account(&config_params, &keypair, &data);
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account.as_on_date
            )],
            writer.write(account)
        );
        write!(
            output_file,
            "{}|{}|{}",
            &config_params.as_on_date(),
            &keypair,
            &data
        )
        .expect("Unable to write txt file.");
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    log_info!(log, "Total time take: {:?}", total_duration);
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&op_path);
}
