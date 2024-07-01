use self::get_file_id::get_file_id;
use configuration_parameters::ConfigurationParameters;
use integer_encoding::*;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub mod config;
mod get_file_id;
mod writers;

pub fn classify(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // init a pool of writers as it depends on classification id
    let mut writers_pool: HashMap<i32, BufWriter<File>> = HashMap::new();
    let files_config = config::get_files(config_params.config_file_path());
    for file in files_config.files {
        let mut file_rdr: Reader =
            reader::Reader::new_at_path(&file.metadata_file_path, &file.input_file_path);
        let rules = AggRules::new_from_path(&file.rules_file_path, &file_rdr);
        let mut acc_enc = 0;
        let mut acc_succ = 0;
        let mut acc_class_map: HashMap<String, i32> = HashMap::new();
        for account in file_rdr.iter() {
            acc_enc += 1;
            // get account no
            let account_number = match account.get_string_for_key(&file.req_acc_field) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            // get classification id
            let file_id = get_file_id(&account, &rules);
            // println!("{}",file_id);
            if acc_class_map.contains_key(&account_number) {
                log_warn!(
                    logger,
                    "Account ID {} Encountered Again in same file!!",
                    account_number
                );
            } else {
                // Since Claim ID is last 4 digits
                let claim_id = file_id % 10000;
                acc_class_map.insert(account_number, claim_id);
            }
            // process
            let data_bytes: Vec<u8> = account.rec_bytes;
            let hdr_bytes: Vec<u8> = data_bytes.len().encode_var_vec();
            // writer
            let writer = match writers_pool.get_mut(&file_id) {
                Some(writer) => writer,
                None => {
                    let new_writer = writers::get_new_writer(file_id, &file.output_file_path);
                    writers_pool.insert(file_id, new_writer);
                    // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                    writers_pool.get_mut(&file_id).unwrap()
                }
            };
            writers::write_data(writer, hdr_bytes, data_bytes);
            acc_succ += 1;
        }
        // flush all writers
        for (_, mut writer) in writers_pool.drain() {
            let _ = writer.flush();
        }
        let mut recon_file_writer = writers::get_recon_writer(&file.recon_file_path);
        for (acc_no, id) in acc_class_map.drain() {
            let op_data = format!("{}|{}", acc_no, id);
            writers::write_recon_data(&mut recon_file_writer, op_data);
        }
        // TODO: Use health check library
        log_info!(logger, "Total account read from input file: {}", acc_enc);
        log_info!(
            logger,
            "Total account written to output files: {}",
            acc_succ
        );
    }
}
