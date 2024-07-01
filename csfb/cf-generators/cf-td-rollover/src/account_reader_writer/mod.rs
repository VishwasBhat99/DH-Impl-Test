use slog::Logger;
mod account_appender;
mod account_reader;
mod account_with_cashflows;
mod account_writer;
mod gen_cashflows;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithCashflows;
use self::gen_cashflows::generate_cashflows;
use account_reader_writer::account_appender::create_account_with_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;
    let mut tot_amt = DEFAULT_FLOAT;
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
        tot_amt += input_account.curr_outstanding_bal;

        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: {}",
                input_account.account_number
            )],
            generate_cashflows(&mut input_account, log, config_params.rollover(),&config_params.as_on_date())
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

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.account_number
            )],
            create_account_with_cashflows(input_account, cashflows)
        );
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

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!("Total Duration: {:?}", total_duration,);
    log_debug!(log, "{}", report_string);

    let health_report = HealthReport::new(
        tot_rec,
        tot_rec - skp_rec,
        skp_rec,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflows) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflows::new(output_path, log);

    (reader, writer)
}
