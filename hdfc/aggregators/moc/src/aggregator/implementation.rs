use aggregator::account_field_names::AccFieldNames;
use aggregator::llg_key::LLGKey;
use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &AccountWithCFs,
    k: &AccFieldNames,
    rules: &AggRules,
    default_llg_code: i32,
    logger: &Logger,
) -> LLGKey {
    let currency = account.get_string_for_key(&k.institution).expect("fail");
    let account_num = account.get_string_for_key(&k.account_number).expect("fail");
    let category = match rules.llg_for_acc(account) {
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
                default_llg_code
            );
            default_llg_code
        }
    };
    let cf_type = "ALL";
    LLGKey::new(currency.to_string(), category, cf_type.to_string())
}
