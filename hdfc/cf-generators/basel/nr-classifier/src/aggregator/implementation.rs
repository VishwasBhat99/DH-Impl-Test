use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

#[allow(dead_code, unused_imports)]
pub fn class_type_for_account(
    account: &AccountWithCFs,
    rules: &AggRules,
    default_code: i32,
    logger: &Logger,
) -> i32 {
    let class_type = match rules.llg_for_acc(account) {
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
                default_code
            );
            default_code
        }
    };
    class_type
}
