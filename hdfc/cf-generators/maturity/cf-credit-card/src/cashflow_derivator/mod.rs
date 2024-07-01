use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod derive_cashflows;
mod reconciliation;

use self::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::derive_cashflows::derive_cashflows;
use cashflow_derivator::reconciliation::ReconKey;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
use health_report::HealthReport;

pub fn derive(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut total_principal_in_input = 0.0;
    let mut total_principal_in_output = 0.0;
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();

    let (reader, mut writer) = create_io_workers(
        config_param.input_file_path(),
        config_param.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;

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
        let input_account = account_opt.expect("Failed to parse 'record'.");

        total_accounts_encountered += 1;
        total_principal_in_input += input_account.total_pd_sum;

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: {}",
                input_account.gl_number
            )],
            derive_cashflows(*config_param.as_on_date(), &input_account)
        );

        total_accounts_with_cashflows += 1;
        total_cfs += cashflows.len();
        {
            let gl_num = &input_account.gl_number.trim();
            let amt: f64 = input_account.total_pd_sum;
            let recon_key = ReconKey::new(
                "INR".to_string(),
                "CREDIT_CARD".to_string(),
                gl_num.to_string(),
            );

            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);
        }

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.gl_number
            )],
            create_account_with_cashflows(input_account, cashflows, *config_param.as_on_date())
        );
        total_principal_in_output += account_with_cashflows.total_pd_sum;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.gl_number
            )],
            writer.write(account_with_cashflows)
        );
    }

    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let mut recon_op_line = String::new();
    for (key, value) in &recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "CC_MAT_EXT1",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push_str("\n");
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconciliation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error
        ),
    };
    writer.close();

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total outstanding amount in input: {:?} \n\
         Total outstanding amount in output: {:?}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        total_principal_in_input,
        total_principal_in_output,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_accounts_encountered - total_accounts_with_cashflows,
        total_principal_in_input,
        total_principal_in_output,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
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
