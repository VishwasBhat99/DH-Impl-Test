use crate::aggregator::config::File;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn get_llg(
    config_file: &File,
    account: &AccountWithCFs,
    rules: &AggRules,
    _logger: &Logger,
) -> i32 {
    let mut llg = match rules.llg_for_acc(account) {
        Some(val) => val.llg,
        None => config_file.default_llg_code(),
    };

    if (llg / 10000) > 0 {
        llg -= 10000;
    }
    llg
}
