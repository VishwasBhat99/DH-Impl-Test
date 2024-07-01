use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn get_llg_id(
    account: &AccountWithCFs,
    account_id: &String,
    rules: &AggRules,
    default_rl1: i32,
    log: &Logger,
) -> i32 {
    let llg_id = match rules.llg_for_acc(account) {
        Some(c) => c.llg,
        None => {
            log_error!(
                &log,
                "Default RL1 code {} used for stamping account {}",
                default_rl1,
                account_id
            );
            default_rl1
        }
    };

    llg_id
}
