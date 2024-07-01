use calamine::{open_workbook, Reader, Xlsx};
use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod cf_date_picker;
mod der_cashflows;

use self::cf_date_picker::CashflowDates;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{date_from_timestamp, NaiveDate};
use statics::*;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;

    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;
    let mut skp_rec = DEFAULT_INT;

    let start_time = SystemTime::now();

    let mut cf_date: HashMap<String, CashflowDates> = HashMap::new();
    let def_dt = NaiveDate::from_ymd(2099, 01, 01);
    let mut ref_excel1: Xlsx<_> = open_workbook(config_params.sub_dept_file_path())
        .expect("Unable to open Subordinate Dept File.");
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_params.sub_dept_sheet_name()) {
        for row in reader.rows() {
            let dates: CashflowDates = CashflowDates {
                call_dt: datevalue_to_naive_date(row[4].to_string()).unwrap_or(def_dt),
                put_dt: datevalue_to_naive_date(row[3].to_string()).unwrap_or(def_dt),
                mat_dt: def_dt,
            };
            cf_date.insert(row[0].to_string(), dates);
        }
    }

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );

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
            skp_rec += 1;
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;
        if let Some(amt) = input_account.orgballcy {
            tot_prin_in_ip += amt;
        }
        if let Some(amt) = input_account.int_amt {
            tot_int_in_ip += amt;
        }

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.deal_no
            )],
            derive_cashflows(
                &mut input_account,
                cf_date.clone(),
                *config_params.as_on_date(),
                log
            )
        );

        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();

        let a_w_cf = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.deal_no
            )],
            create_account_with_cashflows(input_account, cashflows)
        );

        tot_prin_in_op += a_w_cf.total_principal_amount;
        tot_int_in_op += a_w_cf.total_interest_amount;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                a_w_cf.deal_no
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
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd,
        skp_rec -1,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
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

fn datevalue_to_naive_date(date: String) -> Option<NaiveDate> {
    if let Ok(timestamp) = date.parse::<f64>() {
        Some(date_from_timestamp(((timestamp as i64) - 25569) * 86400))
    } else {
        None
    }
}
