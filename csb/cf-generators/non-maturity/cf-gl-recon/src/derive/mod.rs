use self::account::Account;
use self::account_appender::create_account_without_cashflows;
use self::account_writer::AccountWithoutCashflows;
use self::{alm_master::*, derive_fields::*};
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

mod account;
mod account_appender;
mod account_writer;
mod alm_master;
mod derive_fields;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut writer = AccountWithoutCashflows::new(config_param.output_file_path(), log);

    let mut alm_master: HashMap<AlmMasterKey, AlmMaster> = HashMap::new();
    let mut alm_master_excel: Xlsx<_> =
        open_workbook(config_param.alm_master_file()).expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) = alm_master_excel.worksheet_range(config_param.alm_master_sheet_name())
    {
        for row in reader.rows() {
            get_alm_master_data(row, &mut alm_master);
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

        let record = record.trim();
        let fields: Vec<&str> = record.split('|').collect();
        ttl_acc_encntrd += 1;
        if fields.len() != 10 {
            skp_acc += 1;
            continue;
        }

        let mut alm_master_key = AlmMasterKey::new();
        if fields[2].parse().unwrap_or(DEFAULT_FLOAT) < 0.0 {
            alm_master_key.insert(fields[9].to_string(), String::from("D"));
        } else {
            alm_master_key.insert(fields[9].to_string(), String::from("C"));
        };
        let def_alm_master = AlmMaster::new();
        let mut alm_master_data = alm_master.entry(alm_master_key).or_insert(def_alm_master);

        let account: Account = create_account_without_cashflows(&fields, &mut alm_master_data);

        ttl_amt_in_inp += fields[9].parse().unwrap_or(DEFAULT_FLOAT);
        ttl_amt_in_op += account.gl_diff_amt;

        output_acc_info.push_str(get_output_line(&fields, &mut alm_master_data).as_str());
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
