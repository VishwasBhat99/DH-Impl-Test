mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod derive_cashflows;

use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::derive_cashflows::derive_cashflows;
use chrono::NaiveDate;
use health_report::HealthReport;
use macros;
use sdb_day_convention::Conventions;
use slog::Logger;

pub fn derive(input_file_path: &str, output_file_path: &str, as_on_dt: NaiveDate, convention: &Conventions, log: &Logger) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_ip = 0.0;
    let mut tot_prin_in_op = 0.0;

    let (reader, mut writer) = create_io_workers(input_file_path, output_file_path, log);
    let mut reader_iterator = reader;
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }

        let input_account = account_opt.expect("Unable to parse `record`.");
        total_accounts_encountered += 1;
        tot_prin_in_ip += input_account.settle_amt_1st_leg;

        let cashflows = derive_cashflows(&input_account, as_on_dt, *convention, log);
        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let a_w_cf = create_account_with_cashflows(input_account, cashflows);
        tot_prin_in_op += a_w_cf.settle_amt_1st_leg;

        writer.write(a_w_cf)
    }

    writer.close();

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        tot_prin_in_ip,
        tot_prin_in_op,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(output_file_path)
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
