mod currency;
mod implementation;
mod llg_key;
mod organize;
mod structs;

use self::structs::AggregateData;
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::{prelude::*, BufReader, Write};
use std::time::SystemTime;
mod account_field_names;
use self::organize::Organizer;
use std::env;

pub fn aggregate_cashflows(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    let currency_converter = currency::create_currency_converter(
        config_params.consolidated_currency(),
        config_params.currency_conversion_file_path(),
    );
    let mut organizer = Organizer::new(currency_converter.clone());
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);
    let threshold_file_path = match new_buf_rdr(config_params.threshold_file_path()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Could not find threshold file: `{}`",
                config_params.threshold_file_path()
            );
            panic!(
                "Could not find threshold file: `{}` on location `{}` : {}.",
                config_params.threshold_file_path(),
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            );
        }
    };
    let reader = BufReader::new(threshold_file_path);
    let mut threshold_map: HashMap<i32, f64> = HashMap::new();
    for (line_no, line_read) in reader.lines().enumerate() {
        let line: String = match line_read {
            Ok(line) => line,
            Err(error) => {
                panic!(
                    "Cannot read line from threshold file: {:?}.\nError: {}",
                    line_no, error
                );
            }
        };
        if line_no == 0 {
            continue;
        }
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() != 2 {
            log_error!(logger, "Missing values in {}th row", line_no + 1);
            continue;
        }
        threshold_map.insert(
            fields[0].trim().parse().unwrap_or(0),
            fields[1].trim().parse().unwrap_or(0.0),
        );
    }
    let mut prin_amt: f64;
    let mut rt: f64;
    let mut output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` on location `{}`: {}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    for account in account_reader.iter() {
        let currency = account
            .get_string_for_key(&keys.curr)
            .expect("Could not read `currency` field from .cf file.");
        let llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account
                    .get_string_for_key(&keys.acc_no)
                    .unwrap_or(&"NA".to_string())
            )],
            implementation::llg_for_account(
                &account,
                &rules,
                currency,
                config_params.consolidated_currency(),
                config_params.local_consolidation_currency(),
                config_params.default_llg_code(),
                logger
            )
        );
        let mut build = AggregateData::new();

        prin_amt = account
            .get_f64_for_key(&keys.prin_amt)
            .expect("Failed to get `principal amount` from required fields.");

        if config_params.is_consolidated() {
            let ex_rt = *currency_converter.exchange_rate(currency);
            if ex_rt != 0.0 {
                prin_amt = prin_amt / ex_rt;
            } else {
                prin_amt = 0.0
            }
        }

        rt = account
            .get_f64_for_key(&keys.int_rt)
            .expect("Failed to get `interest amount` from required fields.");

        if config_params.is_amt_abs() {
            prin_amt = prin_amt.abs();
        } else {
            if llg.cf_type == "O" {
                prin_amt = prin_amt * -1.0;
            }
        }

        build.add_data(prin_amt, rt);

        log_measurements!(
            diag_logger,
            [format!("Type: OrganiseInLLG, Identifier: {:?}", llg)],
            organizer.build_width(
                &llg,
                &mut build,
                config_params.foreign_consolidation_currency(),
                config_params.local_consolidation_currency(),
            )
        );
    }
    for (llg, mut data) in organizer.drain() {
        let &threshold_val = threshold_map
            .get(&llg.category)
            .unwrap_or(&data.tot_prin_amt);
        if data.tot_prin_amt > threshold_val {
            data.tot_prin_amt = threshold_val;
        }
        write!(
            output_file,
            "{}|{}|{}|DIM1|DIM2|DIM3|DIM4|DIM5|{}\n",
            llg.category,
            config_params.as_on_date().format("%d-%m-%Y"),
            llg.currency,
            data
        )
        .expect("Unable to generate summary file.");
    }
}
