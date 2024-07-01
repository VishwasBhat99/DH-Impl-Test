extern crate serde;
use crate::pre_processor::format::NPAFields;

use self::format::get_op_line;
use chrono::NaiveDate;
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

#[derive(Debug, Clone, Copy)]
pub struct CashFlow {
    interest_amt: f64,
    principal_amt: f64,
    cf_date: NaiveDate,
}
pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut tot_acc = 0;
    let mut tot_succ = 0;
    let mut ttl_amt = 0.0;
    let mut tot_cfs = 0;
    let mut writer = BufWriter::new(output_file);
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
    let mut cashflow_map: HashMap<String, Vec<CashFlow>> = HashMap::new();
    let as_on_date = config_param.as_on_date();
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
        let cashflow = CashFlow {
            interest_amt: fields[2].parse::<f64>().unwrap_or(0.0),
            principal_amt: fields[3].parse::<f64>().unwrap_or(0.0),
            cf_date: NaiveDate::parse_from_str(fields[1], "%d-%m-%Y").unwrap_or(
                NaiveDate::from_ymd_opt(1900, 1, 1).unwrap_or(*config_param.as_on_date()),
            ),
        };
        cashflow_map
            .entry(fields[0].to_string())
            .and_modify(|cf| cf.push(cashflow))
            .or_insert(vec![cashflow]);
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate Cashflow file processing duration.");
    log_debug!(log, "Cashflow file derive duration:{:?}", duration);

    //Sort the cashflows based on cf-date.
    for (_, cashflow_list) in cashflow_map.iter_mut() {
        cashflow_list.sort_by(|a, b| a.cf_date.cmp(&b.cf_date));
    }

    let npa_file = match new_buf_rdr(config_param.npa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.npa_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut npa_map: HashMap<String, NPAFields> = HashMap::new();
    for (line_num, lines) in npa_file.lines().enumerate() {
        let npa_line = match lines {
            Ok(npa_line) => npa_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = npa_line.split('|').collect();
        npa_map.insert(fields[0].to_string(), NPAFields::get_npa_fields(fields));
    }

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
    let deafult_npa = NPAFields::new();
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
        let fields: Vec<&str> = line.split('|').collect();
        //Match the output format to that of cf_ubs_loans ltd.
        let acc_no = fields[0].to_string();
        let default_cf_vec = vec![CashFlow {
            interest_amt: 0.0,
            principal_amt: fields[6].parse::<f64>().unwrap_or(0.0),
            cf_date: NaiveDate::parse_from_str(fields[8], "%d-%m-%Y").unwrap_or(
                NaiveDate::from_ymd_opt(1900, 1, 1).unwrap_or(*config_param.as_on_date()),
            ),
        }];
        let cf_vec = match cashflow_map.get(&acc_no) {
            Some(cashflow_vec) => cashflow_vec,
            None => &default_cf_vec,
        };
        let last_inst_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1900, 1, 1));

        let npa_fields = npa_map.get(&fields[0].to_string()).unwrap_or(&deafult_npa);
        for cf in cf_vec.iter() {
            //Write principal cashflow.
            let component = "PRINCIPAL";
            let cf_amt = cf.principal_amt;
            ttl_amt += cf.principal_amt;
            tot_cfs += 1;
            //Condition to write cashflows.
            if cf.cf_date > last_inst_date {
                let op_line = get_op_line(&fields, cf, component, *as_on_date, cf_amt, &npa_fields);
                match writer.write_all(op_line.as_bytes()) {
                    Ok(val) => val,
                    Err(error) => {
                        panic!("Error writing processed data: {:?}", error);
                    }
                }
                //Write interest cashflow.
                let component = "MAIN_INT";
                let cf_amt = cf.interest_amt;
                let op_line = get_op_line(&fields, cf, component, *as_on_date, cf_amt, &npa_fields);
                match writer.write_all(op_line.as_bytes()) {
                    Ok(val) => val,
                    Err(error) => {
                        panic!("Error writing processed data: {:?}", error);
                    }
                }
            }
        }
        tot_succ += 1;
    }
    let health_report = HealthReport::new(
        tot_acc,
        tot_succ,
        tot_acc - tot_succ,
        ttl_amt,
        ttl_amt,
        tot_cfs,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
