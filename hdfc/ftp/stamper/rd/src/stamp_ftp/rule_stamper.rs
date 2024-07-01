use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

//Reads method rules and produce the matched method for an account passed
#[allow(dead_code, unused_imports)]
pub fn get_method(
    account: &AccountWithCFs,
    rules: &AggRules,
    default_method: i32,
    diag_log: &Logger,
) -> i32 {
    let method = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                diag_log,
                "Account `{}` evaluated to method `{}`, using rule id `{}`",
                "TODO: Account-Id",
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                diag_log,
                "Account `{}` defaulted to method `{}`",
                "TODO: Account-Id",
                default_method
            );
            default_method
        }
    };

    method
}

//Reads basecurve rules and produce the matched basecurve for and account passed
pub fn get_bc(account: &AccountWithCFs, rules: &AggRules, default_bc: i32, logger: &Logger) -> i32 {
    let curve = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to basecurve `{}`, using rule id `{}`",
                "TODO: Account-Id",
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to basecurve `{}`",
                "TODO: Account-Id",
                default_bc
            );
            default_bc
        }
    };

    curve
}
