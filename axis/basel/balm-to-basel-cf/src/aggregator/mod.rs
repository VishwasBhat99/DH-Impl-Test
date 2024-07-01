extern crate serde;
extern crate serde_json;
use self::account_field_names::ReqFields;
use self::bucket::get_bkt_slabs;
use self::implementation::get_buckted_amt;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::num_days_start_to_end;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::io::prelude::*;
mod account_field_names;

mod bucket;
mod implementation;

pub fn aggregate(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_rec = 0;
    let schm_id = config_params.scheme_id().parse().unwrap_or(0);
    let rules_scheme_map =
        get_rules_scheme_map(config_params.rules_scheme_file_path().to_string(), schm_id);
    let computation_map =
        get_computation_map(config_params.tbl_computation_file_path().to_string());
    let keys = ReqFields::new_from_path(&config_params.req_fields_file_path());
    let mut input_reader = reader::Reader::new_at_path(
        &config_params.metadata_file_path(),
        &config_params.input_file_path(),
    );
    let currency_conv_map = get_exchange_rate_map(
        config_params.base_currency().to_string(),
        config_params.exchange_rate_file().to_string(),
    );
    let mut bkt_writer = match buf_file_wrtr(&config_params.output_file_path(), None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create bucket aggregated file: `{}` on location `{}` : {:?}.",
                &config_params.output_file_path(),
                env::current_exe()
                    .unwrap_or_else(|error| {
                        panic!("Unable to find current directory path: {}", error);
                    })
                    .display(),
                error
            );
        }
    };
    let ccy_bkt_file_path = config_params.output_file_path().replace(".txt", "_ccy.txt");
    let mut ccy_bkt_writer = match buf_file_wrtr(&ccy_bkt_file_path, None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create CCY bucket aggregated file: `{}` on location `{}` : {:?}.",
                &config_params.output_file_path(),
                env::current_exe()
                    .unwrap_or_else(|error| {
                        panic!("Unable to find current directory path: {}", error);
                    })
                    .display(),
                error
            );
        }
    };

    let bucket_slabs = get_bkt_slabs(
        config_params.bucket_scheme_file_path(),
        config_params.as_on_date(),
    );
    let mut llg_amt_map: HashMap<(i32, String), Vec<f64>> = HashMap::new();
    let mut ccy_llg_amt_map: HashMap<(i32, String), Vec<f64>> = HashMap::new();
    let balm_rules = AggRules::new_from_path(&config_params.balm_rule_file_path(), &input_reader);
    let basel_rules = AggRules::new_from_path(&config_params.basel_rule_file_path(), &input_reader);
    let def_value = "NA".to_string();
    for data in input_reader.iter() {
        tot_rec += 1;
        let acid = data
            .get_string_for_key(&keys.acid)
            .unwrap_or(&def_value)
            .to_string();
        let currency = data
            .get_string_for_key(&keys.currency)
            .unwrap_or(&config_params.base_currency().to_string())
            .to_string();
        let out_bal_amt = data.get_f64_for_key(&keys.out_bal_amt).unwrap_or(0.0);
        let ex_rt = *currency_conv_map.get(&currency).unwrap_or(&1.0);
        let (mut ccy_amt, mut amt) = if !config_params.is_consolidated() {
            (out_bal_amt, out_bal_amt * ex_rt)
        } else if ex_rt != 0.0 {
            (out_bal_amt / ex_rt, out_bal_amt)
        } else {
            (0.0, out_bal_amt)
        };
        if config_params.is_amt_abs() {
            amt *= -1.0;
            ccy_amt *= -1.0;
        }
        let balm_llg = match balm_rules.llg_for_acc(&data) {
            Some(c) => {
                log_debug!(
                    log,
                    "Account '{}' evaluated to Balm LLGId `{}`, using rule id `{}`",
                    acid,
                    c.llg,
                    c.rule_id
                );
                c.llg
            }
            None => {
                log_debug!(
                    log,
                    "Account '{}' defaulted to Balm LLGId `{}`",
                    acid,
                    config_params.default_balm_llg()
                );
                config_params.default_balm_llg()
            }
        };
        if config_params.is_tbl_def_req() {
            if !computation_map.contains_key(&(balm_llg as i64)) {
                continue;
            } else {
                if computation_map.get(&(balm_llg as i64)).unwrap() != config_params.source_name() {
                    continue;
                }
            }
        }

        let mut amt_percent_map: HashMap<i64, f64> = HashMap::new();
        let mut ccy_amt_percent_map: HashMap<i64, f64> = HashMap::new();
        for (key, val) in bucket_slabs.iter() {
            let percent = rules_scheme_map
                .get(&(balm_llg as i64, *key))
                .unwrap_or(&0.0)
                / 100.0;
            if *key != 4 {
                let num_day = num_days_start_to_end(val.from_date, val.to_date);
                let amts_per_day = amt * percent / num_day as f64;
                amt_percent_map.insert(*key, amts_per_day);
            } else {
                let no_of_months = (num_days_start_to_end(val.from_date, val.to_date) / 365) * 12;
                let amt_per_month = amt * percent / no_of_months as f64;
                amt_percent_map.insert(*key, amt_per_month);
            }
        }
        for (key, val) in bucket_slabs.iter() {
            let percent = rules_scheme_map
                .get(&(balm_llg as i64, *key))
                .unwrap_or(&0.0)
                / 100.0;
                if balm_llg == 6299 {
                    println!("{}", ccy_amt);
                }
            if *key != 4 {
                let num_day = num_days_start_to_end(val.from_date, val.to_date);
                let amts_per_day = ccy_amt * percent / num_day as f64;
                ccy_amt_percent_map.insert(*key, amts_per_day);
            } else {
                let no_of_months = (num_days_start_to_end(val.from_date, val.to_date) / 365) * 12;
                let amt_per_month = ccy_amt * percent / no_of_months as f64;
                ccy_amt_percent_map.insert(*key, amt_per_month);
            }
        }
        let amt_vec = get_buckted_amt(
            amt_percent_map,
            config_params.as_on_date(),
            &bucket_slabs,
        );
        let ccy_amt_vec = get_buckted_amt(
            ccy_amt_percent_map,
            config_params.as_on_date(),
            &bucket_slabs,
        );
        let basel_llg = match basel_rules.llg_for_acc(&data) {
            Some(c) => {
                log_debug!(
                    log,
                    "Account '{}' evaluated to Basel LLGId `{}`, using rule id `{}`",
                    acid,
                    c.llg,
                    c.rule_id
                );
                c.llg
            }
            None => {
                log_debug!(
                    log,
                    "Account '{}' defaulted to Basel LLGId `{}`",
                    acid,
                    config_params.default_basel_llg()
                );
                config_params.default_basel_llg()
            }
        };

        let aggr_llg = if config_params.is_aggr_on_basel_llg() {
            basel_llg
        } else {
            balm_llg
        };
        llg_amt_map
            .entry((aggr_llg.clone(), currency.clone()))
            .and_modify(|vec| {
                vec.iter_mut()
                    .zip(amt_vec.iter())
                    .for_each(|(old, &new)| *old += new);
            })
            .or_insert(amt_vec);
        ccy_llg_amt_map
            .entry((aggr_llg.clone(), currency))
            .and_modify(|vec| {
                vec.iter_mut()
                    .zip(ccy_amt_vec.iter())
                    .for_each(|(old, &new)| *old += new);
            })
            .or_insert(ccy_amt_vec);
    }
    let ason_date = config_params.as_on_date();
    for ((llg, curr), val) in &llg_amt_map {
        let val_str = val
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("|");
        let op_str = format!(
            "{}|{}|{}|{}|{}",
            ason_date,
            config_params.country(),
            curr,
            llg,
            val_str
        );
        writeln!(bkt_writer, "{}", op_str).unwrap_or_else(|error| {
            panic!("Unable to write to the bucketed output file: {}", error);
        });
    }

    for ((llg, curr), val) in &ccy_llg_amt_map {
        let val_str = val
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("|");
        let op_str = format!(
            "{}|{}|{}|{}|{}",
            ason_date,
            config_params.country(),
            curr,
            llg,
            val_str
        );
        writeln!(ccy_bkt_writer, "{}", op_str).unwrap_or_else(|error| {
            panic!("Unable to write to the CCY bucketed output file: {}", error);
        });
    }

    let health_report = HealthReport::new(tot_rec, tot_rec, 0, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_rules_scheme_map(
    rules_scheme_file_path: String,
    scheme_id: i64,
) -> HashMap<(i64, i64), f64> {
    let mut rules_scheme_map: HashMap<(i64, i64), f64> = HashMap::new();
    let rules_scheme_file = match new_buf_rdr(&rules_scheme_file_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found rules scheme file: `{}` on location `{}` : {}.",
            rules_scheme_file_path,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };

    for (line_num, lines) in rules_scheme_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                rules_scheme_file_path,
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 16 {
            continue;
        }
        let llg_id = fields[3].to_string().parse().unwrap_or(0);
        let schm_id = fields[0].to_string().parse().unwrap_or(0);
        let maturity_id = fields[4].to_string().parse().unwrap_or(0);
        let percent_portion = fields[5].to_string().parse().unwrap_or(0.0);
        if schm_id == scheme_id {
            rules_scheme_map.insert((llg_id, maturity_id), percent_portion);
        }
    }

    rules_scheme_map
}

fn get_computation_map(computation_file_path: String) -> HashMap<i64, String> {
    let mut computation_map: HashMap<i64, String> = HashMap::new();
    let computation_file = match new_buf_rdr(&computation_file_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found computation file: `{}` on location `{}` : {}.",
            computation_file_path,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };

    for (line_num, lines) in computation_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                computation_file_path,
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 4 {
            continue;
        }
        let llg_id = fields[1].to_string().parse().unwrap_or(0);
        let deposit_type = fields[3].to_string();
        computation_map.insert(llg_id, deposit_type);
    }

    computation_map
}

pub fn get_exchange_rate_map(cons_curr: String, ccy_path: String) -> HashMap<String, f64> {
    let ccy_file_contents = std::fs::read_to_string(ccy_path).expect("cannot read currency file");
    let mut currency_map: HashMap<String, f64> = HashMap::new();
    for line in ccy_file_contents.lines() {
        let each_line: Vec<&str> = line.split('|').collect();
        if each_line[1] == cons_curr {
            currency_map.insert(
                each_line[0].to_string(),
                each_line[2]
                    .parse::<f64>()
                    .expect("unable to parse exchange rate"),
            );
        }
    }
    return currency_map;
}
