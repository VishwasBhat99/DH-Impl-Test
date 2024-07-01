use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use cashflow_derivator::account_appender::create_account_without_cashflows;
use health_report::HealthReport;
use macros;
use statics::*;
use std::time::SystemTime;

pub fn generate(input_file_path: &str, output_file_path: &str, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_balance_in_ip = DEFAULT_FLOAT;
    let mut total_balance_in_op = DEFAULT_FLOAT;
    let mut line_number: i64 = DEFAULT_INT;
    let start_generator_timer = SystemTime::now();
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

        let input_account = account_opt.expect("Unable to parse record into Input Struct");
        total_accounts_encountered += 1;

        total_balance_in_ip += input_account.cf_amount;
        line_number += 1;

        let account_without_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                line_number
            )],
            create_account_without_cashflows(input_account, &log)
        );
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                line_number
            )],
            writer.write(account_without_cashflows.clone(), line_number)
        );

        total_balance_in_op += account_without_cashflows.cf_amount;
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Total Duration: {:?}\n\
         Total Balance in input: {:?}\n\
         Total Balance in output: {:?}",
        total_accounts_encountered, total_duration, total_balance_in_ip, total_balance_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        line_number,
        line_number,
        line_number - total_accounts_encountered,
        total_balance_in_ip,
        total_balance_in_op,
        0 as i64,
    );
    health_stat.gen_health_rpt(output_file_path)
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithoutCashflows) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithoutCashflows::new(output_path, log);

    (reader, writer)
}
