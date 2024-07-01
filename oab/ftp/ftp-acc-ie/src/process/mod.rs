use self::account::FTPAccIEAmt;
use self::account_field_names::AccFieldNames;
use self::io::get_writer;
use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use sdb_dyn_proto_rdr::reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::io::Write;

mod account;
mod account_field_names;
mod io;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_acc_encntrd = 0;
    let mut tot_amt = 0.0;
    let mut skip_rec_count = 0;

    //Init Output Writer
    let mut op_writer = get_writer(config_params.output_file_path());

    //Reading Exchange Rate File
    let mut exrt_map: HashMap<String, f64> = HashMap::new();
    let exrt_file = read_file(&config_params.exrt_file_path);
    for (line_num, lines) in exrt_file.lines().enumerate() {
        let line = extract_lines(line_num, lines, &config_params.exrt_file_path);
        let fields: Vec<&str> = line.split('|').collect();
        let key: String = fields[0].to_string() + "|" + fields[1];
        let val: f64 = fields[2]
            .to_string()
            .parse::<f64>()
            .expect("could not read exchange rate");
        exrt_map.insert(key, val);
    }

    //Reading FTPAccIE File
    let ftp_accie = File::open(&config_params.ftp_accie_file_path())
        .expect("Could Not Read Input FTPAccIE File");
    let ftp_accie_reader = BufReader::new(ftp_accie);
    let mut ftp_accie_map: HashMap<String, FTPAccIEAmt> = HashMap::new();
    for (line_no, line) in ftp_accie_reader.lines().enumerate() {
        let ftp_accie_info: String = match line {
            Ok(ftp_accie_info) => ftp_accie_info,
            Err(error) => {
                log_error!(
                    log,
                    "Cannot read line {} from FTPAccIE file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        if ftp_accie_info.matches('|').count() < 3 {
            log_error!(
                log,
                "Cannot read line {} from FTPAccIE file at line_no: {:?}",
                ftp_accie_info,
                line_no
            );
            continue;
        }
        let ftp_accie_fields: Vec<&str> = ftp_accie_info.split('|').collect();
        let acc_id = ftp_accie_fields[0].to_string();

        let ftp_accie_val = FTPAccIEAmt::new(ftp_accie_fields);
        if ftp_accie_map.contains_key(&acc_id) {
            if config_params.is_perf_diagnostics_enabled() {
                log_warn!(
                    log,
                    "Skipped accid `{:?}` record at line_no`{:?}` from FTPAccIE File",
                    acc_id,
                    line_no + 1
                );
            }
            continue;
        }
        ftp_accie_map.entry(acc_id).or_insert(ftp_accie_val);
    }
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(_) => panic!("Could not find the input file"),
    };
    // Read Input file as per metadata.
    let account_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut field_names = Vec::new();
    let metadata_reader = fs::read_to_string(&config_params.metadata_file_path())
        .expect("Failed to read metadata file!");
    //Fetch the names from the metadata file.
    for line in metadata_reader.lines() {
        if line.contains("name") {
            let fields: Vec<&str> = line.split(':').collect();
            let mut name = fields[1].to_string();
            name.pop();
            name.pop();
            name = name[2..].to_string();
            field_names.push(name);
        }
    }
    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num + 1,
                    error
                );
                skip_rec_count += 1;
                continue;
            }
        };
        tot_acc_encntrd += 1;
        let input_fields: Vec<&str> = line.split('|').collect();
        tot_acc_encntrd += 1;
        let acc_id = match account_reader.get_field_pos(&keys.account_id) {
            Some(val) => input_fields[val - 1].to_string(),
            None => panic!("unable to get `account_id` value"),
        };
        let acr_int_amt_ccy_pos = match account_reader.get_field_pos(&keys.acr_int_amt_ccy) {
            Some(val) => val,
            None => panic!("unable to get 'acr_int_amt_ccy` position value"),
        };
        let mut acr_int_amt_ccy = input_fields[acr_int_amt_ccy_pos - 1]
            .parse::<f64>()
            .unwrap_or(0.0);
        let acr_int_amt_hcy_pos = match account_reader.get_field_pos(&keys.acr_int_amt_hcy) {
            Some(val) => val,
            None => panic!("unable to get 'acr_int_amt_hcy` position value"),
        };
        let mut acr_int_amt_hcy = input_fields[acr_int_amt_hcy_pos - 1]
            .parse::<f64>()
            .unwrap_or(0.0);
        let bal_amt_hcy = match account_reader.get_field_pos(&keys.bal_amt_ccy) {
            Some(val) => input_fields[val - 1].parse::<f64>().unwrap_or(0.0),
            None => 0.0,
        };
        let ccy_id = match account_reader.get_field_pos(&keys.ccy_id) {
            Some(val) => input_fields[val - 1],
            None => panic!("unable to get `ccy_id` value"),
        };
        let ftp_accie;
        let exrt_key = ccy_id.to_string() + "|" + config_params.base_ccy();
        let exrt = *exrt_map.get(&exrt_key).unwrap();
        tot_amt += bal_amt_hcy;
        //lookup and get the act_int_amt_ccy/hcy columns stamped
        if ftp_accie_map.contains_key(&acc_id) {
            ftp_accie = ftp_accie_map.get(&acc_id).unwrap();
            if config_params.interest_income().contains(&ftp_accie.source) {
                acr_int_amt_ccy = ftp_accie.interest_income;
                acr_int_amt_hcy = acr_int_amt_ccy * exrt;
            } else if config_params.interest_expense().contains(&ftp_accie.source) {
                acr_int_amt_ccy = ftp_accie.interest_expense;
                acr_int_amt_hcy = acr_int_amt_ccy * exrt;
            }
        } else {
            log_error!(log, "Account: `{}` not found in FTPAccIE File", acc_id);
        }
        let mut output_line = String::new();
        for index in 0..input_fields.len() {
            if index == acr_int_amt_ccy_pos - 1 {
                output_line.push_str(&acr_int_amt_ccy.to_string());
                output_line.push_str("|");
                continue;
            }
            if index == acr_int_amt_hcy_pos - 1 {
                output_line.push_str(&acr_int_amt_hcy.to_string());
                output_line.push_str("|");
                continue;
            }
            output_line.push_str(&input_fields[index].to_string());
            output_line.push_str("|");
        }
        output_line.pop();
        output_line.push_str("\n");
        op_writer
            .write_all(output_line.as_bytes())
            .expect("Error writing account data to output path!!");
        if config_params.is_perf_diagnostics_enabled() {
            log_debug!(_diag_log, "{} line written to output-file", output_line);
        }
    }
    // Flush Output Writer
    op_writer
        .flush()
        .expect("Error while flushing data from writer buffer!!");

    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        tot_amt,
        tot_amt,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
