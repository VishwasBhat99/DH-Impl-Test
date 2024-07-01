use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_sub_id(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> i32 {
    // Here llg is the rw
    let sub_id = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => *config_params.default_sub_claim_id(),
    };
    sub_id
}
