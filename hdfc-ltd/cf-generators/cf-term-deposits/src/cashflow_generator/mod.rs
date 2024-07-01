use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;
use self::account_reader::InputAccountReader;
use calamine::{open_workbook, Reader, Xlsx};
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::*;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;

    let start_generate_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut product_code: Xlsx<_> = open_workbook(&config_params.product_code_file_path())
        .expect("Could not find product file path");
    let product_code_sheetname = product_code
        .sheet_names()
        .first()
        .expect("excel is empty")
        .to_owned();
    let mut product_code_records: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = product_code.worksheet_range(product_code_sheetname.as_str()) {
        for record in reader.rows().skip(1) {
            product_code_records.insert(record[0].to_string(), record[1].to_string());
        }
    }
    let mut reader_iterator = reader.into_iter();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: `{}`",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse `record`.");
        total_accounts_encountered += 1;
        if let Some(current_outstanding_bal) = input_account.current_outstanding_bal {
            tot_prin_in_in += current_outstanding_bal;
        }
        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.acc_no
            )],
            generate_cashflows(
                &mut input_account,
                config_params,
                &log,
                &product_code_records
            )
        );

        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.acc_no,
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
                "Type: CreateAccWithCFs, Identifier: `{}`",
                input_account.acc_no
            )],
            create_account_with_cashflows(input_account, cashflows)
        );

        tot_prin_in_op += account_with_cashflows.current_outstanding_bal
            + account_with_cashflows.cum_interest
            + account_with_cashflows.interest_accured_amount;
        tot_int_in_op +=
            account_with_cashflows.cum_interest + account_with_cashflows.interest_accured_amount;
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.acc_no
            )],
            writer.write(account_with_cashflows)
        );
    }

    writer.close();

    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for generate timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:.2?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest amount in output: {:.2}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        tot_prin_in_in,
        tot_prin_in_op,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        tot_prin_in_in,
        tot_prin_in_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
