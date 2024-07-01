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
    let currency = account
        .get_string_for_key(&k.institution)
        .expect("Error while reading currency.");
    let account_num = account
        .get_string_for_key(&k.account_number)
        .expect("Error while reading account number.");
    let mut category = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{:?}` evaluated to LLGId `{}`, using rule id `{}`",
                account_num,
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{:?}` defaulted to LLGId `{}`",
                account_num,
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
    if currency == config_params.consolidated_currency() {
        LLGKey::new(
            config_params.local_consolidation_currency().to_string(), // TODO: REAAALY Get rid of this now by removing the string from the account, instead of a clone.
            category,
            cf_type,
        )
    } else {
        LLGKey::new(
            currency.to_string(), // TODO: REAAALY Get rid of this now by removing the string from the account, instead of a clone.
            category,
            cf_type,
        )
    }
}
