use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod der_cashflows;

use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use std::time::SystemTime;

pub fn derive(
    input_file_path: &str,
    output_file_path: &str,
    as_on_date: &NaiveDate,
    log: &Logger,
    diag_log: &Logger,
) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;

    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;

    let start_time = SystemTime::now();

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

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;
        tot_prin_in_ip += input_account.prin_amt;
        tot_int_in_ip += input_account.int_amt;

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.deal_id
            )],
            derive_cashflows(&mut input_account, log, as_on_date)
        );
        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();

        let a_w_cf = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.deal_id
            )],
            create_account_with_cashflows(input_account, cashflows)
        );

        tot_prin_in_op += a_w_cf.tot_prin_amt;
        tot_int_in_op += a_w_cf.tot_int_amt;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                a_w_cf.deal_id
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
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_report.gen_health_rpt(output_file_path);

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
