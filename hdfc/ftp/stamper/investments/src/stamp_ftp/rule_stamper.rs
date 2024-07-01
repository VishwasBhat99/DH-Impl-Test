use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRules_adj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

//Reads method rules and produce the matched method for an account passed
#[allow(dead_code, unused_imports)]
pub fn get_method(
    account: &AccountWithCFs,
    rules: &AggRules,
    default_method: i32,
    logger: &Logger,
) -> i32 {
    let method = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to method `{}`, using rule id `{}`",
                "TODO: Account-Id",
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
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

pub fn get_adj(account: &AccountWithCFs, rules: &AggRules_adj, logger: &Logger) -> Vec<i32> {
    //Note : 6 => maximum number of adjustments FTP system supports
    let lst_adj = rules.llg_for_acc(account, 6);
    let mut lst_adj_id: Vec<i32> = Vec::new();

    for result in lst_adj {
        lst_adj_id.push(match result {
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
                    0
                );
                0
            }
        });
    }

    lst_adj_id
}
