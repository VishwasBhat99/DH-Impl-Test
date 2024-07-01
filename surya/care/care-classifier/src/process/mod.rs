use crate::statics::DEFAULT_FLOAT;
use crate::statics::DEFAULT_INT;

use self::account_field_names::AccFieldNames;
use self::get_file_id::get_file_id;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use integer_encoding::*;
use macros;
use sdb_agg_rules::agg_rules::get_all_llgs;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod account_field_names;
mod get_file_id;
mod writers;

pub fn classify(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // init a pool of writers as it depends on classification id
    let mut writers_pool: HashMap<i32, BufWriter<File>> = HashMap::new();
    let mut file_rdr: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut acc_enc = DEFAULT_INT;
    let mut acc_succ = DEFAULT_INT;
    let mut input_amount = DEFAULT_FLOAT;
    let mut output_amount = DEFAULT_FLOAT;
    let keys = AccFieldNames::new_from_path(&config_params.req_field_file());
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &file_rdr);
    let llg_ids: Vec<i32> =
        get_all_llgs(config_params.rules_file_path()).expect("Could not find values");
    for file_id in llg_ids {
        let new_writer = writers::get_new_writer(file_id, config_params.output_file_path());
        writers_pool.insert(file_id, new_writer);
    }
    let mut master_file_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut acc_class_map: HashMap<String, i32> = HashMap::new();
    for account in file_rdr.iter() {
        acc_enc += 1;
        // get account no
        let account_number = match account.get_string_for_key(&keys.account_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(
                "can not get the account_number for account {}",
                keys.account_id
            ),
        };
        let amount = match account.get_f64_for_key(& match keys.amount.to_owned() {
            Some(val)=>val,
            None=>"NA".to_string()
        }) {
            Ok(val) => val,
            Err(_) =>0.0,
        };
        // get classification id
        let file_id = get_file_id(&account, &rules);
        let claim_id = file_id % 10000;
        if acc_class_map.contains_key(&account_number) {
            log_warn!(
                logger,
                "Account ID {} Encountered Again in same file!!",
                account_number
            );
        } else {
            // Since Claim ID is last 4 digits
            acc_class_map.insert(account_number.to_owned(), claim_id);
        }
        if config_params.write_master() {
            let cust_id = match account.get_string_for_key(&match keys.cust_id.to_owned() {
                Some(val)=>val,
                None=>"NA".to_string()
            }) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            let currency = match account.get_string_for_key(&match keys.currency .to_owned() {
                Some(val)=>val,
                None=>"NA".to_string()
            }) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            master_file_map.insert(
                account_number,
                vec![cust_id, currency, claim_id.to_string()],
            );
        }
        // process
        let data_bytes: Vec<u8> = account.rec_bytes;
        let hdr_bytes: Vec<u8> = data_bytes.len().encode_var_vec();
        // writer
        let writer = match writers_pool.get_mut(&file_id) {
            Some(writer) => writer,
            None => {
                let new_writer = writers::get_new_writer(file_id, config_params.output_file_path());
                writers_pool.insert(file_id, new_writer);
                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool.get_mut(&file_id).unwrap()
            }
        };
        writers::write_data(writer, hdr_bytes, data_bytes);
        acc_succ += 1;
        input_amount += amount;
        output_amount += amount;
    }
    // flush all writers
    for (_, mut writer) in writers_pool.drain() {
        let _ = writer.flush();
    }
    let mut recon_file_writer = writers::get_recon_writer(config_params.recon_file_path());
    for (acc_no, id) in acc_class_map.drain() {
        let op_data = format!("{}|{}", acc_no, id);
        writers::write_recon_data(&mut recon_file_writer, op_data);
    }
    if config_params.write_master() {
        let mut master_file_writer = writers::get_master_writer(config_params.master_file_path());
        for (acc_no, master_val) in master_file_map.drain() {
            let op_data = format!("{}|{}", acc_no, master_val.join("|"));
            writeln!(&mut master_file_writer, "{}", op_data)
                .expect("master file line can not be written");
        }
    }
    // TODO: Use health check library
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
