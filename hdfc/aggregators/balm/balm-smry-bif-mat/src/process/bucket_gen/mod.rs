use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::{flt_to_str, str_to_flt, str_to_int};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::{Duration, SystemTime};
pub fn bucket_gen(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    llg_mapping: &HashMap<i64, i64>,
    llg_distribution_ration: &HashMap<i64, f64>,
) {
    let bucket_count = vec![499, 499, 112, 499, 499, 67, 499, 499, 112, 499, 499, 67, 8];
    let mut input_file_name: String;
    let mut output_file_name: String;
    let mut init_file_idx = 0;
    let mut total_account = 0;
    let mut total_failed_account = 0;
    let mut total_success_account = 0;
    let mut total_input_amount = 0.0;
    let mut total_output_amount = 0.0;
    let start_time = SystemTime::now();
    for file_index in init_file_idx..=11 {
        let mut header_size = 5;
        input_file_name = format!("{}-{}.txt", config_params.input_file_path(), file_index);
        output_file_name = format!(
            "{}-converted-{}.txt",
            config_params.output_file_path(),
            file_index
        );

        let input_file = match new_buf_rdr(&input_file_name) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot read file at path: '{}', Error: '{}'",
                input_file_name, e
            )),
        };
        log_info!(logger, "PROCESSING {} file", input_file_name);

        let mut writer = match buf_file_wrtr(&output_file_name, None) {
            Ok(r) => r,
            Err(e) => panic!(format!(
                "Cannot write to file at path: '{}', Error: '{}'",
                output_file_name, e
            )),
        };
        for line in BufReader::new(input_file).lines() {
            let mut record = line.expect("Cannot read line from input file.");
            total_account += 1;
            let fields: Vec<String> = record.split('|').map(|s| s.to_string()).collect();
            let mut llg1_data = fields.clone();
            let mut llg2_data = fields.clone();
            let llg1 = str_to_int(llg1_data[0].as_str());
            let llg2 = *llg_mapping.get(&llg1).unwrap_or(&0);
            let do_biforcate = llg_distribution_ration.contains_key(&llg1);
            if !do_biforcate {
                record.push('\n');
                write_output(record.as_str(), &mut writer, logger);
                continue;
            }
            let llg1_ratio = *llg_distribution_ration.get(&llg1).unwrap_or(&0.0);
            let llg2_ratio = *llg_distribution_ration.get(&llg2).unwrap_or(&0.0);
            for bucket_no in (header_size..=bucket_count[file_index]).step_by(3) {
                let total_amount = str_to_flt(fields[bucket_no].as_str());
                total_input_amount += total_amount;
                let llg1_updated_amount = llg1_ratio * total_amount;
                let llg2_updated_amount = llg2_ratio * total_amount;
                total_output_amount += llg1_updated_amount + llg2_updated_amount;
                llg1_data[bucket_no] = llg1_updated_amount.to_string();
                llg2_data[bucket_no] = llg2_updated_amount.to_string();
            }
            total_success_account += 1;
            // writing output for llg1
            write_output(vec_to_str(&llg1_data).as_str(), &mut writer, logger);
            // check if llg2 exist(skip cases when llg_mapping was not found)
            if !llg2.eq(&0) {
                llg2_data[0] = llg2.to_string();
                write_output(vec_to_str(&llg2_data).as_str(), &mut writer, logger);
            } else {
                print!(",   skipped llg2 {} data", llg2);
                log_warn!(
                    logger,
                    "skipping account, llg mapping not found for : {}",
                    llg1
                );
            }
        }
    }
    let health_report = HealthReport::new(
        total_account,
        total_success_account,
        total_failed_account,
        total_input_amount,
        total_output_amount,
        0,
    );
    log_info!(logger, "{}", health_report.display());
    log_info!(logger, "total time for mat : {:#?}", start_time.elapsed());

    println!("{}", health_report.display());
    println!(
        "total time for mat :{:?}",
        start_time.elapsed().unwrap_or(Duration::new(0, 0))
    );
}

fn write_output(account: &str, writer: &mut BufWriter<File>, logger: &Logger) {
    match writer.write_all(account.as_bytes()) {
        Ok(_) => {}
        Err(e) => log_error!(logger, "unable write output to output file: `{}`", e),
    }
}

fn vec_to_str(fields: &Vec<String>) -> String {
    let mut output_line = String::new();
    for i in fields {
        output_line.push_str(i);
        output_line.push('|');
    }
    output_line.pop();
    output_line.push('\n');
    output_line
}
