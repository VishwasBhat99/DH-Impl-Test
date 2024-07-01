use aggregator::account::AccFieldNames;
use aggregator::llg::llg_key::LLGKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use rbdate::{date_from_timestamp, num_days_start_to_end};
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use slog::Logger;

pub mod llg_key;

pub fn llg_for_account(
    acc_no: String,
    account: &AccountWithCFs,
    field_names: &AccFieldNames,
    llg_rules: &AggRules,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> LLGKey {
    let curr_code = account
        .get_string_for_key(&field_names.inst)
        .expect("Error getting `currency`.");

    let llg_code = match llg_rules.llg_for_acc(account) {
        Some(c) => {
            log_debug!(
                logger,
                "Account `{}` evaluated to LLGId `{}`, using rule id `{}`",
                acc_no,
                c.llg,
                c.rule_id
            );
            c.llg
        }
        None => {
            log_debug!(
                logger,
                "Account `{}` defaulted to LLGId `{}`",
                acc_no,
                config_params.default_llg_code()
            );
            config_params.default_llg_code()
        }
    };

    let acc_open_dt = match account.get_i64_for_key(&field_names.acc_open_dt) {
        Ok(dt) => date_from_timestamp(dt),
        Err(error) => panic!(
            "`account_opening_date` is not well-formatted: `{:?}`",
            error
        ),
    };
    let mat_dt = match account.get_i64_for_key(&field_names.mat_dt) {
        Ok(dt) => date_from_timestamp(dt),
        Err(error) => panic!("`maturity_date` is not well-formatted: `{:?}`", error),
    };
    let tenor = num_days_start_to_end(acc_open_dt, mat_dt);

    LLGKey::new(llg_code, curr_code.to_string(), tenor)
}
