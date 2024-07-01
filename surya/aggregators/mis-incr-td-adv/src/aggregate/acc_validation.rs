use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_acc_validation(
    process_wd: bool,
    process_field: &String,
    nwd_prod_codes: &Vec<String>,
    to_skip: bool,
) -> bool {
    if process_wd {
        if nwd_prod_codes.contains(&process_field) || to_skip {
            return false;
        } else {
            return true;
        }
    } else {
        if !nwd_prod_codes.contains(&process_field) || to_skip {
            return false;
        } else {
            return true;
        }
    }
}

pub fn skip_account(account: &AccountWithCFs, rules: &AggRules) -> bool {
    let skip_field = match rules.llg_for_acc(account) {
        Some(_) => true,
        None => false,
    };
    skip_field
}
