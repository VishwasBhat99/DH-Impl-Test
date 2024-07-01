use aggregator::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules_txt::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::Reader;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &String,
    rules: &AggRules,
    reader: &Reader,
    fields: Vec<&str>,
    config_params: &ConfigurationParameters,
) -> LLGKey {
    let currency_pos = match reader.get_field_pos(&"currency".to_string()) {
        Some(val) => val,
        None => panic!("Cannot read currency field from file."),
    };
    let currency = fields[currency_pos - 1];
    let llg_id = match rules.llg_for_acc(account, reader) {
        Some(c) => c.llg,
        None => config_params.default_llg_code(),
    };
    LLGKey::new(currency.to_string(), llg_id)
}
