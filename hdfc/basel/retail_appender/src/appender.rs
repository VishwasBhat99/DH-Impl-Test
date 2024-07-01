use crate::configuration_parameters::ConfigurationParameters;
use crate::io::*;
use crate::macros::LOG_PARAMS;
use crate::macros::PERF_PARAMS;
use crate::statics::*;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{prelude::*, BufReader};
use std::time::SystemTime;
use ahash::{AHasher, RandomState};
use ahash::AHashMap;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    println!("Processing Started,Please Wait ....");
    let start_process_timer = SystemTime::now();
    let total_positive_bal = DEFAULT_FLOAT;
    let total_negative_bal = DEFAULT_FLOAT;
    let mut ttl_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_acc: i64 = DEFAULT_INT;
    let mut input_cust_id_map: AHashMap<i64, i64> = AHashMap::new();
    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    //store the cust_id in input_cust_id_map
    for line in BufReader::new(input_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        if fields.len()==0{
            continue;
        }
        let cust_id =fields[2].parse().unwrap_or(0);
        input_cust_id_map.insert(cust_id,cust_id);
    }
    debug!(
        diag_log,"input file reading is done 1st time");
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for input file reading.");
    debug!(
        diag_log,
        "Total Duration for input file reading: {:?}.", duration
    );
    // Read Reference file the and store data in ref_map
    let ref_file = match new_buf_rdr(config_param.ref_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open reference file : {:?}", error),
    };
    
    let mut ref_map: AHashMap<i64, Vec<String>> = AHashMap::new();
    for line in BufReader::new(ref_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading reference file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("~#~").collect();
        if fields.len()==0{
            continue;
        }
        let mut new_fields: Vec<String> = fields.iter().map(|s| s.to_string()).collect();
        let cust_id = new_fields.remove(0).parse().unwrap_or(0);
        if input_cust_id_map.contains_key(&cust_id) {
            ref_map.insert(cust_id, new_fields);
        }
    }
    debug!(
        diag_log,"reference file reading is done");
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for reference file reading output.");
    debug!(
        diag_log,
        "Total Duration for reference file reading is done: {:?}.", duration
    );
    // Read master file the and store data in master_map
    let master_file = match new_buf_rdr(config_param.master_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open master file : {:?}", error),
    };
    let mut master_map: AHashMap<i64, Vec<String>> = AHashMap::new();
    for line in BufReader::new(master_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading master file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        if fields.len()==0{
            continue;
        }
        let mut new_fields: Vec<String> = fields.iter().map(|s| s.to_string()).collect();
        let cust_id = new_fields.remove(0).parse().unwrap_or(0);
        if input_cust_id_map.contains_key(&cust_id) {
            master_map.insert(cust_id, new_fields);
        }
    }
    debug!(
        diag_log,"master file reading is done");
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for master file reading is done.");
    debug!(
        diag_log,
        "Total Duration for master file reading is done: {:?}.", duration
    );
    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    let mut output_file = get_writer(&(config_param.output_file_path));
    for line in BufReader::new(input_file).lines().skip(0) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let mut fields: Vec<&str> = record.split("|").collect();
        ttl_acc_encntrd += 1;
        if fields.len()==0{
            skp_acc+=1;
            continue;
        }
        for index in 0..8 {
            fields.push("NA");
        }
        let cust_id=fields[2].parse().unwrap_or(0);
        if master_map.contains_key(&cust_id) {
            let curr_master_val = master_map.get(&cust_id).unwrap();
            fields[14] = &curr_master_val[0];
            fields[15] = &curr_master_val[1];
            fields[16] = &curr_master_val[2];
            fields[17] = &curr_master_val[3];
            fields[21] = &curr_master_val[4];
        }
        if ref_map.contains_key(&cust_id) {
            let curr_ref_val = ref_map.get(&cust_id).unwrap();
            fields[18] = &curr_ref_val[1];
            fields[20] = &curr_ref_val[0];
            if curr_ref_val[1].to_uppercase().contains("FIN")
                || curr_ref_val[1].to_uppercase().contains("NBFC")
                || curr_ref_val[1].to_uppercase().contains("BROK")
            {
                fields[19] = "TRUE";
            } else {
                fields[19] = "FALSE";
            }
        }

        writeln!(output_file, "{}", fields.join("|")).expect("Error while writing output fields.");
    }
    debug!(
        diag_log,"output file writing is done");
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for output file writing is done.");
    debug!(
        diag_log,
        "Total Duration for output file writing is done: {:?}.", duration
    );

    let health_report = HealthReport::new(
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        total_negative_bal + total_positive_bal,
        total_negative_bal + total_positive_bal,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
