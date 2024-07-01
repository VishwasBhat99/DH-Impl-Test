extern crate serde;
use self::account::*;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::num_days_start_to_end;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;

mod account;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut tot_acc = 0;
    let mut tot_succ = 0;
    let mut ttl_ip_amt = 0.0;
    let mut ttl_op_amt = 0.0;
    let mut tot_cfs = 0;
    let mut writer = BufWriter::new(output_file);
    let mut currency_prrate_map: HashMap<String, Vec<(i64, f64)>> = HashMap::new();
    let currency_master_reader = fs::read_to_string(config_param.currency_prrate_file_path())
        .expect("Could Not Read Currency pr rate file");

    for (line_no, line) in currency_master_reader.lines().enumerate() {
        let currency_master_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let ccy = get_str(
            config_param.currency_prrate_file_path(),
            &currency_master_vec,
            1,
            line_no,
        );
        let days = get_str(
            config_param.currency_prrate_file_path(),
            &currency_master_vec,
            3,
            line_no,
        )
        .parse::<i64>()
        .unwrap_or(0);
        let rate = get_str(
            config_param.currency_prrate_file_path(),
            &currency_master_vec,
            7,
            line_no,
        )
        .parse::<f64>()
        .unwrap_or(0.0);
        currency_prrate_map
            .entry(ccy.clone())
            .and_modify(|prev_data| prev_data.push((days.clone(), rate)))
            .or_insert(vec![(days, rate)]);
    }

    let mut sorted_currency_prrate_map: HashMap<String, Vec<(i64, f64)>> = HashMap::new();
    for (ccy, mut ele) in currency_prrate_map {
        ele.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_currency_prrate_map.insert(ccy, ele);
    }

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
    let mut cashflow_map: HashMap<String, Vec<Cashflow>> = HashMap::new();
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
        let cf_fields: Vec<&str> = cashflow_line.split('|').collect();
        let cf_data = Cashflow::new(config_param.cashflow_file_path(), &cf_fields, line_num + 1);
        cashflow_map
            .entry(cf_data.deal_num.to_string())
            .and_modify(|cf| cf.push(cf_data.clone()))
            .or_insert(vec![cf_data]);
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate Cashflow file processing duration.");
    log_debug!(log, "Cashflow file derive duration:{:?}", duration);

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
        let fields: Vec<&str> = line.split('|').collect();
        let mut acc = Account::new(config_param.input_file_path(), &fields, line_no + 1);
        ttl_ip_amt += acc.deal_curr_amount;
        match cashflow_map.get(&acc.deal_number.to_string()) {
            Some(cashflows) => {
                for cashflow in cashflows.iter() {
                    tot_cfs += 1;
                    acc.cf_type = cashflow.cf_type.trim().to_uppercase().to_string();
                    acc.cf_sub_type = cashflow.cf_sub_type.to_string();
                    acc.currency = cashflow.currency.to_string();
                    acc.cf_date = cashflow.cf_date.to_string();
                    let as_on_date = NaiveDate::parse_from_str(&acc.asondate, "%d-%m-%Y")
                        .unwrap_or(*config_param.as_on_date());
                    let cf_date =
                        NaiveDate::parse_from_str(&acc.cf_date, "%d-%m-%Y").unwrap_or(as_on_date);
                    let ccy = acc.deal_curr_code.to_string();
                    let def_vec: Vec<(i64, f64)> = Vec::new();
                    let ccy_prrate_vec = sorted_currency_prrate_map
                        .get(&ccy.to_uppercase())
                        .unwrap_or(&def_vec);
                    let resudial_tenor = num_days_start_to_end( as_on_date,cf_date);
                    let mut reval_loss: f64 = 0.0;
                    let ccy_vec_len = ccy_prrate_vec.len();
                    let mut length_counter = 1;
                    if ccy_vec_len != 0 && ccy_prrate_vec[0].0 == resudial_tenor {
                        reval_loss = ccy_prrate_vec[0].1;
                    }
                    let mut flag = false;
                    while length_counter < ccy_vec_len {
                        if ccy_prrate_vec[length_counter].0 >= resudial_tenor {
                            if ccy_prrate_vec[length_counter].0 == resudial_tenor {
                                reval_loss = ccy_prrate_vec[length_counter].1;
                                break;
                            }
                            flag = true;
                            let lower_bound = ccy_prrate_vec[length_counter - 1].0;
                            let upper_bound = ccy_prrate_vec[length_counter].0;
                            let lower_rates = ccy_prrate_vec[length_counter - 1].1;
                            let upper_rates = ccy_prrate_vec[length_counter].1;
                            let rates = upper_rates - lower_rates;
                            let days = upper_bound - lower_bound;
                            let lower_days = resudial_tenor - lower_bound;
                            reval_loss = lower_rates + ((rates / days as f64) * lower_days as f64);
                        }
                        if flag == true {
                            break;
                        }
                        length_counter += 1;
                    }
                    acc.reval_rate = reval_loss.to_string();
                    match cashflow.cf_type.trim().to_uppercase().to_string().as_str() {
                        "PRINCIPAL" => {
                            acc.cf_prin_amount = cashflow.cf_amount;
                            acc.cf_int_amount = 0.0;
                        }
                        "INTEREST" => {
                            acc.cf_prin_amount = cashflow.cf_amount;
                            acc.cf_int_amount = 0.0;
                        }
                        _ => {
                            log_warn!(log, "Invalid CF-Type: {} encountered", cashflow.cf_type);
                        }
                    }
                    ttl_op_amt += cashflow.cf_amount;
                    writeln!(writer, "{}", format_output(acc.clone()))
                        .expect("Error in Writing Output");
                }
            }
            None => {
                ttl_op_amt += acc.deal_curr_amount;
                tot_cfs += 1;
                log_warn!(log, "Could not get cashflows for Deal: {}", acc.deal_number);
                writeln!(writer, "{}", format_output(acc)).expect("Error in Writing Output");
            }
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
