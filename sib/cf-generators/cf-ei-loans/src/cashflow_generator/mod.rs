use chrono::NaiveDate;
use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;

use crate::cashflow_generator::account_reader::input_account::{OverdueData, RepaymentData};

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use statics::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;

    let start_generate_timer = SystemTime::now();

    //Reading Repayment File
    let mut repay_map: HashMap<String, Vec<RepaymentData>> = HashMap::new();
    let repay_file = match new_buf_rdr(config_params.repayment_struct_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` due to: {}.",
            config_params.repayment_struct_file(),
            error
        ),
    };
    for (line_num, lines) in repay_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.repayment_struct_file(),
                line_num + 1,
                error
            ),
        };
        let repay_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let repay_data: RepaymentData = RepaymentData::new(
            config_params,
            config_params.repayment_struct_file(),
            &repay_vec,
            line_num + 1,
        );
        if repay_data.flow_id.to_uppercase() == "EIDEM" {
            repay_map
                .entry(repay_data.acid.to_string())
                .and_modify(|repay| repay.push(repay_data.clone()))
                .or_insert_with(|| vec![repay_data]);
        }
    }

    //Reading Overdue File
    let mut overdue_map: HashMap<String, Vec<(f64, NaiveDate)>> = HashMap::new();
    let overdue_file = match new_buf_rdr(config_params.overdue_input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` due to: {}.",
            config_params.overdue_input_file(),
            error
        ),
    };
    for (line_num, lines) in overdue_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.overdue_input_file(),
                line_num + 1,
                error
            ),
        };
        let overdue_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let overdue_data: OverdueData = OverdueData::new(
            config_params,
            config_params.overdue_input_file(),
            &overdue_vec,
            line_num + 1,
        );
        if overdue_data.ovd_date <= *config_params.as_on_date() {
            overdue_map
                .entry(overdue_data.acid.to_string())
                .and_modify(|ovd| ovd.push((overdue_data.ovd_amt, overdue_data.ovd_date)))
                .or_insert_with(|| vec![(overdue_data.ovd_amt, overdue_data.ovd_date)]);
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
        tot_prin_in_in += input_account.clr_bal_amt;
        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.acid
            )],
            generate_cashflows(
                &mut input_account,
                config_params,
                log,
                &repay_map,
                &overdue_map,
            )
        );

        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.acid,
                cashflows_for_account_result.expect_err("Unable to unwrap error.");
            );
            continue;
        }

        let cashflows = cashflows_for_account_result.expect("Unable to generate cashflows.");

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let (account_with_cashflows, tot_prin_amt, tot_int_amt) = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: `{}`",
                input_account.acid
            )],
            create_account_with_cashflows(input_account, cashflows, config_params,)
        );

        tot_prin_in_op += tot_prin_amt;
        tot_int_in_op += tot_int_amt;
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.acid
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
