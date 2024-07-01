use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod derive_cashflows;

use self::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::derive_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use std::time::SystemTime;

pub fn derive(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut total_principal_in_input = 0.0;
    let mut total_principal_in_output = 0.0;
    let (reader, mut writer) = create_io_workers(
        config_param.input_file_path(),
        config_param.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;

    // Skip the header
    if config_param.is_header_present() {
        reader_iterator.next();
    }

    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );
        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Failed to parse 'record'.");

        total_accounts_encountered += 1;
        total_principal_in_input += input_account.total_outstanding;

        let cashflows = log_measurements!(
            diag_log,
            [format!("Type: GenCFs, Identifier: {}", input_account.org)],
            derive_cashflows(*config_param.as_on_date(), &input_account)
        );

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.org
            )],
            create_account_with_cashflows(input_account, cashflows)
        );
        total_principal_in_output += account_with_cashflows.total_outstanding;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.org
            )],
            writer.write(account_with_cashflows)
        );
    }

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total outstanding amount in input: {:?} \n\
         Total outstanding amount in output: {:?}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_principal_in_input,
        total_principal_in_output,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_accounts_encountered - total_accounts_with_cashflows,
        total_principal_in_input,
        total_principal_in_output,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
}

// MARK: Helper functions

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
