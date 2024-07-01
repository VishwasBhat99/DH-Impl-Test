use self::structs::OutputAccount;
use self::structs::{format_op_rec, get_op_data};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

pub mod structs;

pub fn pre_processor(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    // Read Master Input File
    let input_master_file = match File::open(config_params.input_master_file_path()) {
        Ok(master_file) => master_file,
        Err(error) => panic!("{}", error),
    };
    let master_reader = BufReader::new(input_master_file);
    // Read Cashflow Input File
    let input_cashflow_file = match File::open(config_params.input_cashflow_file_path()) {
        Ok(cashflow_file) => cashflow_file,
        Err(error) => panic!("{}", error),
    };
    let cashflow_reader = BufReader::new(input_cashflow_file);
    // Read Ledger Master File
    let input_ledger_file = match File::open(config_params.input_ledger_file_path()) {
        Ok(ledger_file) => ledger_file,
        Err(error) => panic!("{}", error),
    };
    let ledger_reader = BufReader::new(input_ledger_file);
    // Init Output File Writer
    let output_file = match File::create(config_params.output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let mut op_writer = BufWriter::new(output_file);
    // Read cashflow data in memory
    let mut cf_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cf_rec_count = 0;
    for (line_no, line) in cashflow_reader.lines().enumerate().skip(1) {
        cf_rec_count += 1;
        let cf_info: String = match line {
            Ok(cf_info) => cf_info,
            Err(error) => {
                panic!("Cannot read line {} from input file: {:?}", line_no, error);
            }
        };
        let cf_fields: Vec<&str> = cf_info.split('|').collect();
        let acc_id = cf_fields[0].to_string();
        if cf_fields.len()!=5{
            error!(logger,
                "Cannot read line {} from cashflow file for acc_no: {:?}",
                line_no,
                acc_id);
            continue;
        }
        cf_map
            .entry(acc_id)
            .and_modify(|data| data.push(cf_info.to_string()))
            .or_insert([cf_info].to_vec());
    }
    log_info!(
        logger,
        "Number of records read from cashflow file: {}",
        cf_rec_count
    );

    // Read Ledger data in memory
    let mut ledger_map: HashMap<String, Vec<String>> = HashMap::new();
    for (line_no, line) in ledger_reader.lines().enumerate().skip(*config_params.cf_header_count()) {
        let ledger_info: String = match line {
            Ok(ledger_info) => ledger_info,
            Err(error) => {
                panic!("Cannot read line {} from input file: {:?}", line_no, error);
            }
        };
        let ledger_fields: Vec<&str> = ledger_info.split('|').collect();
        let gl_code = ledger_fields[0].to_string();
        ledger_map
            .entry(gl_code)
            .and_modify(|data| data.push(ledger_info.to_string()))
            .or_insert([ledger_info].to_vec());
    }

    // Process Master Input File
    let mut master_rec_count = 0;
    let mut skip_rec_count = 0;
    let mut total_amt_ip = 0.0;
    let mut total_amt_op = 0.0;
    let date_parser = DateParser::new("%d-%b-%Y".to_string(), false);
    for (line_no, line) in master_reader.lines().enumerate().skip(*config_params.master_header_count()) {
        master_rec_count += 1;
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                skip_rec_count += 1;
                log_error!(
                    diag_logger,
                    "Cannot read line {} from input file: {:?}",
                    line_no,
                    error
                );
                continue;
            }
        };
        let acc_fields: Vec<String> = acc_info.split('|').map(|s| s.to_string()).collect();
        let acc_id = acc_fields[0].to_string();
        if acc_fields.len()!=35{
            skip_rec_count += 1;
            error!(logger,
                "Cannot read line {} from input file for acc_no: {:?}",
                line_no,
                acc_id);
            continue;
        }

        let mut lm_alm_line = "NA";
        if ledger_map.contains_key(&acc_fields[5]){
            let ledger_fields: Vec<&str> = ledger_map[&acc_fields[5]][0].split('|').collect();
            lm_alm_line = ledger_fields[2];
        }

        let mut op_data: OutputAccount = get_op_data(
            acc_fields,
            config_params.as_on_date(),
            lm_alm_line.to_string(),
        );
        total_amt_ip += op_data.prin_ost_bal.parse::<f64>().unwrap_or(0.0);
        match cf_map.get(&acc_id) {
            Some(cf_data_set) => {
                for rec in cf_data_set {
                    let cf_fields: Vec<&str> = rec.split('|').collect();
                    op_data.component = "PRINCIPAL".to_string();
                    op_data.due_dt = date_parser
                        .parse_opt(&cf_fields[2])
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string();
                    op_data.cf_amt = cf_fields[3].to_string();
                    total_amt_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                    // write principal cf data
                    let prin_cf = format_op_rec(&op_data);
                    op_writer
                        .write(prin_cf.as_bytes())
                        .expect("Error writing principal cf to output path!!");
                    op_data.component = "MAIN_INT".to_string();
                    op_data.due_dt = date_parser
                        .parse_opt(&cf_fields[2])
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string();
                    op_data.cf_amt = cf_fields[4].to_string();
                    total_amt_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                    // write interest cf data
                    let int_cf = format_op_rec(&op_data);
                    op_writer
                        .write(int_cf.as_bytes())
                        .expect("Error writing interest cf to output path!!");
                }
            }
            None => {
                let frozen_acc_data = format_op_rec(&op_data);
                total_amt_op += op_data.cf_amt.parse::<f64>().unwrap_or(0.0);
                // write frozen account data
                op_writer
                    .write(frozen_acc_data.as_bytes())
                    .expect("Error writing frozon account data to output path!!");
            }
        };
    }
    log_info!(
        logger,
        "Number of records read from master file: {}",
        master_rec_count
    );
    // Flush Output Writer
    op_writer
        .flush()
        .expect("Error while flushing data from writer buffer!!");
    // Generate Health Check Report
    let health_report = HealthReport::new(
        master_rec_count,
        master_rec_count - skip_rec_count,
        skip_rec_count,
        total_amt_ip,
        total_amt_op,
        cf_rec_count,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
