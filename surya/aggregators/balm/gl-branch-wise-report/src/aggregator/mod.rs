use self::llg_key::LLGKey;
use self::structs::AggregateData;
use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::io::BufRead;
use std::io::Write;
use std::time::SystemTime;

mod account_field_names;
mod duration_extensions;
mod implementation;
mod llg_key;
mod structs;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) {
    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    let debug_op_file_name = format!("{}-debug-rpt.txt", config_params.output_file_path());
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
    let keys = AccFieldNames::new_from_path(config_params.known_fields_file_path());

    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);

    //Get exchange rates
    let rdr = match new_buf_rdr(config_params.currency_conversion_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.currency_conversion_file_path(),
            e
        ),
    };
    let mut debug_writer = match buf_file_wrtr(&debug_op_file_name, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                debug_op_file_name,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let mut exchange_rate: HashMap<String, f64> = HashMap::new();
    for line in rdr.lines() {
        let mut line_components: Vec<String> = Vec::new();
        for component in line.unwrap().split('|') {
            line_components.push(component.to_string());
        }
        let rate: f64 = line_components[2].parse().unwrap();
        let mut key: String = String::new();
        key.push_str(&line_components[0]);
        key.push_str("|");
        key.push_str(&line_components[1]);

        exchange_rate.insert(key, rate);
    }
    let mut store: HashMap<LLGKey, AggregateData> = HashMap::new();
    for account in account_reader.iter() {
        let def_acc_no = String::from("NA");
        let mut llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account
                    .get_string_for_key(&keys.account_number)
                    .unwrap_or(&def_acc_no)
            )],
            implementation::llg_for_account(
                &account,
                &keys,
                &rules,
                config_params.default_llg_code(),
                logger,
                &mut debug_writer,
                &config_params
            )
        );
        let mut amt = account
            .get_f64_for_key(&keys.amt)
            .expect("Cannot get 'amount` field.");
        let conv_indicator = llg.category / 10000;
        if conv_indicator != 0 {
            llg.cf_type = "I".to_string();
            llg.category -= 10000;
        }
        if config_params.neg_cf_type() == llg.cf_type && config_params.is_neg() {
            amt *= -1.0;
        }
        let mut aggr_data = AggregateData::new();
        aggr_data.add_data(amt);

        if config_params.is_consolidated() {
            let consol_llg = LLGKey {
                currency: config_params.consol_ccy().to_string(),
                category: llg.category,
                cf_type: llg.cf_type.to_string(),
                dim_id: llg.dim_id.to_string(),
                item_id: llg.item_id.to_string(),
            };
            insert_into_store(&mut store, &consol_llg, &aggr_data);
            if llg.currency == config_params.src_local_ccy() {
                let base_llg = LLGKey {
                    currency: config_params.display_local_ccy().to_string(),
                    category: llg.category,
                    cf_type: llg.cf_type,
                    dim_id: llg.dim_id.to_string(),
                    item_id: llg.item_id.to_string(),
                };
                insert_into_store(&mut store, &base_llg, &aggr_data);
            } else {
                let base_llg = LLGKey::new(
                    llg.currency.to_string(),
                    llg.category,
                    llg.cf_type.to_string(),
                    llg.dim_id.to_string(),
                    llg.item_id.to_string(),
                );
                let mut key_for_exchange_rate: String = String::new();
                key_for_exchange_rate.push_str(&llg.currency);
                key_for_exchange_rate.push_str("|");
                key_for_exchange_rate.push_str(config_params.src_local_ccy());
                let multiplier = exchange_rate.get_mut(&key_for_exchange_rate);
                if let Some(mult) = multiplier {
                    aggr_data.values_multiplied_by(*mult);
                }
                insert_into_store(&mut store, &base_llg, &aggr_data);
            }
        } else {
            if llg.currency == config_params.src_local_ccy() {
                let base_llg = LLGKey {
                    currency: config_params.display_local_ccy().to_string(),
                    category: llg.category,
                    cf_type: llg.cf_type.to_string(),
                    dim_id: llg.dim_id.to_string(),
                    item_id: llg.item_id.to_string(),
                };
                insert_into_store(&mut store, &base_llg, &aggr_data);
            } else {
                let base_llg = LLGKey {
                    currency: llg.currency.to_string(),
                    category: llg.category,
                    cf_type: llg.cf_type.to_string(),
                    dim_id: llg.dim_id.to_string(),
                    item_id: llg.item_id.to_string(),
                };
                insert_into_store(&mut store, &base_llg, &aggr_data);
            }
            let consol_llg = LLGKey::new(
                config_params.consol_ccy().to_string(),
                llg.category,
                llg.cf_type.to_string(),
                llg.dim_id.to_string(),
                llg.item_id.to_string(),
            );
            let mut key_for_exchange_rate: String = String::new();
            key_for_exchange_rate.push_str(&llg.currency);
            key_for_exchange_rate.push_str("|");
            key_for_exchange_rate.push_str(config_params.src_local_ccy());
            let multiplier = exchange_rate.get_mut(&key_for_exchange_rate);
            if let Some(mult) = multiplier {
                aggr_data.values_multiplied_by(*mult);
            }
            insert_into_store(&mut store, &consol_llg, &aggr_data);
        }
    }

    let formated_as_on_date = format!("{}", config_params.as_on_date().format("%d-%m-%Y"));
    for (llg, data) in store.drain() {
        write!(
            output_file,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            llg.category,
            llg.item_id,
            llg.dim_id,
            formated_as_on_date,
            llg.currency,
            "ALL",
            llg.cf_type,
            data.tot_prin_amt,
            "0.0"
        )
        .expect("Unable to generate summary file.");
    }
    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

fn insert_into_store(
    store: &mut HashMap<LLGKey, AggregateData>,
    llg: &LLGKey,
    data: &AggregateData,
) {
    store
        .entry(llg.clone())
        .and_modify(|m| m.add_from_builder(*data))
        .or_insert(*data);
}
