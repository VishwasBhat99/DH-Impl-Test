use aggregator::account_field_names::AccFieldNames;
use aggregator::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
use macros;
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
        .unwrap_or(&config_param.base_currency().to_string())
        .to_string();
    LLGKey::new(currency, category)
}
