use crate::cashflow_generator::cashflow_appender::create_account_with_cashflows;

use self::ei_biz::*;
use self::ln3_biz::*;
use self::non_ei_biz::*;
use rbdate::{NaiveDate, NaiveDateTime};
use slog::Logger;
use std::io::BufRead;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod ei_biz;
mod gen_cashflows;
mod io;
mod ln3_biz;
mod non_ei_biz;
mod tenor;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::{AccountWithCashflows, Cashflow};
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::tenor::*;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn generate(
    input_file_path: &str,
    output_file_path_principal: &str,
    as_on_date: &NaiveDate,
    log: &Logger,
    diag_log: &Logger,
) {
    //Mutable variables declared that will change during the course of the process.
    let mut g_cur_inst_date = NaiveDate::from_ymd(1907, 1, 1);
    let mut m_early_date = NaiveDate::from_ymd(1907, 1, 1);
    let mut m_prvs_inst_date = NaiveDate::from_ymd(1907, 1, 1);
    let mut m_prvs_end_date = NaiveDate::from_ymd(1907, 1, 1);

    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut total_principal_in_input = 0.0;
    let mut total_principal_in_output = 0.0;
    let mut total_interest_in_output = 0.0;
    let mut writer_prin = AccountWithCashflowsWriter::new(output_file_path_principal, log);
    let mut writer_od =
        AccountWithCashflowsWriter::new(&format!("{}_overude", output_file_path_principal), log);

    let mut account_with_cashflows_prin: Vec<Cashflow> = Vec::new();
    let mut account_with_cashflows_od: Vec<Cashflow> = Vec::new();

    let start_time = SystemTime::now();
    let reader = InputAccountReader::new(input_file_path, log);

    let mut reader_iterator = reader.into_iter();
    //Vector to store same key records.
    let mut buffer_list: Vec<InputAccount> = Vec::new();
    let mut key = String::new();

    let mut tot = 0.0;
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
        let mut input_account = account_opt.expect("Cannot read the record data.");
        input_account.end_date = rbdate::NaiveDate::from_ymd(3099, 1, 1);

        total_accounts_encountered += 1;
        //Check for first record.
        if buffer_list.is_empty() {
            key = input_account.acid.to_owned();
            buffer_list.push(input_account);
        } else if input_account.acid == buffer_list[0].acid {
            buffer_list.push(input_account);
        } else {
            process_ln3(
                buffer_list.clone(),
                *as_on_date,
                log,
                diag_log,
                &mut account_with_cashflows_prin,
                &mut account_with_cashflows_od,
                &mut m_prvs_inst_date,
                &mut m_prvs_end_date,
                &mut g_cur_inst_date,
            );
            let final_prin_op_acc = create_account_with_cashflows(
                buffer_list[0].clone(),
                account_with_cashflows_prin.clone(),
                "NA".to_string(),
            );

            if !account_with_cashflows_prin.is_empty() {
                writer_prin.write(final_prin_op_acc);
            }
            let final_od_op_acc = create_account_with_cashflows(
                buffer_list[0].clone(),
                account_with_cashflows_od.clone(),
                "NA".to_string(),
            );

            if !account_with_cashflows_od.is_empty() {
                writer_od.write(final_od_op_acc);
            }

            buffer_list.clear();
            account_with_cashflows_prin.clear();
            account_with_cashflows_od.clear();
            buffer_list.push(input_account);
            total_accounts_with_cashflows += 1;
        }
    }
    //Process the last set of records in the bufferlist.
    if !buffer_list.is_empty() {
        process_ln3(
            buffer_list.clone(),
            *as_on_date,
            log,
            diag_log,
            &mut account_with_cashflows_prin,
            &mut account_with_cashflows_od,
            &mut m_prvs_inst_date,
            &mut m_prvs_end_date,
            &mut g_cur_inst_date,
        );

        let final_prin_op_acc = create_account_with_cashflows(
            buffer_list[0].clone(),
            account_with_cashflows_prin.clone(),
            "NA".to_string(),
        );

        if !account_with_cashflows_prin.is_empty() {
            writer_prin.write(final_prin_op_acc);
        }

        let final_od_op_acc = create_account_with_cashflows(
            buffer_list[0].clone(),
            account_with_cashflows_od.clone(),
            "NA".to_string(),
        );

        if !account_with_cashflows_od.is_empty() {
            writer_od.write(final_od_op_acc);
        }

        buffer_list.clear();
        account_with_cashflows_prin.clear();
        account_with_cashflows_od.clear();
    }

    writer_prin.close();
    writer_od.close();

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
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        total_principal_in_input,
        total_principal_in_output,
        total_interest_in_output
    );
    log_info!(log, "{}", report_string);
    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        total_principal_in_input,
        total_principal_in_output,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(output_file_path_principal);
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}
