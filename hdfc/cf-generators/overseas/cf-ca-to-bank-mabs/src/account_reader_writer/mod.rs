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

pub fn generate(input_file_path: &str, output_file_path: &str, log: &Logger, diag_log: &Logger) {
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);
    let mut reader_iterator = reader.into_iter();
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
    reader_iterator.next();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_rec
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }

        let input_account = account_opt.expect("Unable to parse record into Input Struct");
        tot_rec += 1;
        tot_amt += if let Some(bal) = input_account.crnt_book_bal {
            bal
        } else {
            DEFAULT_FLOAT
        };

        let account_without_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.casa_acc_no
            )],
            create_account_without_cashflows(input_account, &log)
        );
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_without_cashflows.casa_acc_no
            )],
            writer.write(account_without_cashflows)
        );
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!("Total Duration: {:?}", total_duration,);
    log_debug!(log, "{}", report_string);

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(output_file_path);
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
