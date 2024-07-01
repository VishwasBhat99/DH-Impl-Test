use cashflow_generator::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::gen_cashflows::generate_cashflows;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;

use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use health_report::HealthReport;
use macros;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use std::time::SystemTime;

pub fn derive(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;

    let mut tot_prin_in_ip = DEFAULT_FLOAT;

    let start_time = SystemTime::now();

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut tot_prin_amt = DEFAULT_FLOAT;
    let mut tot_int_amt = DEFAULT_FLOAT;
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

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;

        tot_prin_in_ip += input_account.os_bal;

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.deal_num
            )],
            generate_cashflows(&mut input_account, config_params, log)
        );
        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();

        let a_w_cf = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.deal_num
            )],
            create_account_with_cashflows(
                input_account,
                &mut tot_prin_amt,
                &mut tot_int_amt,
                cashflows
            )
        );

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                a_w_cf.deal_num
            )],
            writer.write(a_w_cf)
        );
    }
    writer.close();

    let end_time = SystemTime::now();
    let tot_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Total Duration: {:?}\n\
         Total outstanding amount in output: {:.2} \n\
         Total interest amount in output: {:.2}",
        tot_duration, tot_prin_amt, tot_int_amt
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_ip,
        tot_cfs as i64,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());

    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}

pub fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);
    (reader, writer)
}
