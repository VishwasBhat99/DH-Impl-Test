use self::ln2_biz::*;
use chrono::NaiveDate;
use slog::Logger;
use std::io::BufRead;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod calculate;
mod cashflow_appenders;
mod gen_cashflows;
mod ln2_biz;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn generate(
    input_file_path: &str,
    output_file_path_principal: &str,
    as_on_date: &NaiveDate,
    calc_ir_from_ason: String,
    log: &Logger,
    diag_log: &Logger,
) {
    //Mutable variables declared that will change during the course of the process.

    let mut total_accounts_encountered: i64 = 0;
    let mut total_cfs: usize = 0;
    let total_principal_in_input = 0.0;
    let total_principal_in_output = 0.0;
    let total_interest_in_output = 0.0;
    let mut writer_prin = AccountWithCashflowsWriter::new(output_file_path_principal, log);
    let mut account_with_cashflows_prin: Vec<AccountWithCashflows> = Vec::new();
    let mut account_with_cashflows_od: Vec<AccountWithCashflows> = Vec::new();

    let start_time = SystemTime::now();
    let reader = InputAccountReader::new(input_file_path, log);

    let mut reader_iterator = reader.into_iter();
    //Vector to store same key records.
    let mut buffer_list: Vec<InputAccount> = Vec::new();

    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        total_accounts_encountered += 1;
        let input_account =
            account_opt.expect("Unexpected error occured while unwraping account data");
        if buffer_list.is_empty() || input_account.acc_no == buffer_list[0].acc_no {
            buffer_list.push(input_account);
        } else {
            process_ln2(
                buffer_list.clone(),
                *as_on_date,
                calc_ir_from_ason.to_owned(),
                log,
                diag_log,
                &mut account_with_cashflows_prin,
                &mut account_with_cashflows_od,
            );
            for acc in account_with_cashflows_prin.iter() {
                if !acc.cashflows.is_empty() {
                    writer_prin.write(acc.to_owned());
                }
            }

            for acc in account_with_cashflows_od.iter() {
                if !acc.cashflows.is_empty() {
                    writer_prin.write(acc.to_owned());
                }
            }

            account_with_cashflows_prin.clear();
            account_with_cashflows_od.clear();
            buffer_list.clear();
            buffer_list.push(input_account);
            total_accounts_encountered += 1;
        }
    }
    if !buffer_list.is_empty() {
        process_ln2(
            buffer_list.clone(),
            *as_on_date,
            calc_ir_from_ason,
            log,
            diag_log,
            &mut account_with_cashflows_prin,
            &mut account_with_cashflows_od,
        );
        for acc in account_with_cashflows_prin.iter() {
            total_cfs += 1;
            if !acc.cashflows.is_empty() {
                writer_prin.write(acc.to_owned());
            }
        }
        for acc in account_with_cashflows_od.iter() {
            if !acc.cashflows.is_empty() {
                writer_prin.write(acc.to_owned());
            }
            total_cfs += 1;
        }
    }

    writer_prin.close();

    let end_time = SystemTime::now();
    let total_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in output: {:.2}",
        total_accounts_encountered,
        total_cfs,
        total_cfs,
        total_duration,
        total_principal_in_input,
        total_principal_in_output,
        total_interest_in_output
    );
    log_info!(log, "{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_cfs as i64,
        0,
        total_principal_in_input,
        total_principal_in_output,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(output_file_path_principal);
}
