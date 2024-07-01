use super::process_data::*;
use super::ConfigurationParameters;
use std::fs::File;
pub mod account_structs;
use self::account_field_names::AccFieldNames;
use super::chrono::NaiveDate;
use super::writer::*;
use macros;
use process::currency;
use process::currency::currency_converter::CurrencyConverter;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use std::collections::HashMap;
pub mod account_field_names;

pub fn process_data(
    config_params: &ConfigurationParameters,
    output_acount_level: &File,
    aggr_output: &File,
    logger: &Logger,
) {
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &account_reader);

    let mut accounts: account_structs::Accounts = HashMap::new();
    let as_on_date = config_params.as_on_date();
    let delimiter = config_params.delimiter();
    // Read currency exchange rate file
    let currency_converter: CurrencyConverter = currency::create_currency_converter(
        config_params.consolidated_currency(),
        config_params.exchange_rate_file(),
    );
    let acc_keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    for account_with_cfs in account_reader.iter() {
        let account_level_data = get_account_info(
            &as_on_date,
            &account_with_cfs,
            &rules,
            &acc_keys,
            config_params,
            &currency_converter,
            logger,
        );
        let aggr_key = account_structs::AggrKey {
            llg_id: account_level_data.llg_id.to_string(),
            currency_id: account_level_data.ccy_id.to_string(),
        };
        let aggr_data = account_structs::AggrData {
            balance: account_level_data.bal_amt_hcy,
            weighted_avg: account_level_data.bal_amt_hcy * account_level_data.duration,
        };
        // if INR -> INR and RUP, if USD -> USD, INR, and FCY
        if aggr_key.currency_id == config_params.consolidated_currency() {
            accounts.insert(aggr_key, aggr_data.clone());
            let lcy_aggr_key = account_structs::AggrKey {
                llg_id: account_level_data.llg_id.to_string(),
                currency_id: config_params.local_consolidation_currency().to_string(),
            };
            accounts.insert(lcy_aggr_key, aggr_data);
        } else {
            accounts.insert(aggr_key, aggr_data.clone());
            let consol_aggr_key = account_structs::AggrKey {
                llg_id: account_level_data.llg_id.to_string(),
                currency_id: config_params.consolidated_currency().to_string(),
            };
            accounts.insert(consol_aggr_key, aggr_data.clone());
            let fcy_aggr_key = account_structs::AggrKey {
                llg_id: account_level_data.llg_id.to_string(),
                currency_id: config_params.consolidated_currency().to_string(),
            };
            accounts.insert(fcy_aggr_key, aggr_data);
        }
        write_account_info(output_acount_level, account_level_data, delimiter);
    }
    write_aggrdata(aggr_output, accounts, get_aod(as_on_date), delimiter);
}
pub fn get_llg_id(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> String {
    match rules.llg_for_acc(account) {
        Some(val) => {
            log_debug!(
                logger,
                "Evaluated to llg_id `{}`, using rule id `{}`",
                val.llg,
                val.rule_id
            );
            val.llg.to_string()
        }
        None => {
            log_debug!(
                logger,
                "Defaulted to llg_id `{}`",
                config_params.default_llg()
            );
            config_params.default_llg().to_string()
        }
    }
}
pub fn get_account_number(account: &AccountWithCFs, acc_keys: &AccFieldNames) -> String {
    account
        .get_string_for_key(&acc_keys.account_number)
        .expect("Error while reading account_number.")
        .to_string()
}
pub fn get_ccy_id(account: &AccountWithCFs, acc_keys: &AccFieldNames) -> String {
    account
        .get_string_for_key(&acc_keys.currency)
        .expect("Error while reading currency.")
        .to_string()
}
pub fn get_duration(account: &AccountWithCFs, acc_keys: &AccFieldNames) -> f64 {
    account
        .get_f64_for_key(&acc_keys.duration)
        .expect("Error while reading duration.")
}
pub fn get_aod(aod: &str) -> NaiveDate {
    NaiveDate::parse_from_str(aod, "%d-%m-%Y").expect("unable to parse person date")
}
