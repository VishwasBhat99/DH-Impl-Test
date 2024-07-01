use aggregator::reader::Reader;
use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules_txt::agg_rules::AggRules;
#[allow(dead_code, unused_imports)]
pub fn llg_for_txt_account(
    account: &String,
    rules: &AggRules,
    reader: &Reader,
    config_params: &ConfigurationParameters,
) -> i32 {
    let llg_id = match rules.llg_for_acc(account, reader) {
        Some(c) => c.llg,
        None => config_params.default_llg_code(),
    };
    llg_id
}
