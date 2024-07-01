use crate::configuration_parameters::ConfigurationParameters;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
pub fn get_llg(
    config_params: &ConfigurationParameters,
    account: &AccountWithCFs,
    acc_id: &String,
    amount: &f64,
    rules: &AggRules,
    logger: &Logger,
) -> i32 {
    let llg = match rules.llg_for_acc(account) {
        Some(val) => {
            log_debug!(
                logger,
                "Account '{}' evaluated to LLGId `{}`, amount `{}`",
                acc_id,
                val.llg,
                amount,
            );
            val.llg
        }
        None => {
            log_debug!(
                logger,
                "Account '{}' defaulted to LLGId `{}`,amount: `{}`",
                acc_id,
                config_params.default_llg_id().parse().unwrap_or(0 as i32),
                amount,
            );
            config_params.default_llg_id().parse().unwrap_or(0 as i32)
        }
    };
    llg
}
