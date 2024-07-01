use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn get_llg_id(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> i32 {
    match rules.llg_for_acc(account) {
        Some(val) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to LLGId `{}`, using rule id `{}`",
                "TODO: Account-Id",
                val.llg,
                val.rule_id
            );
            val.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to LLGId `{}`",
                "TODO: Account-Id",
                config_params.default_llg_code()
            );
            *config_params.default_llg_code()
        }
    }
}
