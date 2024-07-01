use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_rw(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> i32 {
    // Here llg is the rw
    let rw = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => *config_params.default_risk_weight(),
    };
    rw
}
