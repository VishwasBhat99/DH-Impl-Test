use crate::aggregator::account_field_names::AccFieldNames;
use crate::aggregator::llg_key::LLGKey;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &AccountWithCFs,
    k: &AccFieldNames,
    rules: &AggRules,
    config_param: &ConfigurationParameters,
    logger: &Logger,
) -> LLGKey {
    let category = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to LLGId `{}`, using rule id `{}`",
                account
                    .get_string_for_key(&k.account_number)
                    .unwrap_or(&"NONE".to_string()),
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to LLGId `{}`",
                account
                    .get_string_for_key(&k.account_number)
                    .unwrap_or(&"NONE".to_string()),
                config_param.default_llg_code()
            );
            config_param.default_llg_code()
        }
    };
    let currency = account
        .get_string_for_key(&k.currency)
        .expect("Error while reading currency.")
        .to_string();
    LLGKey::new(currency, category)
}
