use rbdate::NaiveDate;
use sdb_day_convention::Conventions;
use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use account_reader_writer::account_appender::create_account_without_cashflows;
use health_report::HealthReport;
use macros;
use statics::*;
use std::time::SystemTime;

pub fn generate(
    input_file_path: &str,
    output_file_path: &str,
    log: &Logger,
    diag_log: &Logger,
    as_on_date: &NaiveDate,
    convention: Conventions,
) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_principal = DEFAULT_FLOAT;
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);
    let mut reader_iterator = reader.into_iter();
    reader_iterator.next();
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

        let input_account = account_opt.expect("Unable to parse record into Input Struct.");
        total_accounts_encountered += 1;
        total_principal += if let Some(bal) = input_account.crnt_book_bal {
            bal
        } else {
            DEFAULT_FLOAT
        };

        let account_without_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.cod_acc_no
            )],
            create_account_without_cashflows(input_account, &log, as_on_date, convention)
        );
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_without_cashflows.cod_acc_no
            )],
            writer.write(account_without_cashflows)
        );
    }

    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:?}",
        total_accounts_encountered, total_duration, total_principal
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_encountered,
        0,
        total_principal,
        total_principal,
        0,
    );
    health_stat.gen_health_rpt(output_file_path);
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
