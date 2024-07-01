use self::aggregation::AccData;
use self::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

mod aggregation;
mod implementation;
mod llg_key;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let account_metadata_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_metadata_reader);
    let input_file = File::open(config_params.input_file_path()).expect("Cannot open input file.");
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut aggr_data: HashMap<LLGKey, AccData> = HashMap::new();
    for line_opt in BufReader::new(input_file).lines() {
        let record = line_opt.expect("Cannot read line from input file.");
        let fields: Vec<&str> = record.split('|').collect();
        let amount_pos = match account_metadata_reader.get_field_pos(&"amount".to_string()) {
            Some(val) => val,
            None => panic!("Cannot read currency field from file."),
        };
        let id_pos = match account_metadata_reader.get_field_pos(&"id".to_string()) {
            Some(val) => val,
            None => panic!("Cannot read currency field from file."),
        };
        let acc_data = AccData {
            amount: match fields[amount_pos - 1].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field amount: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos - 1],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            amount_lcy: match fields[amount_pos].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field amount_lcy: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            int_rate: match fields[amount_pos + 1].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field int_rate: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 1],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim1: match fields[amount_pos + 2].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim1: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 2],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim1_lcy: match fields[amount_pos + 3].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim1_lcy: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 3],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim2: match fields[amount_pos + 4].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim2: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 4],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim2_lcy: match fields[amount_pos + 5].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim2_lcy: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 5],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim3: match fields[amount_pos + 6].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim3: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 6],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim3_lcy: match fields[amount_pos + 7].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim3_lcy: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 7],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim4: match fields[amount_pos + 8].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim4: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 8],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim4_lcy: match fields[amount_pos + 9].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim4_lcy: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 9],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim5: match fields[amount_pos + 10].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim5: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 10],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
            num_dim5_lcy: match fields[amount_pos + 11].parse() {
                Ok(val) => val,
                Err(error) => {
                    log_error!(
                        logger,
                        "Invalid field num_dim5_lcy: `{}` for id: `{}` :`{}`.",
                        fields[amount_pos + 11],
                        fields[id_pos - 1],
                        error
                    );
                    0.0
                }
            },
        };
        let llg = implementation::llg_for_account(
            &record,
            &rules,
            &account_metadata_reader,
            fields,
            config_params,
        );
        aggr_data
            .entry(llg)
            .and_modify(|m| m.add_data(&acc_data))
            .or_insert(acc_data);
    }
    for (key, value) in aggr_data.drain() {
        write!(
            output_file,
            "{}|{}|{}|{}",
            config_params.as_on_date().format("%d-%m-%Y").to_string(),
            key.llg_id,
            key.currency,
            value
        )
        .expect("Unable to generate summary file.");
    }
}
