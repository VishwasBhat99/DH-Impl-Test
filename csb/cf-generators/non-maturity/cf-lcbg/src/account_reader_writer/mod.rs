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
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_prin_in_inp = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut succ_rec = DEFAULT_INT;
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);
    let mut reader_iterator = reader.into_iter();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                tot_acc_encntrd
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }

        let input_account = account_opt.expect("Unable to parse record into Input Struct");
        tot_acc_encntrd += 1;
        succ_rec += 1;
        tot_prin_in_inp += input_account.bal_os;

        let account_without_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.acc_num
            )],
            create_account_without_cashflows(input_account)
        );
        tot_prin_in_op += account_without_cashflows.bal_os;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_without_cashflows.acc_num
            )],
            writer.write(account_without_cashflows)
        );
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    log_debug!(log, "Total Duration: {:?}", total_duration);

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        succ_rec,
        tot_acc_encntrd - succ_rec,
        tot_prin_in_inp,
        tot_prin_in_op,
        0,
    );
    println!("{}", health_stat.display());
    info!(log, "{}", health_stat.display());
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
