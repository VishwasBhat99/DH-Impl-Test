use self::llg_key::LLGKey;
use self::structs::AggregateData;
use aggregator::account_field_names::AccFieldNames;
use macros;
use rbdate::NaiveDate;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::time::SystemTime;

mod account_field_names;
mod duration_extensions;
mod implementation;
mod llg_key;
mod structs;

pub fn aggregate_cashflows(
    cf_file_path: &str,
    as_on_date: &NaiveDate,
    output_path: &str,
    consolidated_currency: &str,
    local_consolidation_currency: &str,
    currency_conversion_file_path: &str,
    known_fields_file_path: &str,
    account_metadata_file_path: &str,
    rules_file_path: &str,
    default_llg_code: i32,
    logger: &Logger,
    diag_logger: &Logger,
) {
    // Prepare data we will require for processing.
    let start_time = SystemTime::now();
    let mut output_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
    {
        Ok(create) => create,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                output_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let keys = AccFieldNames::new_from_path(known_fields_file_path);

    let mut account_reader = reader::Reader::new_at_path(account_metadata_file_path, cf_file_path);
    let rules = AggRules::new_from_path(rules_file_path, &account_reader);

    //Get exchange rates
    let rdr = match sdb_io::new_buf_rdr(currency_conversion_file_path) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            currency_conversion_file_path, e
        )),
    };
    let mut exchange_rate: HashMap<String, f64> = HashMap::new();
    for line in rdr.lines() {
        let mut line_components: Vec<String> = Vec::new();
        for component in line.unwrap().split('|') {
            line_components.push(component.to_string());
        }
        let rate: f64 = line_components[2].parse().unwrap();
        let mut key: String = "".to_string();
        key.push_str(&line_components[0]);
        key.push_str("|");
        key.push_str(&line_components[1]);

        exchange_rate.insert(key, rate);
    }
    let mut store: HashMap<LLGKey, AggregateData> = HashMap::new();
    for account in account_reader.iter() {
        let mut llg = log_measurements!(
            diag_logger,
            [format!(
                "Type: GetLLG, Identifier: {:?}",
                account
                    .get_string_for_key(&keys.account_number)
                    .expect("fail")
            )],
            implementation::llg_for_account(&account, &keys, &rules, default_llg_code, logger)
        );
        let amt = account
            .get_f64_for_key(&keys.amt)
            .expect("Cannot get 'amount` field.");
        let int_rate = account
            .get_f64_for_key(&keys.int_rate)
            .expect("Cannot get 'amount` field.");
        let _should_convert: bool = {
            let conv_indicator = llg.category / 10000;
            if conv_indicator == 0 {
                true
            } else {
                llg.cf_type = "I".to_string();
                llg.category -= 10000;
                false
            }
        };
        let mut aggr_data = AggregateData::new();
        aggr_data.add_data(amt, int_rate);
        let conv_llg = LLGKey {
            currency: consolidated_currency.to_string(),
            category: llg.category,
            cf_type: llg.cf_type.clone(),
        };
        insert_into_store(&mut store, &conv_llg, &aggr_data);
        if llg.currency != consolidated_currency {
            let mut key_for_exchange_rate: String = "".to_string();
            key_for_exchange_rate.push_str(&llg.currency);
            key_for_exchange_rate.push_str("|");
            key_for_exchange_rate.push_str(consolidated_currency);
            let multiplier = exchange_rate.get_mut(&key_for_exchange_rate);
            if multiplier.is_some() {
                aggr_data.values_divided_by(*multiplier.unwrap());
            }
            insert_into_store(&mut store, &llg, &aggr_data);
        }
        if llg.currency == consolidated_currency {
            let conv_llg = LLGKey {
                currency: local_consolidation_currency.to_string(),
                category: llg.category,
                cf_type: conv_llg.cf_type,
            };
            insert_into_store(&mut store, &conv_llg, &aggr_data);
        }
    }
    let formated_as_on_date = format!("{}", as_on_date.format("%d-%m-%Y"));
    for (llg, data) in store.drain() {
        write!(
            output_file,
            "{}|{}|{}|{}|{}|{}|{}|{}\n",
            llg.category,
            formated_as_on_date,
            llg.currency,
            "SLR",
            "Master",
            llg.cf_type,
            data.tot_prin_amt,
            data.rate_amount_weighted / data.tot_prin_amt
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
    if store.get_mut(&llg).is_some() {
        store.get_mut(&llg).unwrap().add_from_builder(*data);
    } else {
        store.insert(llg.clone(), *data);
    }
}
