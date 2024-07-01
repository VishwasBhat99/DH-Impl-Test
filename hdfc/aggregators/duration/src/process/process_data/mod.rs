use super::read_data::account_field_names::AccFieldNames;
use super::read_data::*;
use super::ConfigurationParameters;
use process::currency::currency_converter::CurrencyConverter;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn get_account_info(
    asondate: &str,
    account: &AccountWithCFs,
    rules: &AggRules,
    acc_keys: &AccFieldNames,
    config_params: &ConfigurationParameters,
    currency_converter: &CurrencyConverter,
    logger: &Logger,
) -> account_structs::AccountInfo {
    let currency = get_ccy_id(&account, &acc_keys);
    let bal_1 = account.get_f64_for_key(&acc_keys.balance).unwrap_or(0.0);
    let bal_2 =
        currency_converter.convert(&currency, bal_1, config_params.is_consolidated(), logger);
    let bal_ccy;
    let bal_lcy;
    if config_params.is_consolidated() {
        bal_ccy = bal_2;
        bal_lcy = bal_1;
    } else {
        bal_ccy = bal_1;
        bal_lcy = bal_2;
    }
    account_structs::AccountInfo {
        as_on_dt: get_aod(asondate),
        llg_id: get_llg_id(&account, &rules, config_params, logger),
        account_number: get_account_number(&account, &acc_keys),
        ccy_id: get_ccy_id(&account, &acc_keys),
        bal_amt_ccy: bal_ccy,
        bal_amt_hcy: bal_lcy,
        duration: get_duration(&account, &acc_keys),
    }
}

pub fn calc_duration_data(aggr_data: &account_structs::AggrData) -> f64 {
    let balance = aggr_data.balance;
    let weighted_avr = aggr_data.weighted_avg;
    weighted_avr / balance
}
