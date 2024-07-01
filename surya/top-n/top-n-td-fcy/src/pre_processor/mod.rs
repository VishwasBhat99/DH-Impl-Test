extern crate serde;
mod structure;

use self::structure::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };

    let mut tot_input_acc_encntrd: i64 = 0;
    let mut tot_acc_skippd: i64 = 0;
    let mut tot_succ_rec: i64 = 0;
    let mut writer = BufWriter::new(output_file);
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);

    let top_n_int_rate_file = match new_buf_rdr(config_param.top_n_int_rate()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found top n cust acc file: `{}` on location `{}` : {}.",
            config_param.top_n_cust_acc(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut top_n_int_rt_map: HashMap<TopNIntRateKey, TopNIntRateValue> = HashMap::new();
    let mut int_cust_vec: Vec<String> = Vec::new();
    for (line_num, lines) in top_n_int_rate_file.lines().enumerate() {
        let top_n_int_rt_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.top_n_cust_acc(),
                line_num + 1,
                error
            ),
        };

        let top_n_int_fields = top_n_int_rt_line.split("|").collect::<Vec<&str>>();
        int_cust_vec.push(top_n_int_fields[2].to_owned());
        let int_key = TopNIntRateKey {
            cust_id: top_n_int_fields[2].to_string(),
            ccy: top_n_int_fields[3].to_string(),
        };
        top_n_int_rt_map.entry(int_key).or_insert(TopNIntRateValue {
            country_cd: top_n_int_fields[0].to_string(),
            as_on_dt: top_n_int_fields[1].to_string(),
            sa_amt_hcy: top_n_int_fields[4].parse::<f64>().unwrap_or(0.0),
            sa_amt_ccy: top_n_int_fields[5].parse::<f64>().unwrap_or(0.0),
            ca_amt_hcy: top_n_int_fields[6].parse::<f64>().unwrap_or(0.0),
            ca_amt_ccy: top_n_int_fields[7].parse::<f64>().unwrap_or(0.0),
            sa_int_rt: top_n_int_fields[12].parse::<f64>().unwrap_or(0.0),
            ca_int_rt: top_n_int_fields[13].parse::<f64>().unwrap_or(0.0),
            tdwd_int_rt: top_n_int_fields[14].parse::<f64>().unwrap_or(0.0),
            tdnwd_int_rt: top_n_int_fields[15].parse::<f64>().unwrap_or(0.0),
        });
    }

    let top_n_cust_acc = match new_buf_rdr(config_param.top_n_cust_acc()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found top n cust acc file: `{}` on location `{}` : {}.",
            config_param.top_n_cust_acc(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut top_n_cust_acc_map: HashMap<CustDataKey, HashMap<String, CustDataValue>> =
        HashMap::new();
    let mut def_cust: HashMap<String, CustDataValue> = HashMap::new();
    def_cust.insert(
        "NA".to_string(),
        CustDataValue {
            amt_hcy: 0.0,
            amt_ccy: 0.0,
            int_rate: 0.0,
        },
    );
    for (line_num, lines) in top_n_cust_acc.lines().enumerate() {
        let top_n_cust_acc_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.top_n_cust_acc(),
                line_num + 1,
                error
            ),
        };

        let top_n_cust_acc_fields = top_n_cust_acc_line.split('|').collect::<Vec<&str>>();
        if top_n_cust_acc_fields.len() == 13 {
            //Store the LoanAccNo and NPA_Classification as key-value pairs.

            let key = CustDataKey {
                cust_id: top_n_cust_acc_fields[2].to_string(),
                ccy: top_n_cust_acc_fields[3].to_string(),
            };
            if top_n_cust_acc_map.contains_key(&key) {
                let cust_value_map = top_n_cust_acc_map.entry(key).or_insert(def_cust.clone());
                let cust_data = cust_value_map
                    .entry(top_n_cust_acc_fields[7].to_string())
                    .or_insert(CustDataValue {
                        amt_hcy: 0.0,
                        amt_ccy: 0.0,
                        int_rate: 0.0,
                    });
                cust_data.amt_hcy += top_n_cust_acc_fields[8].parse::<f64>().unwrap_or(0.0);
                cust_data.amt_ccy += top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0);
                cust_data.int_rate += (top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0)
                    * top_n_cust_acc_fields[10].parse::<f64>().unwrap_or(0.0));
            } else {
                let mut data = HashMap::new();
                data.insert(
                    top_n_cust_acc_fields[7].to_string(),
                    CustDataValue {
                        amt_hcy: top_n_cust_acc_fields[8].parse::<f64>().unwrap_or(0.0),
                        amt_ccy: top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0),
                        int_rate: (top_n_cust_acc_fields[9].parse::<f64>().unwrap_or(0.0)
                            * top_n_cust_acc_fields[10].parse::<f64>().unwrap_or(0.0)),
                    },
                );
                top_n_cust_acc_map.insert(key, data);
            }
        } else {
            log_error!(
                log,
                "Found line :{} at line number {} defective in top n cust acc data file.",
                line_num,
                top_n_cust_acc_line
            );
        }
    }
    let mut op_line = String::new();
    let def = TopNIntRateValue::new();
    for (key, value) in top_n_cust_acc_map.clone() {
        if int_cust_vec.contains(&key.cust_id) {
            let int_key = TopNIntRateKey {
                cust_id: key.clone().cust_id,
                ccy: key.clone().ccy,
            };
            let cust_key = CustDataKey {
                cust_id: key.cust_id.clone(),
                ccy: key.ccy.clone(),
            };

            let cust_data = top_n_cust_acc_map.get(&cust_key).unwrap_or(&def_cust);

            let mut tdwd_amt_hcy = 0.0;
            let mut tdwd_amt_ccy = 0.0;
            let mut tdnwd_amt_hcy = 0.0;
            let mut tdnwd_amt_ccy = 0.0;
            let mut tdwd_int_rate = 0.0;
            let mut tdnwd_int_rate = 0.0;
            for (prod_code, cust_data) in cust_data {
                if prod_code == "TDWD" {
                    tdwd_amt_hcy = cust_data.amt_hcy;
                    tdwd_amt_ccy = cust_data.amt_ccy;
                    tdwd_int_rate = if cust_data.amt_ccy == 0.0 {
                        0.0
                    } else {
                        cust_data.int_rate / cust_data.amt_ccy
                    };
                }
                if prod_code == "TDNWD" {
                    tdnwd_amt_hcy = cust_data.amt_hcy;
                    tdnwd_amt_ccy = cust_data.amt_ccy;
                    tdnwd_int_rate = if cust_data.amt_ccy == 0.0 {
                        0.0
                    } else {
                        cust_data.int_rate / cust_data.amt_ccy
                    };
                }
            }
            if key.ccy == config_param.base_currency() {
                let int_data = top_n_int_rt_map.get(&int_key).unwrap_or(&def);

                op_line = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                    config_param.country_id(),
                    int_data.as_on_dt,
                    key.cust_id.clone(),
                    key.ccy.clone(),
                    int_data.sa_amt_hcy,
                    int_data.sa_amt_ccy,
                    int_data.ca_amt_hcy,
                    int_data.ca_amt_ccy,
                    tdwd_amt_hcy,
                    tdwd_amt_ccy,
                    tdnwd_amt_hcy,
                    tdnwd_amt_ccy,
                    int_data.sa_int_rt,
                    int_data.ca_int_rt,
                    if tdwd_amt_hcy == 0.0 {
                        0.0
                    } else {
                        tdwd_int_rate
                    },
                    int_data.tdnwd_int_rt,
                );
            } else {
                op_line = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                    config_param.country_id(),
                    config_param.as_on_date().format("%Y-%m-%d"),
                    key.cust_id.clone(),
                    key.ccy.clone(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    tdwd_amt_hcy,
                    tdwd_amt_ccy,
                    tdnwd_amt_hcy,
                    tdnwd_amt_ccy,
                    0.0,
                    0.0,
                    tdwd_int_rate,
                    tdnwd_int_rate
                );
            }
            match writer.write_all(op_line.as_bytes()) {
                Ok(val) => val,
                Err(error) => {
                    panic!("Error writing processed data: {:?}", error);
                }
            }
            op_line.clear();
        }
    }

    let health_report = HealthReport::new(
        tot_input_acc_encntrd,
        tot_succ_rec,
        tot_acc_skippd,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_param.output_file_path());
}
