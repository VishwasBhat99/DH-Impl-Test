use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;

use self::account_reader::InputAccount;
use self::account_writer::AccountWithoutCashflows;
use cashflow_derivator::account_appender::create_account_without_cashflows;
use csv::{Reader, ReaderBuilder};
use health_report::HealthReport;
use macros;
use statics::*;
use std::env::current_dir;
use std::time::SystemTime;

pub fn generate(input_file_path: &str, output_file_path: &str, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_fld: i64 = DEFAULT_INT;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let start_generator_timer = SystemTime::now();
    let (mut reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);
    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    input_file_path,
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_acc_encntrd += 1;
        // Skip header
        if line_num == 0 {
            tot_acc_fld += 1;
            continue;
        }
        tot_prin_in_in += input_account
            .cf_amount
            .trim()
            .replace(",", "")
            .parse::<f64>()
            .unwrap_or(DEFAULT_FLOAT);
        let account_without_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.account_id
            )],
            create_account_without_cashflows(input_account, &log)
        );
        tot_prin_in_op += account_without_cashflows.cf_amount;
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_without_cashflows.account_id
            )],
            writer.write(account_without_cashflows)
        );
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    debug!(
        log,
        "Total Duration for reading input file and writing .cf file: `{:?}`.", total_duration
    );

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_fld,
        tot_acc_fld,
        tot_prin_in_in,
        tot_prin_in_op,
        0,
    );
    log_info!(log, "{}", health_stat.display());
    println!("{}", health_stat.display());
    health_stat.gen_health_rpt(output_file_path)
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (Reader<std::fs::File>, AccountWithoutCashflows) {
    let reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(input_path)
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            input_path,
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let writer = AccountWithoutCashflows::new(output_path, log);

    (reader, writer)
}
