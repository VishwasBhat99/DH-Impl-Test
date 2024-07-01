use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;

pub fn get_ccf_prcnt(
    account: &AccountWithCFs,
    rules: &AggRules,
    config_params: &ConfigurationParameters,
) -> (String, f64) {
    // Get CCF Value from Rules
    let (ccf_rule_id, ccf_prcnt) = match rules.llg_for_acc(account) {
        Some(val) => (
            format!("{:0>8}", val.llg.to_string())[4..8].to_string(),
            val.llg as f64,
        ),
        None => (
            format!("{:0>4}", *config_params.default_ccf_prcnt()).to_string(),
            *config_params.default_ccf_prcnt(),
        ),
    };
    (ccf_rule_id, ccf_prcnt)
}
