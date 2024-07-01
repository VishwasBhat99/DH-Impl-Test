use crate::macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_agg_rules_adj::agg_rules::AggRulesAdj;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

//Reads method rules and produce the matched method for an account passed
pub fn get_method(
    acc_id: &str,
    account: &AccountWithCFs,
    rules: &AggRules,
    default_method: String,
    logger: &Logger,
) -> i32 {
    let method = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to method `{}`, using rule id `{}`",
                acc_id,
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to method `{}`",
                acc_id,
                default_method
            );
            default_method.parse::<i32>().unwrap()
        }
    };

    method
}

//Reads basecurve rules and produce the matched basecurve for and account passed
pub fn get_bc(
    acc_id: &str,
    account: &AccountWithCFs,
    rules: &AggRules,
    default_bc: String,
    logger: &Logger,
) -> i32 {
    let curve = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to basecurve `{}`, using rule id `{}`",
                acc_id,
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to basecurve `{}`",
                acc_id,
                default_bc
            );
            default_bc.parse::<i32>().unwrap()
        }
    };

    curve
}

pub fn get_adj(
    acc_id: &str,
    account: &AccountWithCFs,
    rules: &AggRulesAdj,
    result_count: i32,
    logger: &Logger,
) -> Vec<i32> {
    let lst_adj = rules.llg_for_acc(account, result_count);
    let mut lst_adj_id: Vec<i32> = Vec::new();

    for result in lst_adj {
        lst_adj_id.push(match result {
            Some(c) => {
                log_debug!(
                    logger,
                    "Account `{}` evaluated to adjustment `{}`, using rule id `{}`",
                    acc_id,
                    c.llg,
                    c.rule_id
                );
                c.llg
            }
            None => {
                log_debug!(
                    logger,
                    "Account `{}` defaulted to adjustment `{}`",
                    acc_id,
                    0
                );
                0
            }
        });
    }

    lst_adj_id
}

pub fn get_aorl_value(
    acc_id: &str,
    account: &AccountWithCFs,
    rules: &AggRules,
    default_aorl: &str,
    logger: &Logger,
) -> String {
    let llg_value = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to a_or_l `{}`, using rule id `{}`",
                acc_id,
                c.llg,
                c.rule_id
            );
            let value = match c.llg {
                1000 => "A",
                2000 => "L",
                _ => default_aorl,
            };
            value
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to a_or_l value: `{}`",
                acc_id,
                default_aorl
            );
            default_aorl
        }
    };

    llg_value.to_string()
}

//Reads llg rules and produce the matched llg id for and account passed
pub fn get_llg(
    acc_id: &str,
    account: &AccountWithCFs,
    rules: &AggRules,
    default_llg: &str,
    logger: &Logger,
) -> String {
    let curve = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to LLG `{}`, using rule id `{}`",
                acc_id,
                c.llg,
                c.rule_id
            );
            c.llg.to_string()
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to LLG `{}`",
                acc_id,
                default_llg
            );
            default_llg.to_string()
        }
    };

    curve
}
