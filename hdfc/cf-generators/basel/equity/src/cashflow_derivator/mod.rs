use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use cashflow_derivator::account_appender::create_account_without_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::io::*;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut tot_prin_in_in = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let index_rdr = match sdb_io::new_buf_rdr(config_params.index_file_path()) {
        Ok(r) => r,
        Err(e) => panic!(format!(
            "Cannot read file at path: '{}', Error: '{}'",
            config_params.index_file_path(),
            e
        )),
    };
    let mut isin_map: HashMap<String, String> = HashMap::new();
    for line in index_rdr.lines() {
        let info = line.expect("Unable to read file content.");
        let fields: Vec<&str> = info.split(',').collect();
        isin_map.insert(fields[0].to_string(), "Y".to_string());
    }

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
        tot_prin_in_in += input_account.book_value;

        let mut index_flag = String::new();
        if isin_map.contains_key(&input_account.isin) {
            index_flag = "Y".to_string();
        } else {
            index_flag = "N".to_string();
        }
        let account_without_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.deal_no
            )],
            create_account_without_cashflows(input_account, index_flag)
        );
        tot_prin_in_op += account_without_cashflows.book_value;
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_without_cashflows.deal_no
            )],
            writer.write(account_without_cashflows)
        );
    }
    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:?}\n\
         Total outstanding amount in output: {:?}",
        total_accounts_encountered, total_duration, tot_prin_in_in, tot_prin_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_encountered,
        0,
        tot_prin_in_in,
        tot_prin_in_op,
        0,
    );

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
