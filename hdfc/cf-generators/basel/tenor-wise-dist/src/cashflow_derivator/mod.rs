use slog::Logger;
mod account;
mod account_appender;
mod account_writer;
mod convert_datatype;
mod get_lcr_output;
mod get_output;
mod io;
mod structs;

use self::io::{get_cf_writer, get_txt_writer};
use crate::cashflow_derivator::account_appender::create_account;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use convert_datatype::str_to_int;
use get_lcr_output::get_lcr_map;
use get_output::get_output_map;
use health_report::HealthReport;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;
use structs::{Data, Key};

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_generator_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut tot_amt = 0.0;

    let op_path = config_params.output_file_path().to_string();
    let output_path = format!("{}.txt", &config_params.output_file_path());
    let mut output_file = get_txt_writer(&output_path);
    let mut writer = get_cf_writer(&op_path, log);

    let wd_cust_file = match File::open(&config_params.wd_cust_file_path()) {
        Ok(wd_cust_file) => wd_cust_file,
        Err(error) => panic!("{}", error),
    };
    let wd_cust_reader = BufReader::new(wd_cust_file);

    let nwd_cust_file = match File::open(&config_params.nwd_cust_file_path()) {
        Ok(nwd_cust_file) => nwd_cust_file,
        Err(error) => panic!("{}", error),
    };
    let nwd_cust_reader = BufReader::new(nwd_cust_file);

    //Hashmap of retail and non-retail input files {Key:custid ,value:classid}
    let lcr_map = get_lcr_map(&config_params);
    let mut output_map: HashMap<Key, Data> = HashMap::new();
    for line in wd_cust_reader.lines() {
        match line {
            Ok(input_line) => {
                let input_fields = input_line.split('|').collect::<Vec<&str>>();
                tot_rec += 1;
                let classid = lcr_map.get(&str_to_int(input_fields[1]));
                output_map = get_output_map(
                    &"WD".to_string(),
                    output_map,
                    input_fields,
                    classid.unwrap_or(&0),
                );
            }
            Err(error) => {
                skp_rec += 1;
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    for line in nwd_cust_reader.lines() {
        match line {
            Ok(input_line) => {
                let input_fields = input_line.split('|').collect::<Vec<&str>>();
                tot_rec += 1;
                let classid = lcr_map.get(&str_to_int(input_fields[1]));
                output_map = get_output_map(
                    &"NWD".to_string(),
                    output_map,
                    input_fields,
                    classid.unwrap_or(&0),
                );
            }
            Err(error) => {
                skp_rec += 1;
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
    }
    for (key, data) in output_map.drain() {
        let account = create_account(&config_params, &key, &data);
        tot_amt += data.tot_amt;
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
            &key,
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
