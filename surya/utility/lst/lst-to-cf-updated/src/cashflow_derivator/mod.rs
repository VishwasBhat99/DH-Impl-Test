use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod der_cashflows;
pub mod field_struct;
mod reference_data_handler;

use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::append_cashflows;
use self::append_write_cashflows::create_io_workers;
use self::append_write_cashflows::write_cashflows;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use rbdate::NaiveDate;
use statics::*;
use std::collections::HashMap;
use std::time::SystemTime;
pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut prev_acc_num: String = "".to_string();
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;

    //logic for using reference file
    let mut use_ref_file = use_reference_file(config_params.ref_file_path());
    let mut fields = field_struct::Fields::default();
    let mut account_ref_data: HashMap<i64, Vec<String>> = HashMap::new();
    if use_ref_file {
        account_ref_data = read_ref_file(config_params.ref_file_path(), log);
        fields = field_struct::Fields::new_from_path(config_params.req_fields_file_path());
    }
    //logic for reference file ends
    let start_derive_timer = SystemTime::now();
    let mut cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithCashflows::new();
    let mut nxt_rep_dt: Option<NaiveDate> = None;
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
            tot_cfs += cfs.len();
            tot_prin_in_op += a_w_cf.total_principal_amount;
            tot_int_in_op += a_w_cf.total_interest_amount;
            if use_ref_file {
                reference_data_handler::add_reference_file_values(
                    &mut a_w_cf,
                    &fields,
                    &account_ref_data,
                    log,
                );
            }
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_prin_in_ip += input_account.amount;
        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.acc_num
            )],
            derive_cashflows(&mut input_account, &config_params, log)
        );

        if prev_acc_num == "" {
            tot_acc_encntrd += 1;
            tot_acc_with_cfs += 1;
            prev_acc_num = input_account.acc_num.to_string();
            nxt_rep_dt = input_account.reprice_dt;
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
        } else if prev_acc_num != input_account.acc_num {
            tot_acc_encntrd += 1;
            tot_acc_with_cfs += 1;
            tot_cfs += cfs.len();
            tot_prin_in_op += a_w_cf.total_principal_amount;
            tot_int_in_op += a_w_cf.total_interest_amount;
            if use_ref_file {
                reference_data_handler::add_reference_file_values(
                    &mut a_w_cf,
                    &fields,
                    &account_ref_data,
                    log,
                );
            }
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            cfs.clear();
            cfs.push(cashflows);
            nxt_rep_dt = input_account.reprice_dt;
            a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
            prev_acc_num = input_account.acc_num.to_string();
        } else {
            if nxt_rep_dt < input_account.reprice_dt {
                nxt_rep_dt = input_account.reprice_dt;
            }
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
        }
    }

    writer.close();

    let end_derive_timer = SystemTime::now();
    let tot_duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total duration for derive timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn read_ref_file(path: &str, logger: &Logger) -> HashMap<i64, Vec<String>> {
    let mut account_data_map: HashMap<i64, Vec<String>> = HashMap::new();
    let mut ref_file: Xlsx<_> = open_workbook(path).expect("Error while opening `Reference File`.");
    let sheet_name = ref_file
        .sheet_names()
        .first()
        .unwrap_or(&"Sheet1".to_string())
        .to_owned();
    if let Some(Ok(reader)) = ref_file.worksheet_range(sheet_name.as_str()) {
        for row in reader.rows().skip(1) {
            let acc_id = str_to_int(&row[0].to_string().as_str());
            let mut acc_data: Vec<String> = Vec::new();
            for element in row.iter().skip(1) {
                acc_data.push(element.to_string());
            }
            account_data_map.insert(acc_id, acc_data);
        }
    }
    account_data_map
}

pub fn str_to_flt(num: &str) -> f64 {
    num.parse().unwrap_or(DEFAULT_FLOAT)
}
pub fn str_to_int(num: &str) -> i64 {
    num.parse().unwrap_or(DEFAULT_INT)
}

fn use_reference_file(path: &str) -> bool {
    if path.is_empty() {
        false
    } else {
        true
    }
}
