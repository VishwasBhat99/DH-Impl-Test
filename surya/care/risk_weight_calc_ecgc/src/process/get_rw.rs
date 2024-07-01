use config::File;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_rw(account: &AccountWithCFs, rules: &AggRules, file: &File) -> String {
    // Here llg is the rw
    let rw = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => file.default_risk_weight,
    };
    rw.to_string()
}
