use self::account_field_names::AccFieldNames;
use self::get_crm_data::get_crm_data;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

mod account_field_names;
mod get_crm_data;
mod get_haircut_prnct;
mod structs;

pub fn classify(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // init a pool of writers as it depends on classification id
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let mut file_rdr: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut acc_class_map: HashMap<String, String> = HashMap::new();
    let rdr = match sdb_io::new_buf_rdr(config_params.acc_class_map_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.acc_class_map_file_path(),
            e
        )),
    };
    for (line_num, lines) in rdr.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read rules file at line number: `{}` : {}",
                line_num + 1,
                error
            ),
        };
        let line_info: Vec<&str> = line.split('|').collect();
        acc_class_map.insert(line_info[0].to_string(), line_info[1].to_string());
    }

    let acc_keys = AccFieldNames::new_from_path(config_params.req_file_path());
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &file_rdr);
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    for account in file_rdr.iter() {
        acc_enc += 1;
        let exp_acc_no = match account.get_string_for_key(&acc_keys.exp_acc_no) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };
        let class_id = acc_class_map
            .get(&exp_acc_no)
            .unwrap_or(&"0".to_string())
            .to_string();
        let op = get_crm_data(&account, &acc_keys, &rules, &config_params).print();
        let writer = match writers_pool.get_mut(&class_id) {
            Some(writer) => writer,
            None => {
                let new_writer = get_new_writer(&class_id, config_params.output_file_path());
                writers_pool.insert(class_id.to_string(), new_writer);
                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool.get_mut(&class_id).unwrap()
            }
        };
        let _ = writer.write(op.as_bytes());
        acc_succ += 1;
    }
    // flush all writers
    for (_, mut writer) in writers_pool.drain() {
        let _ = writer.flush();
    }

    // TODO: Use health check library
    log_info!(logger, "Total account read from input file: {}", acc_enc);
    log_info!(
        logger,
        "Total account written to output files: {}",
        acc_succ
    );
    let health_stat =
        health_report::HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, 0.0, 0.0, 0);
    health_stat.gen_health_rpt(config_params.output_file_path())
}

pub fn get_new_writer(file_id: &str, output_file_path: &str) -> BufWriter<File> {
    let full_path = format!("{}-{}.txt", output_file_path, file_id);
    match buf_file_wrtr(&full_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file '{}'. Error: {:?}.",
            full_path, error
        ),
    }
}
