use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod cf_date_iterator;
mod duration_extensions;
mod gen_cashflows;
mod io;

use self::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_generator::cashflow_appender::create_account_with_cashflows;
use cashflow_generator::gen_cashflows::generate_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;

    let mut total_principal_in_input = 0.0;
    let mut tot_prin_in_op = 0.0;

    let start_time = SystemTime::now();
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
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        let mut input_account = account_opt.unwrap();

        //Updating Int-Proj for required Accs using a config-param where AsOn is Month-End-Date
        input_account.int_proj = if config_params
            .int_proj_accs_prefix()
            .contains(&input_account.acc_type[0..2].to_string())
        //&& rbdate::get_month_end_date(*config_params.as_on_date())
        //== *config_params.as_on_date()
        {
            input_account.curr_bal + input_account.term_int_comp_amt
        } else {
            input_account.curr_bal
        };

        //Assigning Curr-Bal to Curr-Bal-LCY before updating
        input_account.curr_bal_lcy = input_account.curr_bal;

        //Updating Curr-Bal for required Accs using a config-param
        if config_params
            .curr_bal_accs_prefix()
            .contains(&input_account.acc_type[0..2].to_string())
        {
            input_account.curr_bal += input_account.int_available;
            log_info!(
                log,
                "Curr-Bal updated for Account: `{:?}`",
                input_account.key_1
            );
        }
        // Get rid of this total_accounts_encountered
        total_accounts_encountered += 1;
        total_principal_in_input += input_account.curr_bal;

        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: {}",
                input_account.customer_no
            )],
            generate_cashflows(
                config_params.as_on_date(),
                &mut input_account,
                log,
                config_params.day_convention()
            )
        );

        // If this account didn't generate cashflows due to an error
        // log the error and move on to the next account.
        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: '{}'. Error: {}",
                input_account.customer_no,
                cashflows_for_account_result.err().unwrap();
            );
            continue;
        }

        let cashflows = cashflows_for_account_result.unwrap();

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.customer_no
            )],
            create_account_with_cashflows(input_account, cashflows, log)
        );
        tot_prin_in_op += account_with_cashflows.tot_prin_amt;
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.customer_no
            )],
            writer.write(account_with_cashflows)
        );
    }

    writer.close();

    let end_time = SystemTime::now();
    let total_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    debug!(log, "Total Cashflows: {:?}", total_duration);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        total_principal_in_input,
        tot_prin_in_op,
        total_cfs as i64,
    );
    println!("{}", health_stat.display());
    info!(log, "{}", health_stat.display());
    health_stat.gen_health_rpt(config_params.output_file_path());
}

// MARK: Helper functions

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
