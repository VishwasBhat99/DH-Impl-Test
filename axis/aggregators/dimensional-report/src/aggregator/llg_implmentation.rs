use aggregator::llg_keys::LLGKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_txt::agg_rules::AggRules as AggRules_txt;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;

#[allow(dead_code, unused_imports)]
pub fn llg_for_cf_account(
    account: &AccountWithCFs,
    rules: &AggRules,
    acc_id: String,
    config_param: &ConfigurationParameters,
    logger: &Logger,
) -> LLGKey {
    let llg_key = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account '{}' evaluated to LLGId `{}`, using rule id `{}`",
                acc_id,
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account '{}' defaulted to LLGId `{}`",
                acc_id,
                config_param.default_llg_code()
            );
            config_param.default_llg_code()
        }
    };
    LLGKey::new(llg_key)
}

#[allow(dead_code, unused_imports)]
pub fn llg_for_txt_account(
    account: &String,
    rules: &AggRules_txt,
    reader: &Reader,
    config_params: &ConfigurationParameters,
) -> i32 {
    match rules.llg_for_acc(account, reader) {
        Some(c) => c.llg,
        None => config_params.default_llg_code(),
    }
}
