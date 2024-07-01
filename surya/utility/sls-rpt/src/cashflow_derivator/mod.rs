use slog::Logger;
mod account_appender;
mod account_reader;
mod account_without_cashflows;
mod account_writer;

use self::account_reader::InputAccountReader;
use self::account_writer::AccountWithoutCashflows;
use cashflow_derivator::account_appender::create_account_without_cashflows;
use configuration_parameters::ConfigurationParameters;
use macros;
use statics::*;
use std::time::SystemTime;

pub fn aggregate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let start_generator_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.sls_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let mut line_count=DEFAULT_INT;
    loop {
        //Skipping header
        if line_count == 0{
            log_info!(log, "Skipped headers at line number: {}", line_count+1);
            line_count+=1;
            continue;
        }
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
        let input_account = account_opt.expect("Unexpected error occured.");
        // Get rid of this total_accounts_encountered
        total_accounts_encountered += 1;

        let llg_id = input_account.llg_id.clone();
        let currency = input_account.currency.clone();
        let mut total_amt = DEFAULT_FLOAT;
        let range_start: usize = config_params
            .from_bkts()
            .parse::<usize>()
            .expect("Invalid start range value.");
        let range_end: usize = config_params
            .to_bkts()
            .parse::<usize>()
            .expect("Invalid end range value.");
        for (i, val) in input_account.values.into_iter().enumerate() {
            if i >= range_start && i <= range_end {
                let amt: f64 = val.parse().unwrap_or(DEFAULT_FLOAT);
                total_amt += amt;
            }
        }

        let cf_data = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.llg_id
            )],
            create_account_without_cashflows(llg_id, total_amt, currency)
        );
        writer.write(cf_data);
    }

    writer.close();

    let end_generator_timer = SystemTime::now();
    let total_duration = end_generator_timer
        .duration_since(start_generator_timer)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Total Duration: {:?}",
        total_accounts_encountered, total_duration
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
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
