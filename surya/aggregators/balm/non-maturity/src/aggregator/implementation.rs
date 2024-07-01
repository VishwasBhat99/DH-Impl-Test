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
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> LLGKey {
    let currency = account.get_string_for_key(&k.curr).expect("fail");
    let mut category = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to LLGId `{}`, using rule id `{}`",
                "TODO: Account-Id",
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to LLGId `{}`",
                "TODO: Account-Id",
                config_params.default_llg_code()
            );
            config_params.default_llg_code()
        }
    };
    let cf_type = {
        let cf_type_id = category / 10000;
        if cf_type_id == 1 {
            "I".to_string()
        } else {
            "O".to_string()
        }
    };
    if cf_type == "I" {
        category = category - 10000;
    }

    LLGKey::new(currency.to_string(), category, cf_type)
}
