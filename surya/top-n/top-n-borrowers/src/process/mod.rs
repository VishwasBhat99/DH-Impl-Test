mod account_field_names;
pub mod config;
pub mod reader;
pub mod structs;
pub mod writer;

use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::account_field_names::AccFieldNames;
use process::structs::*;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
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
    let mut ucic_cust_map: HashMap<String, UcicDet> = HashMap::new();
    for (idx, line) in reader.lines().enumerate() {
        let ucic_det_line = line.expect(&format!("Cannot read line at {}", idx));
        let fields: Vec<&str> = ucic_det_line.split("~#~").collect();
        let ucic_data = UcicDet {
            ucic_id: fields[3].to_string(),
            ucic_name: fields[2].to_string(),
            cust_type: fields[1].to_string(),
        };
        if !ucic_cust_map.contains_key(fields[0]) {
            ucic_cust_map.insert(fields[0].to_string(), ucic_data);
        }
    }
    info!(
        diag_log,
        "End of creating hashtable for custid and ucic details mapping."
    );
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
    let mut cust_map: HashMap<String, CustVal> = HashMap::new();
    let mut country_cd = "".to_string();
    let mut amt_vec: Vec<f64> = Vec::new();
    for file in files_config.files {
        country_cd = file.country_cd;
        let mut file_rdr: Reader =
            reader::read_file(&file.input_file_path, &file.metadata_file_path);

        let deafult_cust_id: String = "NA".to_string();
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        for account in file_rdr.iter() {
            let cust_id = account
                .get_string_for_key(&keys.cust_id)
                .unwrap_or(&deafult_cust_id);
            if cust_id == &deafult_cust_id {
                log_debug!(
                    log,
                    "Cust ID not found for an account"
                );
            }
            let ost_prin_amt = account
                .get_f64_for_key(&keys.ost_prin_amt)
                .expect("Error while reading principal amt.");
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
            let amt_hcy: f64;
            if files_config.is_consolidated == "true" {
                amt_hcy = ost_prin_amt / exrt;
            } else {
                amt_hcy = ost_prin_amt * exrt;
            }
            let mut cust_name = "NONE".to_string();
            let mut cust_type = "NONE".to_string();
            if ucic_cust_map.contains_key(cust_id) {
                let ucic_map_values = ucic_cust_map
                    .get(cust_id)
                    .expect(&format!("Could not fetch ucic info for : {}", cust_id));
                cust_name = ucic_map_values.ucic_name.to_string();
                cust_type = ucic_map_values.cust_type.to_string();
            }
            if cust_map.contains_key(cust_id) {
                let val = cust_map
                    .get(cust_id)
                    .expect(&format!("cannot fetch value for cust id: {} ", cust_id));
                let new_amt = amt_hcy
                    + val
                        .tot_amt_hcy
                        .parse::<f64>()
                        .expect("cannot convert amount");
                let new_val = CustVal {
                    cust_name: val.cust_name.to_string(),
                    cust_type: val.cust_type.to_string(),
                    tot_amt_hcy: new_amt.to_string(),
                    npa_class: npa_class,
                };
                cust_map.insert(cust_id.to_string(), new_val);
            } else {
                let val = CustVal {
                    cust_name: cust_name,
                    cust_type: cust_type,
                    tot_amt_hcy: amt_hcy.to_string(),
                    npa_class: npa_class,
                };
                cust_map.insert(cust_id.to_string(), val);
            }
        }
    }
    for (_, value) in cust_map.iter() {
        amt_vec.push(
            value
                .tot_amt_hcy
                .parse::<f64>()
                .expect("cannot convert amount whie making vector"),
        );
    }
    amt_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    if (config_params.top_cust_count() + 1) as usize <= amt_vec.len() {
        amt_vec.split_off((config_params.top_cust_count() + 0) as usize);
    }
    writer::write_file(
        &mut writer,
        &config_params.as_on_date(),
        &country_cd.to_string(),
        cust_map,
        amt_vec,
        log,
    );
    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&files_config.output_file_path);
}
