use configuration_parameters::ConfigurationParameters;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn get_balm_llg(
    config_params: &ConfigurationParameters,
    account: &AccountWithCFs,
    rules: &AggRules,
    _logger: &Logger,
) -> i32 {
    let mut balm_llg = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => config_params.balm_default_llg(),
    };

    if (balm_llg / 10000) > 0 {
        balm_llg -= 10000;
    }
    balm_llg
}
