use super::bkt_def::BktData;
use crate::configuration_parameters::ConfigurationParameters;
use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_without_cashflows::AccountWithoutCashflows;
use cashflow_derivator::account_without_cashflows_writer::AccountWithoutCashflowsWriter;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn append_cashflows(
    diag_log: &Logger,
    input_account: &InputAccount,
    config_params: &ConfigurationParameters,
    mapping_master_map: &HashMap<String, String>,
    bkt_def_vec: &Vec<BktData>,
) -> AccountWithoutCashflows {
    let account_with_cashflows = log_measurements!(
        diag_log,
        [format!(
            "Type: CreateAccWithCFs, Identifier: {}",
            input_account.customer_no
        )],
        create_account_with_cashflows(
            input_account.clone(),
            config_params,
            mapping_master_map,
            bkt_def_vec
        )
    );
    account_with_cashflows
}

pub fn write_cashflows(
    writer: &mut AccountWithoutCashflowsWriter,
    log: &Logger,
    diag_log: &Logger,
    account_with_cashflows: &AccountWithoutCashflows,
) {
    log_debug!(log, "Account: `{}`", account_with_cashflows.customer_no,);
    log_measurements!(
        diag_log,
        [format!(
            "Type: WriteAccWithCFs, Identifier: {}",
            account_with_cashflows.customer_no
        )],
        writer.write(account_with_cashflows.clone())
    );
}
pub fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithoutCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithoutCashflowsWriter::new(output_path, log);

    (reader, writer)
}
