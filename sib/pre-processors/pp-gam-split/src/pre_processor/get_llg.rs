use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::Reader;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &String,
    rules: &AggRules,
    reader: &Reader,
    config_params: &ConfigurationParameters,
) -> i32 {
    match rules.llg_for_acc(account, reader) {
        Some(c) => c.llg,
        None => config_params.default_llg_code(),
    }
}
