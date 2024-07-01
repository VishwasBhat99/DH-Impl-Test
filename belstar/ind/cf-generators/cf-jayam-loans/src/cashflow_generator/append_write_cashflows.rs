use crate::configuration_parameters::ConfigurationParameters;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use macros;
use slog::Logger;
use std::time::SystemTime;

pub fn append_cashflows(
    diag_log: &Logger,
    input_account: &InputAccount,
    config_params: &ConfigurationParameters,
    log: &Logger,
) -> AccountWithCashflows {
    let account_with_cashflows = log_measurements!(
        diag_log,
        [format!(
            "Type: CreateAccWithCFs, Identifier: {}",
            input_account.loan_acc_no
        )],
        create_account_with_cashflows(input_account.clone(), config_params, log,)
    );
    account_with_cashflows
}

pub fn write_cashflows(
    writer: &mut AccountWithCashflowsWriter,
    log: &Logger,
    diag_log: &Logger,
    account_with_cashflows: &AccountWithCashflows,
) {
    log_measurements!(
        diag_log,
        [format!(
            "Type: WriteAccWithCFs, Identifier: {}",
            account_with_cashflows.loan_acc_no
        )],
        writer.write(account_with_cashflows.clone())
    );
}

pub fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (
    InputAccountReader,
    AccountWithCashflowsWriter,
    AccountWithCashflowsWriter,
) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);
    let output_path_sec = output_path.to_string() + "_securitized";
    let writer_sec = AccountWithCashflowsWriter::new(&output_path_sec, log);

    (reader, writer, writer_sec)
}
