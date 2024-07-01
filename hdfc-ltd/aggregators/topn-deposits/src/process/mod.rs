use self::io::*;
use self::structs::CustDet;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::time::SystemTime;

mod io;
mod structs;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let output_path = config_params.output_file_path();
    let mut op_writer = get_writer(&output_path);

    let ex_rates = get_exchange_rates(config_params.ex_rate_file_path(), config_params.base_ccy(), &logger);

    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    let mut input_map: HashMap<String, CustDet> = HashMap::new();
    let td_reader = fs::read_to_string(&config_params.td_input_file_path())
        .expect("Failed to read TD-Input file!");
    for line in td_reader.lines() {
        let td_fields: Vec<&str> = line.split('|').collect();
        if td_fields.len()!=43{
            log_error!(
                logger,
                "line skipped for Account Number: `{}` in TD_Input_File.",
                td_fields[0],
            );
            continue;
        }
        let mut custdata = CustDet::new();
        custdata.curr = td_fields[4].to_string();
        custdata.amt = td_fields[14].to_string().parse::<f64>().unwrap();
        input_map
            .entry(td_fields[2].to_string())
            .and_modify(|val| val.update_amt(&mut custdata))
            .or_insert(custdata);
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
    }

    let rd_reader = fs::read_to_string(&config_params.rd_input_file_path())
        .expect("Failed to read RD-Input file!");
    for line in rd_reader.lines() {
        let rd_fields: Vec<&str> = line.split('|').collect();
        if rd_fields.len()!=43{
            log_error!(
                logger,
                "line skipped for Account Number: `{}` in RD_Input_File.",
                rd_fields[0],
            );
            continue;
        }
        let mut custdata = CustDet::new();
        custdata.curr = rd_fields[4].to_string();
        custdata.amt = rd_fields[14].to_string().parse::<f64>().unwrap();
        input_map
            .entry(rd_fields[2].to_string())
            .and_modify(|val| val.update_amt(&mut custdata))
            .or_insert(custdata);
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
    }

    for (custid, custdata) in input_map {
        let base_amt = custdata.amt * (ex_rates.get(&custdata.curr).unwrap_or(&1.0));
        tot_amt += base_amt;
        write!(
            op_writer,
            "{}|{}|{}|{}|{}|||||{}|{}\n",
            config_params.country_code(),
            config_params.as_on_date().format("%d-%m-%Y"),
            custdata.curr,
            custid,
            custid,
            custdata.amt,
            base_amt,
        );
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt.into(),
        tot_amt.into(),
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for processing: {:?}", total_duration);
}

//this function gets exchange rates for INR -> Anything
pub fn get_exchange_rates(exchange_rate_file: &str, base_ccy: &str, logger: &Logger) -> HashMap<String, f64> {
    let mut exchange_rates: HashMap<String, f64> = HashMap::new();
    let rdr = match new_buf_rdr(exchange_rate_file) {
        Ok(r) => r,
        Err(e) => panic!(
            "{}",
            format!(
                "Cannot read file at path: '{}', Error: '{}'",
                exchange_rate_file, e
            )
        ),
    };
    let mut line_num = 1;
    for line in rdr.lines() {
        if let Ok(each_line) = line {
            let line_contents: Vec<&str> = each_line.split("|").collect();
            if line_contents.len() < 3 {
                log_warn!(
                    logger,
                    "ex_rate skipped at line_number: `{}`.",
                    line_num,
                );
                continue;
            }
            if line_contents[1].eq(base_ccy) {
                exchange_rates.insert(
                    line_contents[0].to_string(),
                    line_contents[2].parse().unwrap_or(0.0),
                );
            }
        }
        line_num+=1;
    }
    exchange_rates
}
