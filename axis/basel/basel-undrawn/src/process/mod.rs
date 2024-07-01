extern crate serde;
extern crate serde_json;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut writer = match buf_file_wrtr(&config_params.output_file_path(), None) {
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
    let rtl_custid_lcr_class_map =
        get_aggr_map(config_params.rtl_aggr_input_file_path().to_string());
    let nrtl_custid_lcr_class_map =
        get_aggr_map(config_params.nrtl_aggr_input_file_path().to_string());
    let lcr_undrawn_file = match new_buf_rdr(config_params.lcr_undrawn_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found lcr undrawn file: `{}` on location `{}` : {}.",
            config_params.lcr_undrawn_file_path(),
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };
    let conv_map = get_exchange_rate_map(
        config_params.base_currency().to_string(),
        config_params.exchange_rate_file_path().to_string(),
    );
    for (line_num, lines) in lcr_undrawn_file.lines().enumerate() {
        tot_rec += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.lcr_undrawn_file_path(),
                line_num + 1,
                error
            ),
        };

        let mut fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 9 {
            skp_rec += 1;
            log_error!(
                log,
                "Found Record with less than nine fields at line number:{}",
                line_num + 1
            );
            continue;
        }
        if fields[4] == "NULL" || fields[4] == " " || fields[4].to_string().is_empty() {
            fields[4] = "CRE"
        }
        if fields[5] != "Y" {
            fields[5] = "N"
        }
        if fields[6] == "NULL" || fields[6] == " " || fields[6] == "" {
            fields[6] = "BLANK"
        }
        let op_line = fields.join("|");
        let def_val = &("".to_string(), fields[7].to_string()).clone();
        let def_val1 = &("".to_string(), "".to_string()).clone();
        let cust_id = fields[1].to_string();
        let deposit_code = &rtl_custid_lcr_class_map
            .get(&cust_id)
            .unwrap_or(nrtl_custid_lcr_class_map.get(&cust_id).unwrap_or(def_val1))
            .1;
        let const_code = &rtl_custid_lcr_class_map
            .get(&cust_id)
            .unwrap_or(nrtl_custid_lcr_class_map.get(&cust_id).unwrap_or(&def_val))
            .1;
        let lcr_class = &rtl_custid_lcr_class_map
            .get(&cust_id)
            .unwrap_or(nrtl_custid_lcr_class_map.get(&cust_id).unwrap_or(&def_val))
            .0;
        let mut conv_amt = fields[8].parse().unwrap_or(0.0);
        let mut amt = fields[8].parse().unwrap_or(0.0);
        let ccy = fields[3].to_string();
        let exrt = conv_map.get(&ccy).unwrap_or(&1.0);
        if !config_params.is_consolidated() {
            conv_amt = amt * exrt;
        } else {
            amt = amt / exrt;
            fields[8] = &amt.to_string()
        }
        let op_str = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            config_params.as_on_date().format("%d-%m-%Y"),
            config_params.country(),
            op_line,
            deposit_code,
            const_code,
            lcr_class,
            conv_amt
        );
        writeln!(writer, "{}", op_str).unwrap_or_else(|error| {
            panic!("Unable to write to the bucketed output file: {}", error);
        });
    }
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}

pub fn get_aggr_map(aggr_file_path: String) -> HashMap<String, (String, String)> {
    let mut aggr_custid_lcr_class_map: HashMap<String, (String, String)> = HashMap::new();
    let aggr_file = match new_buf_rdr(&aggr_file_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found aggregator file: `{}` on location `{}` : {}.",
            aggr_file_path,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };

    for (line_num, lines) in aggr_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                aggr_file_path,
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() < 16 {
            continue;
        }
        let cust_id = fields[0].to_string();
        let lcr_class = fields[16].to_string();
        let final_consti_code = fields[13].to_string();
        aggr_custid_lcr_class_map.insert(cust_id, (lcr_class, final_consti_code));
    }

    aggr_custid_lcr_class_map
}

pub fn get_exchange_rate_map(cons_curr: String, ccy_path: String) -> HashMap<String, f64> {
    let ccy_file_contents = fs::read_to_string(ccy_path).expect("cannot read currency file");
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
