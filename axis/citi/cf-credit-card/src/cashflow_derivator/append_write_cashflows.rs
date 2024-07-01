use cashflow_derivator::account_reader::input_account::InputAccount;
use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows::AccountWithCashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use macros;
use slog::Logger;
use std::time::SystemTime;

pub fn append_cashflows(
    diag_log: &Logger,
    input_account: &InputAccount,
    cfs: &[Cashflow],
) -> AccountWithCashflows {
    let account_with_cashflows = log_measurements!(
        diag_log,
        [format!(
            "Type: CreateAccWithCFs, Identifier: {}",
            input_account.account_number
        )],
        create_account_with_cashflows(input_account.clone(), cfs.to_vec())
    );
    account_with_cashflows
}

pub fn write_cashflows(
    writer: &mut AccountWithCashflowsWriter,
    log: &Logger,
    diag_log: &Logger,
    account_with_cashflows: &AccountWithCashflows,
) {
    log_debug!(
        log,
        "Account: `{}`, cashflows: `{:?}`, interest rate: `{}`.",
        account_with_cashflows.account_number,
        account_with_cashflows.cashflows,
        account_with_cashflows.rate_of_int
    );
    log_measurements!(
        diag_log,
        [format!(
            "Type: WriteAccWithCFs, Identifier: {}",
            account_with_cashflows.account_number
        )],
        writer.write(account_with_cashflows.clone())
    );
}

pub fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
