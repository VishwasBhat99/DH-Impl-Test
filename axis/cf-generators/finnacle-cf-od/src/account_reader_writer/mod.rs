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

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_prin_in_inp = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut succ_rec = DEFAULT_INT;
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    loop {
        let account_opt = reader_iterator.next();

        if account_opt.is_none() {
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse record into Input Struct");
        tot_acc_encntrd += 1;
        succ_rec += 1;
        tot_prin_in_inp += input_account.out_bal_amt;

        let cashflows_for_account_result =
            generate_cashflows(&mut input_account, config_params, &log);

        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.acid,
                cashflows_for_account_result.err().expect("Unable to unwrap error.");
            );
            continue;
        }

        let cashflows = cashflows_for_account_result.expect("Unable to generate cashflows.");

        let account_with_cashflows = create_account_with_cashflows(input_account, cashflows);
        tot_prin_in_op += account_with_cashflows.out_bal_amt;

        writer.write(account_with_cashflows);
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
    health_stat.gen_health_rpt(config_params.output_file_path())
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
