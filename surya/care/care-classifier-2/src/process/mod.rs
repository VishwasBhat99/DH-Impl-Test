use self::account_fields_names::AccFieldNames;
use self::get_file_id::get_file_id;
use crate::statics::DEFAULT_FLOAT;
use crate::statics::DEFAULT_INT;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use integer_encoding::*;
use macros;
use sdb_agg_rules::agg_rules::get_all_llgs;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod account_fields_names;
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
    let mut master_file_map: HashMap<String, Vec<String>> = HashMap::new();
    // init total balance cf file
    let mut tot_bal_file_rdr: Reader = reader::Reader::new_at_path(
        config_params.tot_bal_metadata_file_path(),
        config_params.tot_bal_file_path(),
    );
    let tot_bal_rules =
        AggRules::new_from_path(config_params.tot_bal_rules_file_path(), &tot_bal_file_rdr);
    let mut llg_ids: Vec<i32> =
        get_all_llgs(config_params.tot_bal_rules_file_path()).expect("Could not find values");

    //Adding default and 0, to create extra output files:
    llg_ids.push(config_params.default_class_id());
    llg_ids.push(0);
    for file_id in llg_ids {
        let new_writer = writers::get_new_writer(file_id, config_params.output_file_path());
        writers_pool.insert(file_id, new_writer);
    }
    let mut tot_bal_map: HashMap<String, AccountWithCFs> = HashMap::new();
    for record in tot_bal_file_rdr.iter() {
        let key = match record.get_string_for_key(&config_params.tot_bal_key_field()) {
            Ok(val) => val.to_string(),
            Err(_) => record
                .get_i64_for_key(&config_params.tot_bal_key_field())
                .expect("Cannot read tot bal key field.")
                .to_string(),
        };
        tot_bal_map.insert(key, record);
    }
    let mut acc_class_map: HashMap<String, i32> = HashMap::new();
    // Input File Processing
    for account in file_rdr.iter() {
        acc_enc += 1;
        // get cust_id
        let split_key = match account.get_string_for_key(&keys.input_key) {
            Ok(val) => val.to_string(),
            Err(_) => account
                .get_i64_for_key(&keys.input_key)
                .expect("Cannot read tot bal key field for account.")
                .to_string(),
        };
        // get classification id
        let file_id = match tot_bal_map.get(&split_key) {
            Some(acc_rec) => get_file_id(&acc_rec, &tot_bal_rules),
            None => config_params.default_class_id(),
        };
        // get account no
        let account_number = match account.get_string_for_key(&keys.account_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };

        let amount = match account.get_f64_for_key(&match keys.amount.to_owned() {
            Some(val) => val,
            None => "NA".to_string(),
        }) {
            Ok(val) => val,
            Err(_) => 0.0,
        };
        input_amount += amount;
        // Since Claim ID is last 4 digits
        let claim_id = file_id % 10000;
        if acc_class_map.contains_key(&account_number) {
            log_warn!(
                logger,
                "Account ID {} Encountered Again in same file!!",
                account_number
            );
        } else {
            acc_class_map.insert(account_number.to_owned(), claim_id);
        }
        // process
        if config_params.write_master() {
            let cust_id = match account.get_string_for_key(&match keys.cust_id.to_owned() {
                Some(val) => val,
                None => "NA".to_string(),
            }) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            let currency = match account.get_string_for_key(&match keys.currency.to_owned() {
                Some(val) => val,
                None => "NA".to_string(),
            }) {
                Ok(val) => val.to_string(),
                Err(_) => format!(""),
            };
            master_file_map.insert(
                account_number,
                vec![cust_id, currency, claim_id.to_string()],
            );
        }
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
        output_amount += amount;
    }
    //write master file
    if config_params.write_master() {
        if config_params.master_file_path() == "NA" {
            panic!(
                "Master file path is not correct,passed path: {}",
                config_params.master_file_path()
            );
        }
        let mut master_file_writer = writers::get_master_writer(config_params.master_file_path());
        for (acc_no, master_val) in master_file_map.drain() {
            let op_data = format!("{}|{}", acc_no, master_val.join("|"));
            writeln!(&mut master_file_writer, "{}", op_data)
                .expect("master file line can not be written");
        }
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
    //Writing the health check report
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
