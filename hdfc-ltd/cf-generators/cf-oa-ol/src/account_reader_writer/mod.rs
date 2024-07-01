use slog::Logger;
mod account_appender;
mod account_reader;
mod account_with_cashflows;
mod account_writer;
mod gen_cashflows;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use account_reader_writer::account_appender::create_account_with_cashflows;
use account_reader_writer::gen_cashflows::generate_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::time::SystemTime;

pub fn generate(
    input_file_path: &str,
    output_file_path: &str,
    config_params: &ConfigurationParameters,
    log: &Logger,
    diag_log: &Logger,
) {
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);
    let mut reader_iterator = reader.into_iter();
    let mut tot_rec = 0;
    let mut total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let mut total_cfs: usize = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
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

        let mut input_account = account_opt.expect("Unable to parse record into Input Struct");
        tot_rec += 1;
        tot_amt += input_account.amt;

        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!("Type: GenCFs, Identifier: `{}`", input_account.id)],
            generate_cashflows(&mut input_account, config_params, &log)
        );

        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.id,
                cashflows_for_account_result.err().expect("Unable to unwrap error.");
            );
            continue;
        }

        let cashflows = cashflows_for_account_result.expect("Unable to generate cashflows.");

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.id
            )],
            create_account_with_cashflows(input_account, cashflows, &log)
        );
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.id
            )],
            writer.write(account_with_cashflows)
        );
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!("Total Duration: {:?}", total_duration,);
    log_debug!(log, "{}", report_string);

    let health_report = HealthReport::new(
        tot_rec,
        total_accounts_with_cashflows,
        skp_rec,
        tot_amt,
        tot_amt,
        total_cfs as i64,
    );
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
