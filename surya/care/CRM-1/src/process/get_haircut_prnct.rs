use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_haircut_prnct(account: &AccountWithCFs, rules: &AggRules) -> f64 {
    // Here llg is the file class id
    let haircut_prnct = match rules.llg_for_acc(account) {
        Some(val) => val.llg as f64,
        None => 0.0,
    };
    // since llg will return 9099 for 90.99%
    haircut_prnct / 100.0
}
