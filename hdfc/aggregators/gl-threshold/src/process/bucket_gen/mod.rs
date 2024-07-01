use self::structs::{Account, GrpData, OPKey, OPVal};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::{flt_to_str, str_to_flt, str_to_int, structs};
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
    op_map: &HashMap<OPKey, OPVal>,
    prod_rpt_map: &HashMap<i64, GrpData>,
    is_active_map: &mut HashMap<String, String>,
    grp_amt_map: &HashMap<String, f64>,
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
        let mut is_file_empty: bool = true;
        let mut file_count = 0;
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
            is_file_empty = false;
            let mut record = line.expect("Cannot read line from input file.");
            total_account += 1;
            let mut fields: Vec<String> = record.split('|').map(|s| s.to_string()).collect();
            if fields[4] == "IRS" || fields[4] == "INT" {
                continue;
            }
            if fields[4] == "SLR" {
                fields[4] = "ALL".to_string();
            }
            let llg = fields[0].to_string();
            if !llg_mapping.contains_key(&str_to_int(llg.as_str())) {
                write_output(vec_to_str(&fields).as_str(), &mut writer, logger);
                continue;
            }
            let grp_id = llg_mapping
                .get(&str_to_int(llg.as_str()))
                .expect("cant fetch data for llg");
            let grp_data = prod_rpt_map
                .get(&grp_id)
                .expect("cant fetch data for group id");
            let ccy = fields[2].to_string();
            let grp_amt_key = grp_id.to_string() + &"|".to_string() + &ccy.to_string();
            let grp_amt = grp_amt_map
                .get(&grp_amt_key)
                .expect("cannot fetch amt for grp id ccy combo");
            let ratio = if grp_amt > &grp_data.limit {
                1.0 - (grp_data.limit / grp_amt)
            } else {
                0.0
            };
            if file_index == 0 || file_index == 6 {
                is_active_map.insert(
                    llg.to_string() + &"|".to_string() + &ccy.to_string(),
                    "Y".to_string(),
                );
            }
            for bucket_no in (header_size..=bucket_count[file_index]).step_by(3) {
                let bucket_amt = str_to_flt(fields[bucket_no].as_str());
                if bucket_amt == 0.0 {
                    continue;
                }
                let updated_amt = bucket_amt * ratio;
                total_input_amount += bucket_amt;
                fields[bucket_no] = updated_amt.to_string();
            }
            total_success_account += 1;
            // writing output for llg
            let mut op = vec_to_str(&fields);
            op.pop();
            op.push_str("|0.00|0.00|0.00|0.00\n");
            write_output(&op, &mut writer, logger);
        }
        if is_file_empty {
            continue;
        }
        for (key, value) in op_map {
            let llg = key.llg.to_string();
            let is_active_key = llg + &"|".to_string() + &key.ccy.to_string();
            if is_active_map.contains_key(&is_active_key) {
                let is_active = is_active_map
                    .get(&is_active_key)
                    .expect("Cant get active status for llg");
                if is_active == "Y" {
                    continue;
                }
            }
            write!(
                writer,
                "{}|{}|{}|{}|{}|",
                key.llg, key.as_on, key.ccy, key.field5, key.field4,
            )
            .expect("summary file writing error");
            let mut empty_bkt: Vec<String> = Vec::with_capacity(bucket_count[file_index]);
            for bkt_no in 0..bucket_count[file_index] {
                if bkt_no == 0 && (file_index == 0 || file_index == 6) {
                    empty_bkt.push(value.amt.to_string());
                } else {
                    empty_bkt.push("0.00".to_string());
                }
            }
            write_output(vec_to_str(&empty_bkt).as_str(), &mut writer, logger);
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
