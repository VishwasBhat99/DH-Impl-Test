use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod gen_cashflows;
mod io;
mod payment;

use crate::statics::*;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::gen_cashflows::generate_cashflows;
use cashflow_generator::payment::PaymentDetails;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_day_convention::conventions::Conventions;
use std::time::SystemTime;

pub fn generate(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut tot_prin_in_inp = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut succ_rec = DEFAULT_INT;

    let start_time = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_param.input_file_path(),
        config_param.output_file_path(),
        log,
    );

    let mut reader_iterator = reader.into_iter();
    let mut prev_desc = String::new();
    let mut prev_amt = 0.0;
    let mut count = 0;
    let mut interest_rate_due_date: Vec<PaymentDetails> = Vec::new();
    let mut flag = true;

    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: {}",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        let input_account = account_opt.expect("Unable to parse record into Input Struct");

        total_accounts_encountered += 1;
        succ_rec += 1;
        tot_prin_in_inp += input_account.current_balance_amount;

        if flag == true {
            prev_desc = input_account.description;
            prev_amt = input_account.current_balance_amount;
            count += 1;
            let int_due_date =
                PaymentDetails::assignvalues(input_account.int_rate, input_account.due_date);
            interest_rate_due_date.push(int_due_date);
            flag = false;
            continue;
        }
        let current_desc = input_account.description.to_owned();
        let current_amt = input_account.current_balance_amount;

        if current_desc == prev_desc && current_amt == prev_amt {
            count += 1;
            let int_due_date =
                PaymentDetails::assignvalues(input_account.int_rate, input_account.due_date);
            interest_rate_due_date.push(int_due_date);
        } else {
            log_measurements!(
                diag_log,
                [format!(
                    "Type: GenCFs, Identifier: {}",
                    input_account.description
                )],
                generate_cashflows(
                    config_param.as_on_date(),
                    prev_amt,
                    prev_desc,
                    &interest_rate_due_date,
                    count,
                    &mut writer,
                    log,
                    config_param.day_convention(),
                )
            );

            interest_rate_due_date.clear();

            prev_desc = input_account.description;
            prev_amt = input_account.current_balance_amount;
            count = 1;
            let int_due_date =
                PaymentDetails::assignvalues(input_account.int_rate, input_account.due_date);
            interest_rate_due_date.push(int_due_date);
        }
    }
    generate_cashflows(
        config_param.as_on_date(),
        prev_amt,
        prev_desc,
        &interest_rate_due_date,
        count,
        &mut writer,
        log,
        config_param.day_convention(),
    );
    writer.close();
    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    log_debug!(log, "Total Duration: {:?}", total_duration);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        succ_rec,
        total_accounts_encountered - succ_rec,
        tot_prin_in_inp,
        tot_prin_in_op,
        0,
    );
    println!("{}", health_stat.display());
    info!(log, "{}", health_stat.display());
    health_stat.gen_health_rpt(config_param.output_file_path());
}

// MARK: Helper functions

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
