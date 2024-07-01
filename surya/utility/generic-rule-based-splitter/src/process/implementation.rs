use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

#[allow(dead_code, unused_imports)]
pub fn llg_for_account(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> i32 {
    let mut category = match rules.llg_for_acc(account) {
        Some(c) => c.llg,
        None => config_params.default_llg_code(),
    };

    let cf_type = {
        let cf_type_id = category / 10000;
        if cf_type_id == 1 {
            "I".to_string()
        } else {
            "O".to_string()
        }
    };
    if cf_type == "I" {
        category -= 10000;
    }

    category
}
