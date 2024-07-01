use macros;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub fn get_llg_id(
    account: &AccountWithCFs,
    account_id: &String,
    rules: &AggRules,
    default_stamp_code: i32,
    log: &Logger,
) -> i32 {
    let llg_id = match rules.llg_for_acc(account) {
        Some(c) => c.llg,
        None => {
            log_error!(
                &log,
                "Default Stamp code {} used for stamping account {}",
                default_stamp_code,
                account_id
            );
            default_stamp_code
        }
    };

    llg_id
}
