use config::File;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_sub_id(account: &AccountWithCFs, rules: &AggRules, file: &File) -> i32 {
    // Here llg is the rw
    let sub_id = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => file.default_sub_claim_id,
    };
    sub_id
}

pub fn get_ecgc_type(account: &AccountWithCFs, rules: &AggRules) -> String {
    let ecgc_type = match rules.llg_for_acc(account) {
        Some(_) => "PRE".to_string(),
        None => "POST".to_string(),
    };
    ecgc_type
}
