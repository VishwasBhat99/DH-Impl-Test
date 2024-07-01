use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_file_id(account: &AccountWithCFs, rules: &AggRules) -> i32 {
    // Here llg is the file class id
    let file_class_id = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => 0,
    };
    file_class_id
}
