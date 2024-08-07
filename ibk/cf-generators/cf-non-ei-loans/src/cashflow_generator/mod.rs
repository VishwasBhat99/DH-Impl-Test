use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;
mod get_holiday_data;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use cashflow_generator::get_holiday_data::get_holiday_map;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use itertools::Itertools;
use macros;
use sdb_io::new_buf_rdr;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
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

    let repayment_file = match new_buf_rdr(config_params.repayment_struct_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.repayment_struct_file(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut res_file_date: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (line_num, lines) in repayment_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.repayment_struct_file(),
                line_num + 1,
                error
            ),
        };
        if config_params.is_perf_diagnostics_enabled() {
            info!(
                log,
                "Processing line: {:?} and line-no: {:?} from Res-File", line, line_num,
            );
        }
        let mut fields: Vec<String> = Vec::new();
        for component in line.split('|') {
            fields.push(component.to_string());
        }

        let key = fields[0].to_owned();
        if res_file_date.contains_key(&fields[0]) {
            res_file_date
                .get_mut(&fields[0])
                .as_mut()
                .unwrap()
                .push(fields);
        } else {
            let mut cf_val: Vec<Vec<String>> = Vec::new();
            cf_val.push(fields);
            res_file_date.insert(key.to_string(), cf_val);
        }
    }
    let mut holiday_map: HashMap<rbdate::NaiveDate, String> = HashMap::new();
    let holiday_file = match new_buf_rdr(config_params.holiday_yearrccy_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.holiday_yearrccy_file(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in holiday_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.holiday_yearrccy_file(),
                line_num + 1,
                error
            ),
        };
        let mut fields: Vec<&str> = line.split('|').collect();
        get_holiday_map(&mut fields, config_params, &mut holiday_map);
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
        if config_params.is_perf_diagnostics_enabled() {
            info!(log, "Processing line: {:?} from Master-File", input_account);
        }
        total_accounts_encountered += 1;
        // if let Some(dis_amt) = input_account.dis_amt {
        tot_prin_in_in += input_account.dis_amt;
        // }
        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.acid
            )],
            generate_cashflows(
                &mut input_account,
                config_params,
                &log,
                &res_file_date,
                &mut holiday_map
            )
        );

        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.acid,
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
                input_account.acid
            )],
            create_account_with_cashflows(
                input_account,
                cashflows,
                config_params,
                &mut holiday_map
            )
        );

        tot_prin_in_op += account_with_cashflows.dis_amt;
        tot_int_in_op += 0.0;
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
