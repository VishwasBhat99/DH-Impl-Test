use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_spec_risk_cap_prcnt(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> i32 {
    let val = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => *config_params.default_spec_risk_cap_prcnt(),
    };
    val
}

pub fn get_gen_mr_rule_prcnt(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> i32 {
    let val = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => *config_params.default_gen_mr_rule_prcnt(),
    };
    val
}

pub fn get_llg(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> i32 {
    let val = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => *config_params.default_llg(),
    };
    val
}
