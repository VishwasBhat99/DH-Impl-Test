use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;

mod io;

pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let start_time = SystemTime::now();
    let output_path = config_params.output_file_path();
    let mut op_writer = get_writer(&output_path);

    let ex_rates = get_exchange_rates(config_params.ex_rate_file_path(), config_params.base_ccy(), &logger);

    let input =
        File::open(&config_params.input_file_path()).expect("Could Not Read Borrowings Input File");
    let input_reader = BufReader::new(input);

    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;
    let mut tot_amt = 0.0;

    for (index, line) in input_reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let borr_amt: f64 = input_fields[7].to_string().parse::<f64>().unwrap();
        let mut borr_amt_lcy = 0.0;

        if input_fields.len()!=26{
            log_error!(
                logger,
                "line skipped for Issuer Number: `{}`.",
                input_fields[0],
            );
            continue;
        }
        else if input_fields[8] != "" || input_fields[8] != "INR" {
            let ex_rate = ex_rates.get(&input_fields[8].to_string());
            borr_amt_lcy = borr_amt * ex_rate.unwrap_or(&0.0);
        } else {
            borr_amt_lcy = borr_amt;
        }
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
        tot_amt += borr_amt_lcy;
        write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}\n",
            config_params.country_code(),
            config_params.as_on_date().format("%d-%m-%Y"),
            input_fields[8],
            input_fields[0],
            input_fields[6],
            borr_amt,
            borr_amt_lcy,
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
