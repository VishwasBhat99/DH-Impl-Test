use configuration_parameters::ConfigurationParameters;
use slog::Logger;
pub mod config;
use self::io::*;
mod io;
use sdb_dyn_proto_rdr::reader::Reader;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod exchange_rate;
use std::io::Write;

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct id {
        cf_type: String,
        llg_id: String,
        item_type: String,
        level_1: String,
        level_2: String,
        level_3: String,
        level_4: String,
    }
    impl id {
        fn new(
            cf_type: String,
            llg_id: String,
            item_type: String,
            level_1: String,
            level_2: String,
            level_3: String,
            level_4: String,
        ) -> id {
            id {
                cf_type: cf_type.to_string(),
                llg_id: llg_id.to_string(),
                item_type: item_type.to_string(),
                level_1: level_1.to_string(),
                level_2: level_2.to_string(),
                level_3: level_3.to_string(),
                level_4: level_4.to_string(),
            }
        }
    }

    let mut foreign_key_values = HashMap::new();
    let mut master_values = HashMap::new();

    let files_config = config::get_files(config_params.config_file_path());

    let mut op_writer = get_writer(&files_config.output_file_path);
    let master_file = File::open(&files_config.master_file_path).expect("Could Not Read File");
    let master_reader = BufReader::new(master_file);
    let bucket_len = files_config.bucket_end - files_config.llg_details_end;

    for (index, line) in master_reader.lines().enumerate().skip(1) {
        let mut master_vals: Vec<f64> = Vec::new();
        let line = line.expect("Could Not Read Line").to_string();
        let fields: Vec<&str> = line.split('|').collect();

        for i in files_config.llg_details_end..files_config.bucket_end {
            master_vals.push(fields[i].parse::<f64>().unwrap_or(0.0));
        }

        master_values.insert(
            id::new(
                fields[files_config.cf_type].to_string(),
                fields[files_config.llg_id].to_string(),
                fields[files_config.item_type].to_string(),
                fields[files_config.level_1].to_string(),
                fields[files_config.level_2].to_string(),
                fields[files_config.level_3].to_string(),
                fields[files_config.level_4].to_string(),
            ),
            master_vals,
        );
    }
    let exchange_rate_map = exchange_rate::read_exchange_rate(&files_config.exchange_file_path);
    for file in files_config.files {
        let current_files = File::open(&file.input_file_path).expect("Could Not Read File");

        let exchange_rate = exchange_rate::get_exch_rate(
            file.currency.to_string(),
            &files_config.base_currency.to_string(),
            &exchange_rate_map,
        );

        let reader = BufReader::new(current_files);

        for (index, line) in reader.lines().enumerate().skip(1) {
            let mut vals: Vec<f64> = Vec::new();
            let linee = line.expect("Could Not Read Line").to_string();
            let fields: Vec<&str> = linee.split('|').collect();

            for i in files_config.llg_details_end..files_config.bucket_end {
                vals.push(fields[i].parse::<f64>().unwrap_or(0.0));
            }

            foreign_key_values.insert(
                id::new(
                    fields[files_config.cf_type].to_string(),
                    fields[files_config.llg_id].to_string(),
                    fields[files_config.item_type].to_string(),
                    fields[files_config.level_1].to_string(),
                    fields[files_config.level_2].to_string(),
                    fields[files_config.level_3].to_string(),
                    fields[files_config.level_4].to_string(),
                ),
                vals,
            );
        }
        for key in foreign_key_values.keys() {
            write!(
                op_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}",
                key.cf_type,
                key.llg_id,
                key.item_type,
                key.level_1,
                key.level_2,
                key.level_3,
                key.level_4,
                file.currency
            );
            for i in 0..bucket_len {
                write!(
                    op_writer,
                    "|{}",
                    (foreign_key_values.get(key).expect("Could Not Read Value")[i] * exchange_rate)
                );
            }
            write!(op_writer, "\n");
        }

        for key in foreign_key_values.keys() {
            if master_values.contains_key(key) {
                for i in 0..bucket_len {
                    master_values.get_mut(key).expect("Could Not Read Value")[i] -=
                        foreign_key_values.get(key).expect("Could Not Read Value")[i] * exchange_rate;
                }
            }
        }
    }
    for (key,value) in master_values {
        write!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}",
            key.cf_type,
            key.llg_id,
            key.item_type,
            key.level_1,
            key.level_2,
            key.level_3,
            key.level_4,
            files_config.base_currency
        );
        for i in 0..bucket_len {
            write!(op_writer, "|{}",value[i]);
        }
        write!(op_writer, "\n");
    }
}
