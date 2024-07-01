use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::date_from_timestamp;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use splitter::reader::account_with_cfs::get_field_value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::time::SystemTime;

mod implementation;
mod llg_key;
mod writers;

pub fn split_files(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    // Initialize a pool of writers.
    let mut writers_pool: HashMap<String, BufWriter<File>> = HashMap::new();
    let mut account_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let input_reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let mut field_names = Vec::new();
    let metadata_reader = fs::read_to_string(&config_params.metadata_file_path())
        .expect("Failed to read metadata file!");
    //Fetch the names from the metadata file.
    for line in metadata_reader.lines() {
        if line.contains("name") {
            let fields: Vec<&str> = line.split(':').collect();
            let mut name = fields[1].to_string();
            name.pop();
            name.pop();
            name = name[2..].to_string();
            field_names.push(name);
        }
    }
    let mut source_map: HashMap<String, String> = HashMap::new();
    let source_map_reader = fs::read_to_string(&config_params.source_map_file_path())
        .expect("Failed to read source map file!");

    for line in source_map_reader.lines() {
        let source_fields = line.split('|').collect::<Vec<&str>>();
        if source_fields.len() >= 2 {
            source_map.insert(source_fields[0].to_string(), source_fields[1].to_string());
            let new_writer = writers::get_new_writer(
                source_fields[1].to_string(),
                config_params.output_file_path(),
            );
            writers_pool.insert(source_fields[0].to_string(), new_writer);
        } else {
            log_error!(logger, "Line not proper:{}", line);
        }
    }
    let rules = AggRules::new_from_path(config_params.rule_file_path(), &input_reader);
    let _as_on_date = config_params.as_on_date();
    for (_count, account) in account_reader.iter().enumerate() {
        let llg = log_measurements!(
            diag_logger,
            ["Type: GetLLG, Identifier".to_string()],
            implementation::llg_for_account(
                &account,
                &rules,
                field_names[0].to_string(),
                config_params,
                logger
            )
        );
        let mut output_line = String::new();
        let date_vec: Vec<&str> = config_params.dates_pos().split(',').collect();
        let mut temp_date_vec: Vec<String> = Vec::new();
        for i in date_vec {
            temp_date_vec.push(i.to_string());
        }
        let mut pos_count = 0;
        for key in &field_names {
            pos_count += 1;
            let value = get_field_value(&account, &input_reader, key.to_string())
                .unwrap_or_else(|_| "NA".to_string());
            if temp_date_vec.contains(&pos_count.to_string()) {
                let op_val = date_from_timestamp(value.parse::<i64>().unwrap_or(0));
                let a = op_val.format("%d-%m-%Y").to_string();
                output_line.push_str(&a.to_string());
                output_line.push('|');
                continue;
            }
            output_line.push_str(&value);
            output_line.push('|');
        }
        output_line.pop();
        output_line.push('\n');
        // writer
        let writer = match writers_pool.get_mut(&llg.to_string()) {
            Some(writer) => writer,
            None => {
                //If the source id could not be found the output is written to a default file: "NA.txt".
                let new_writer = writers::get_new_writer(
                    config_params.default_file_name().to_string(),
                    config_params.output_file_path(),
                );
                writers_pool.insert(llg.source_code.to_string(), new_writer);
                // cannot return new writer as ownership of that writer is assigned to writers_pool in prev step
                writers_pool.get_mut(&llg.source_code.to_string()).unwrap()
            }
        };
        writers::write_data(writer, output_line, logger);
    }
}
