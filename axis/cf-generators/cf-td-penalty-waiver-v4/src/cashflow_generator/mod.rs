use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod gen_cashflows;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use std::time::SystemTime;

pub fn generate(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut tot_amt_ip: f64 = 0.0;
    let mut tot_amt_op = 0.0;
    let mut acc_read_fail = 0;
    let mut total_accounts_with_cashflows = 0;
    let start_time = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    if config_params.skip_header() {
        reader_iterator.next();
    }
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
        total_accounts_encountered += 1;

        let mut input_account = match account_opt {
            Some(val) => val,
            None => {
                log_info!(log, "Unable to parse `record`.");
                acc_read_fail += 1;
                continue;
            }
        };
        tot_amt_ip += input_account.out_bal_amt;
        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: `{}`",
                input_account.acid
            )],
            generate_cashflows(&mut input_account, &config_params, &log)
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
                *config_params.as_on_date(),
                cashflows,
                config_params.source_name().to_string()
            )
        );

        tot_amt_op += account_with_cashflows.clr_bal_amt;
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
         Total outstanding amount in output: {:.2}\n",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_duration,
        tot_amt_ip,
        tot_amt_op
    );
    log_info!(log, "{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_cfs as i64,
        acc_read_fail,
        tot_amt_ip,
        tot_amt_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path());
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
