use self::account::Account;
use self::account_appender::create_account_without_cashflows;
use self::account_writer::AccountWithoutCashflows;
use self::derive_fields::get_output_line;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::time::SystemTime;

mod account_appender;
mod derive_fields;
mod account;
mod account_writer;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut writer = AccountWithoutCashflows::new(config_param.output_file_path(), log);

    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.alm_master_file())
        .expect("Error while opening `ALM Master File`.");
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            ia_llg.insert(row[2].to_string(), row[7].to_string());
            balm_llg.insert(row[2].to_string(), row[9].to_string());
        }
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_process_timer = SystemTime::now();
    let mut ttl_amt_in_inp = DEFAULT_FLOAT;
    let mut ttl_amt_in_op = DEFAULT_FLOAT;
    let mut output_acc_info: String = String::new();
    let mut ttl_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_acc: i64 = DEFAULT_INT;
    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to open the file: `{}`. : {:?}",
            config_param.input_file(),
            error
        ),
    };

    let exclude_file = match new_buf_rdr(config_param.master_exclude_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to open the file: `{}`. : {:?}",
            config_param.master_exclude_file(),
            error
        ),
    };
    let mut exclude_fields: Vec<String> = Vec::new();
    for (line_num, line) in BufReader::new(exclude_file).lines().enumerate() {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!(
                    "Error while reading the file: `{}` at line number: `{}`. : {:?}",
                    config_param.master_exclude_file(),
                    line_num + 1,
                    error
                );
            }
        };
        exclude_fields.push(record.to_string());
    }
    let mut output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Error while creating file: `{}`. : {:?}",
            config_param.output_file_path(),
            error
        ),
    };

    for (line_num, line) in BufReader::new(input_file).lines().enumerate() {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!(
                    "Error while reading the file: `{}` at line number: `{}`. : {:?}",
                    config_param.input_file(),
                    line_num + 1,
                    error
                );
            }
        };

        let fields: Vec<&str> = record.split('|').collect();
        ttl_acc_encntrd += 1;
        if exclude_fields.contains(&fields[6].to_string()) {
            skp_acc += 1;
            continue;
        }
        if fields.len() != 10 {
            skp_acc += 1;
            continue;
        }

        let balm_llg = balm_llg
            .entry(fields[6].to_string())
            .or_insert_with(|| "NONE".to_string());

        let ia_llg = ia_llg
            .entry(fields[6].to_string())
            .or_insert_with(|| "NONE".to_string());

        let account: Account =
            create_account_without_cashflows(&fields, balm_llg.to_string(), ia_llg.to_string());

        ttl_amt_in_inp += fields[9].parse().unwrap_or(DEFAULT_FLOAT);
        ttl_amt_in_op += account.gl_diff_amt;

        log_debug!(
            log,
            "gl: `{}`, ia_llg: `{}` and balm_llg: `{}`.",
            fields[6],
            ia_llg,
            balm_llg
        );

        output_acc_info.push_str(get_output_line(&fields, balm_llg, ia_llg).as_str());

        write!(output_file, "{}", output_acc_info).expect("Error while writing output line.");

        writer.write(account);
        output_acc_info.clear();
    }
    writer.close();

    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for deriving fields and writing output.");
    debug!(
        diag_log,
        "Total Duration for deriving fields and writing output: {:?}.", duration
    );

    let report_string = format!(
        "Records encountered: {}\n\
         Records proccessed suceessfully: {}\n\
         Records failed to process: {}\n\
         Total Amount in input: {}\n\
         Total Amount in output: {}",
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt_in_inp,
        ttl_amt_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_report = HealthReport::new(
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt_in_inp,
        ttl_amt_in_op,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
