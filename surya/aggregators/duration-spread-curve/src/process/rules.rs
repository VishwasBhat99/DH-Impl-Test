use process::reader::account_with_cfs::AccountWithCFs;
use sdb_agg_rules::agg_rules::AggRules;

pub fn get_llg(account: &AccountWithCFs, rules: &AggRules, default_llg: i32) -> i32 {
    let mut llg = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => default_llg,
    };

    if (llg / 10000) > 0 {
        llg -= 10000;
    }
    llg
}
