use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
use splitter::llg_key::LLGKey;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
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
