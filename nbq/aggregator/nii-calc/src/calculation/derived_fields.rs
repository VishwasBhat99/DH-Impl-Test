use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(account: &AccountWithCFs, rules: &AggRules, def_llg: i32) -> i32 {
    let mut llg_id = match rules.llg_for_acc(account) {
        Some(data) => data.llg,
        None => def_llg,
    };
    let cf_type = {
        let cf_type_id = llg_id / 10000;
        if cf_type_id == 1 {
            "I".to_string()
        } else {
            "O".to_string()
        }
    };
    if cf_type == "I" {
        llg_id = llg_id - 10000;
    }

    llg_id
}

pub fn read_field(account: &AccountWithCFs, field_name: &str) -> String {
    match account.get_f64_for_key(&field_name.to_string()) {
        Ok(val) => val.to_string(),
        Err(_) => match account.get_f32_for_key(&field_name.to_string()) {
            Ok(val) => val.to_string(),
            Err(_) => match account.get_i64_for_key(&field_name.to_string()) {
                Ok(val) => val.to_string(),
                Err(_) => match account.get_i32_for_key(&field_name.to_string()) {
                    Ok(val) => val.to_string(),
                    Err(_) => match account.get_string_for_key(&field_name.to_string()) {
                        Ok(val) => val.to_string(),
                        Err(_) => String::from("NA"),
                    },
                },
            },
        },
    }
}
