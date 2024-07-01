extern crate serde;
use self::format::get_op_line;
use self::format::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod format;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {

    let output_file_irs_swap = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut tot_acc = 0;
    let mut tot_succ = 0;
    let mut ttl_ip_amt = 0.0;
    let mut ttl_op_amt = 0.0;
    let mut tot_cfs = 0;

    let mut writer_irs_swap = BufWriter::new(output_file_irs_swap);
    let cashflow_file = match new_buf_rdr(config_param.cashflow_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.cashflow_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let start_timer = SystemTime::now();
    let mut cashflow_map: HashMap<String, Vec<CashflowData>> = HashMap::new();
    for (line_num, lines) in cashflow_file.lines().enumerate() {
        let cashflow_line = match lines {
            Ok(cashflow_line) => cashflow_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cashflow_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = cashflow_line.split('|').collect();
        let cashflow = get_cashflow_data(fields.clone());
        cashflow_map
            .entry(cashflow.deal_number.to_owned())
            .and_modify(|cf| cf.push(cashflow.clone()))
            .or_insert(vec![cashflow]);
    }

    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate CashflowData file processing duration.");
    log_debug!(log, "CashflowData file derive duration:{:?}", duration);

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input data file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_no, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(data) => data,
            Err(error) => panic!(
                "Unable to read file `{}` at line no:{}: {}",
                config_param.input_file_path(),
                line_no + 1,
                error
            ),
        };
        tot_acc += 1;
        let ip_fields: Vec<&str> = line.split('|').collect();
        let input_data = get_input_data(ip_fields.clone());
        ttl_ip_amt+=input_data.deal_amt;

        let default_cf_value: Vec<CashflowData> = vec![CashflowData {
            cf_date: input_data.due_date.to_owned(),
            as_on_date: *config_param.as_on_date(),
            deal_number: input_data.deal_ref.to_owned(),
            cf_type: "PRINCIPAL".to_string(),
            cf_sub_type: input_data.pay_recv.to_owned(),
            cf_amount: input_data.deal_amt.to_owned(),
            currency: "INR".to_string(),
        }];

        let cf_vec = match cashflow_map.get(&input_data.deal_ref.to_owned()) {
            Some(cashflow_vec) => cashflow_vec,
            None => &default_cf_value,
        };

        for cf in cf_vec.iter() {
            let op_line = get_op_line(input_data.clone(), cf);
            match writer_irs_swap.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
            tot_cfs+=1;
            ttl_op_amt+=cf.cf_amount;
        }
        tot_succ += 1;
    }
    let health_report = HealthReport::new(
        tot_acc,
        tot_succ,
        tot_acc - tot_succ,
        ttl_ip_amt,
        ttl_op_amt,
        tot_cfs,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
