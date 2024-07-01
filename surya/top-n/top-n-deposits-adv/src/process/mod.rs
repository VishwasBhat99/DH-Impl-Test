mod account_field_names;
pub mod config;
pub mod reader;
pub mod writer;

use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::account_field_names::AccFieldNames;
use rbdate::incr_dt_by_mon_presrv_eom;
// use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let files_config = config::get_files(config_params.config_file_path());
    let mut writer = writer::create_writer_for_path(&files_config.output_file_path);
    let ucic_file = match new_buf_rdr(&files_config.ucic_map_file_path) {
        Ok(val) => val,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(ucic_file);
    let mut cust_id_vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split("|").collect();
        cust_id_vec.push(fields[2].to_string());
    }
    let exrt_file = match new_buf_rdr(&files_config.exrt_file_path) {
        Ok(val) => val,
        Err(error) => panic!("{}", error),
    };
    let exrt_reader = BufReader::new(exrt_file);
    let mut exrt_map: HashMap<String, f64> = HashMap::new();
    for line in exrt_reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split("|").collect();
        if fields[1] == &files_config.base_ccy {
            exrt_map.insert(
                fields[0].to_string(),
                fields[2].parse::<f64>().expect(&format!(
                    "could not parse exchange rate value for : {}",
                    fields[2]
                )),
            );
        }
    }
    let mut tot_rec = 0;
    let skp_rec = 0;
    let tot_amt = 0.0;
    for file in files_config.files {
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);

        let deafult_cust_id: String = "NA".to_string();
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        for account in file_rdr.iter() {
            tot_rec += 1;
            let cust_id = account
                .get_string_for_key(&keys.deal_id)
                .unwrap_or(&deafult_cust_id);
            if cust_id == &deafult_cust_id {
                log_debug!(
                    log,
                    "data not found for: {}",
                    account
                        .get_string_for_key(&keys.acc_no)
                        .expect("Error while reading acc number.")
                );
            }

            if !cust_id_vec.contains(&cust_id) {
                continue;
            }

            let acc_no = account
                .get_string_for_key(&keys.acc_no)
                .expect("Error while reading acc number.");
            let ost_prin_amt = account
                .get_f64_for_key(&keys.ost_prin_amt)
                .expect("Error while reading principal amt.");
            let int_rt = account
                .get_f64_for_key(&keys.int_rt)
                .expect("Error while reading int rate.");
            let mat_dt = account.get_i64_for_key(&keys.mat_dt).unwrap_or(0);
            let acc_start_dt = account.get_i64_for_key(&keys.acc_start_dt).unwrap_or(0);
            let ccy = account
                .get_string_for_key(&keys.ccy)
                .expect("Error while reading acc number.");
            let npa_class: String = account
                .get_string_for_key(&keys.npa_class)
                .unwrap_or(&"NONE".to_string())
                .to_string();
            let exrt = exrt_map
                .get(ccy)
                .expect(&format!("Could not get exchange rate for : {}", ccy));
            let mut amt_hcy: f64;
            if files_config.is_consolidated == "true" {
                amt_hcy = ost_prin_amt / exrt;
            } else {
                amt_hcy = ost_prin_amt * exrt;
            }
            writer::write_file(
                &mut writer,
                &cust_id,
                &acc_no,
                &ost_prin_amt,
                &int_rt,
                &mat_dt,
                &acc_start_dt,
                &ccy,
                &file.prod_type,
                &config_params.as_on_date(),
                &amt_hcy,
                &npa_class,
                &file.country_cd,
            );
        }
        let health_report =
            HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
        log_info!(log, "{}", health_report.display());
        health_report.gen_health_rpt(&files_config.output_file_path);
    }
}
