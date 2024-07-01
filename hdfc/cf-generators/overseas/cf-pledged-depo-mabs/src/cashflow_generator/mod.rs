use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod cf_date_iterator;
mod duration_extensions;
mod gen_cashflows;
mod io;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use std::time::SystemTime;

pub fn generate(
    input_file_path: &str,
    output_file_path: &str,
    as_on_date: &NaiveDate,
    day_convention: &Conventions,
    log: &Logger,
    diag_log: &Logger,
) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;

    let mut total_principal_in_input = 0.0;
    let mut total_principal_in_output = 0.0;
    let mut total_interest_in_output = 0.0;

    let start_time = SystemTime::now();
    let (reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);

    let mut reader_iterator = reader.into_iter();

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
        let mut input_account = account_opt.unwrap();
        // Get rid of this total_accounts_encountered
        total_accounts_encountered += 1;
        total_principal_in_input += input_account.current_book_balance;

        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: {}",
                input_account.account_number
            )],
            generate_cashflows(as_on_date, &mut input_account, log, day_convention)
        );

        // If this account didn't generate cashflows due to an error
        // log the error and move on to the next account.
        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: '{}'. Error: {}",
                input_account.account_number,
                cashflows_for_account_result.err().unwrap();
            );
            continue;
        }

        let cashflows = cashflows_for_account_result.unwrap();

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.account_number
            )],
            create_account_with_cashflows(input_account, cashflows)
        );
        total_principal_in_output += account_with_cashflows.total_principal_amount;
        total_interest_in_output += account_with_cashflows.total_interest_amount;
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.account_number
            )],
            writer.write(account_with_cashflows)
        );
    }

    writer.close();

    let end_time = SystemTime::now();
    let total_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in output: {:.2}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        total_principal_in_input,
        total_principal_in_output,
        total_interest_in_output
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        total_principal_in_input,
        total_principal_in_output,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(output_file_path);
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
