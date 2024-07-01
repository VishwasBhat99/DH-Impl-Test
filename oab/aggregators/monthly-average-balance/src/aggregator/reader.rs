use crate::statics::DEFAULT_FLOAT;

use super::structs::InputBalances;
use super::{
    extract_lines, macros, read_file, AggregateData, ConfigurationParameters, GetDates, HashMap,
    InputAccount, InputParsedAccount, Logger,
};
use chrono::Datelike;
use sdb_io::new_buf_rdr;
use std::fs::{create_dir, metadata, File};
use std::io::BufRead;
use std::path::Path;

pub fn get_data(
    dates: &GetDates,
    config_params: &ConfigurationParameters,
    account_pool: &mut HashMap<String, Vec<f64>>,
    log: &Logger,
    bal_org: &mut HashMap<String, AggregateData>,
) {
    let mut ip_avg_bal: HashMap<String, InputBalances> = HashMap::new();
    let mut nxt_aggr_dt = dates.start_date;
    let date_format = config_params.date_format();
    let as_on_dt = config_params.as_on_date().format(date_format).to_string();
    let mut day: usize = 1;
    while nxt_aggr_dt <= dates.end_date {
        let date_folder_name = nxt_aggr_dt.format(date_format).to_string();
        let inp_file_path = config_params
            .input_file_path()
            .replace(&as_on_dt, &date_folder_name);
        let empty_file_path = inp_file_path.to_string();
        if !metadata(inp_file_path.to_string()).is_ok() {
            log_error!(
                log,
                "Average Balance File: `{}` not available for date: `{}`.",
                inp_file_path,
                nxt_aggr_dt.format("%d-%m-%Y"),
            );
            let directory_path = &empty_file_path
                .rsplit_once("/")
                .expect("Could not extract directory path.")
                .0;
            // create the empty file in the directory
            let _ = create_dir(directory_path).is_ok();
            let _ = File::create(&empty_file_path);
        }

        let avg_bal_file_path = config_params
            .avg_bal_input_file_path()
            .replace(&as_on_dt, &date_folder_name);
        if Path::new(&avg_bal_file_path).exists() {
            let avg_bal_file = match new_buf_rdr(&avg_bal_file_path) {
                Ok(file) => file,
                Err(error) => panic!("Could not found file `{}` : {}.", avg_bal_file_path, error),
            };
            for lines in avg_bal_file.lines() {
                let line = match lines {
                    Ok(line) => line,
                    Err(error) => panic!(
                        "Unable to read file `{}` : {}",
                        config_params.avg_bal_input_file_path(),
                        error
                    ),
                };
                let ip_acc: Vec<&str> = line.split('|').collect();
                ip_avg_bal.insert(
                    ip_acc[0].to_string(),
                    InputBalances {
                        input_avg_bal: ip_acc[config_params.avg_bal_pos() - 1]
                            .parse::<f64>()
                            .unwrap_or(DEFAULT_FLOAT),
                        days: nxt_aggr_dt.day() as f64,
                        acr_int_amt_ccy: ip_acc[config_params.acr_int_amt_ccy_pos() - 1]
                            .parse::<f64>()
                            .unwrap_or(DEFAULT_FLOAT),
                        acr_int_amt_hcy: ip_acc[config_params.acr_int_amt_hcy_pos() - 1]
                            .parse::<f64>()
                            .unwrap_or(DEFAULT_FLOAT),
                    },
                );
            }
        }

        let mut reader = read_file(&inp_file_path, &empty_file_path, log);
        for (line_num, lines) in reader.deserialize().enumerate() {
            let input_account: InputAccount = extract_lines(line_num, lines, &inp_file_path, log);
            let account: InputParsedAccount = input_account.parse();
            let mut def_bals = vec![0.0; dates.no_of_days as usize];
            def_bals[day - 1] = account.amt;
            let def_avg_bal = InputBalances {
                input_avg_bal: 0.0,
                days: 0.0,
                acr_int_amt_ccy: 0.0,
                acr_int_amt_hcy: 0.0,
            };
            let input_avgbal = ip_avg_bal.get(&account.acc_no).unwrap_or(&def_avg_bal);
            if input_avgbal.input_avg_bal != 0.0 {
                if let Some(mut bals) = account_pool.remove(&account.acc_no) {
                    bals[day - 1] = input_avgbal.input_avg_bal;
                    account_pool.insert(account.acc_no.to_string(), bals);
                } else {
                    account_pool.insert(account.acc_no.to_string(), def_bals.clone());
                }
                let rec_data = AggregateData {
                    calc_flag: "From Source".to_string(),
                    calculated_avg_bal: 0.0,
                    input_avg_bal: input_avgbal.input_avg_bal,
                    int_rate: account.int_rt,
                    days: input_avgbal.days as f64,
                    acr_int_amt_ccy: input_avgbal.acr_int_amt_ccy,
                    acr_int_amt_hcy: input_avgbal.acr_int_amt_hcy,
                };
                bal_org.insert(account.acc_no.to_string(), rec_data);
            } else {
                let rec_data = AggregateData {
                    calc_flag: "System Generated".to_string(),
                    calculated_avg_bal: account.amt,
                    input_avg_bal: 0.0,
                    int_rate: 0.0,
                    days: 0 as f64,
                    acr_int_amt_ccy: input_avgbal.acr_int_amt_ccy,
                    acr_int_amt_hcy: input_avgbal.acr_int_amt_hcy,
                };
                bal_org
                    .entry(account.acc_no.to_string())
                    .and_modify(|m| {
                        m.add(
                            "System Generated".to_string(),
                            account.amt,
                            account.int_rt,
                            input_avgbal.acr_int_amt_ccy,
                            input_avgbal.acr_int_amt_hcy,
                        )
                    })
                    .or_insert(rec_data);
                if let Some(mut bals) = account_pool.remove(&account.acc_no) {
                    bals[day - 1] = account.amt;
                    account_pool.insert(account.acc_no.to_string(), bals);
                } else {
                    account_pool.insert(account.acc_no.to_string(), def_bals.clone());
                }
            }
        }
        nxt_aggr_dt = nxt_aggr_dt.succ();
        day += 1;
    }
}
