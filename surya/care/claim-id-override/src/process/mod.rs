use crate::statics::DEFAULT_INT;

use self::get_file_id::get_file_id;
use self::input_config::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use integer_encoding::VarInt;
use macros;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::DEFAULT_FLOAT;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::{BufWriter, Write};
mod get_file_id;
mod input_config;
mod writers;

pub fn classify(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // init a pool of writers as it depends on classification id
    let mut writers_pool: HashMap<i32, BufWriter<File>> = HashMap::new();
    let mut acc_enc = DEFAULT_INT;
    let mut acc_succ = DEFAULT_INT;
    let mut input_amount = DEFAULT_FLOAT;
    let mut output_amount = DEFAULT_FLOAT;
    let input_config = AccFieldNames::new_from_path(config_params.input_config_file());
    log_debug!(logger, "Master File Reading Started");
    let master_file = match new_buf_rdr(&input_config.master_file) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found master_file: `{}` Error: {}.",
            &input_config.master_file, error
        ),
    };
    let mut master_file_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in master_file.lines().enumerate() {
        let master_line = match lines {
            Ok(master_line) => master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                &input_config.master_file,
                line_num + 1,
                error
            ),
        };
        let master_fields = master_line.split("|").collect::<Vec<&str>>();
        let cust_id: String = master_fields[1].to_string();
        master_file_map.insert(cust_id, master_fields[3].to_string());
    }
    log_debug!(logger, "Master File Reading Completed");
    log_debug!(logger, "Claim id File Reading Started");
    let claimid_file = match new_buf_rdr(&input_config.claim_id_file) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found claim_id_file: `{}` Error: {}.",
            &input_config.claim_id_file, error
        ),
    };
    let mut claim_id_vec: Vec<i32> = Vec::new();
    for (line_num, lines) in claimid_file.lines().enumerate() {
        let claim_id_line = match lines {
            Ok(claim_id_line) => claim_id_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                &input_config.claim_id_file,
                line_num + 1,
                error
            ),
        };
        claim_id_vec.push(claim_id_line.parse::<i32>().expect(&format!(
            "can not parse claimid at line {} in file {}",
            line_num, &input_config.claim_id_file
        )));
    }
    for file_id in claim_id_vec {
        let new_writer = writers::get_new_writer(file_id, config_params.output_file_path());
        writers_pool.insert(file_id, new_writer);
    }
    log_debug!(logger, "Claim_id File Reading Completed");
    for input_files in input_config.input_files {
        let mut file_rdr: Reader =
            reader::Reader::new_at_path(&input_files.metadata_files, &input_files.input_file);
        for account in file_rdr.iter() {
            acc_enc += 1;
            // get cust_id
            let cust_id = match account.get_string_for_key(&input_files.cust_id_field) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            let amount_field = match account.get_f64_for_key(&input_files.amount_field) {
                Ok(val) => val,
                Err(_) => 0.0,
            };
            // get classification id
            let file_id = get_file_id(
                input_files.input_file.to_string(),
                &master_file_map,
                &cust_id,
            );
            let data_bytes: Vec<u8> = account.rec_bytes;
            let hdr_bytes: Vec<u8> = data_bytes.len().encode_var_vec();
            // writer
            let writer = match writers_pool.get_mut(&file_id) {
                Some(writer) => writer,
                None => {
                    let new_writer =
                        writers::get_new_writer(file_id, config_params.output_file_path());
                    writers_pool.insert(file_id, new_writer);
                    // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                    writers_pool.get_mut(&file_id).unwrap()
                }
            };
            writers::write_data(writer, hdr_bytes, data_bytes);
            acc_succ += 1;
            input_amount += amount_field;
            output_amount += amount_field;
        }
    }
    // flush all writers
    for (_, mut writer) in writers_pool.drain() {
        let _ = writer.flush();
    }
    let health_stat = HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        input_amount,
        output_amount,
        0 as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path());
    log_info!(logger, "Total account read from input file: {}", acc_enc);
    log_info!(
        logger,
        "Total account written to output files: {}",
        acc_succ
    );
}
