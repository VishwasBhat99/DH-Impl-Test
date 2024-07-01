use crate::macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;
#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &AccountWithCFs,
    rules: &AggRules,
    default_llg_code: String,
    logger: &Logger,
) -> i32 {
    let llg_id = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                " evaluated to LLGId `{}`, using rule id `{}`",
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(logger, "defaulted to LLGId `{}`", default_llg_code);
            default_llg_code.parse().unwrap_or(0)
        }
    };
    llg_id
}
