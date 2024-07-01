use slog::Logger;
use macros;
use aggregator::llg_key::LLGKey;
use aggregator::account_field_names::AccFieldNames;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_agg_rules::agg_rules::AggRules;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(account: &AccountWithCFs, k: &AccFieldNames, rules: &AggRules, logger: &Logger) -> LLGKey {

    let category = match rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(logger, "Account `{}` evaluated to LLGId `{}`, using rule id `{}`", "TODO: Account-Id", c.llg, c.rule_id);
            c.llg
        },
        None => {
            // TODO: Take the default LLG as an input parameter to the program
            log_debug!(logger, "Account `{}` defaulted to LLGId `{}`", "TODO: Account-Id", 4226);
            4226
        }
    };

    LLGKey::new(
        account.get_string_for_key(&k.institution).expect("fail").to_string(),  // TODO: REAAALY Get rid of this now by removing the string from the account, instead of a clone.
        category
    )
}