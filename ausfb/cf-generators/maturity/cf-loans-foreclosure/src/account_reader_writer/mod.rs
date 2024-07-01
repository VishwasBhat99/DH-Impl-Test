use chrono::{NaiveDate, Duration, Datelike};
use slog::Logger;
mod account_appender;
mod account_reader;
mod account_with_cashflows;
mod account_writer;
use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use self::structs::CashflowData;
use crate::account_reader_writer::account_reader::input_account::InputAccount;
use account_reader_writer::account_appender::create_account_with_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

mod derive_cashflow;
mod structs;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_prin_in_inp = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut succ_rec = DEFAULT_INT;

    let start_generator_timer = SystemTime::now();

    // Read the cashflow file
    let cashflow_file_path = Path::new(config_params.cashflow_file_path());
    let cashflow_file = File::open(&cashflow_file_path).expect("Unable to open cashflow file");
    let cashflow_reader = BufReader::new(cashflow_file);

    // Create a hashmap to store the cashflow data
    let mut cashflow_data: HashMap<String, Vec<CashflowData>> = HashMap::new();

    // Read the lines of the input file
    for line in cashflow_reader.lines().skip(1) {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split('|').collect();

        // Parse the fields
        let account_num = fields[0].trim().replace("'", "").to_string();
        let interest_flow = fields[9].trim().parse::<f64>().unwrap();
        let principal_flow = fields[8].trim().parse::<f64>().unwrap();
        let flow_date = update_year(fields[7].trim(),config_params.as_on_date());

        // Create a new cashflow
        let cashflow = CashflowData {
            interest_flow,
            principal_flow,
            flow_date,
        };

        // Add the cashflow to the account's vector
        let account_cashflows = cashflow_data.entry(account_num).or_insert(vec![]);
        account_cashflows.push(cashflow);
    }

    // Sort each account's cashflows by cash flow date
    for cashflows in cashflow_data.values_mut() {
        cashflows.sort_by(|a, b| a.flow_date.cmp(&b.flow_date));
    }
    // Reading of file and sorting ends.
    log_debug!(log,"cod_acct_no|principal_flow|interest_flow|old_sch_pymt|new_prin_out_bal|foreclosure|new_sch_pymt|new_int_flow|new_prin_flow|new_cls_bal|foreclosure_amt|revised_cls_bal|revised_prin_flow|cf_date");
    // Read input file
    let (reader, _writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );

    let mut reader_iterator = reader.into_iter();
    let mut writer = AccountWithoutCashflows::new(config_params.output_file_path(), log);
    // default vec of cashflow data
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }
        tot_acc_encntrd += 1;
        let input_account =
            InputAccount::new_from_acc(account_opt.expect("Cannot get the input account"));
        let def_cashflow = vec![CashflowData {
            principal_flow: 0.0,
            interest_flow: 0.0,
            flow_date: input_account.maturity_date.unwrap(),
        }];
        let cashflow_vec = cashflow_data
            .get(&input_account.cod_acct_no)
            .unwrap_or(&def_cashflow);
        for cf in cashflow_vec {
            tot_prin_in_inp += cf.principal_flow;
            tot_prin_in_op += cf.principal_flow;
        }
        let acc_with_casflow = create_account_with_cashflows(cashflow_vec, input_account, config_params.as_on_date(),log);

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                acc_with_casflow.customer_id
            )],
            writer.write(acc_with_casflow.to_owned())
        );
        succ_rec += 1;
    }
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


fn update_year(date_string: &str, as_on_date: &NaiveDate) -> NaiveDate {
    let date = NaiveDate::parse_from_str(date_string, "%d-%m-%y").unwrap();
    let current_year = date.year() % 100;
    let current_century = (as_on_date.year() / 100) * 100;
    let final_year = current_century + current_year;
    let updated_date = NaiveDate::from_ymd(final_year, date.month(), date.day());
    updated_date
}
