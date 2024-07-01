use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod der_cashflows;

use self::append_write_cashflows::create_io_workers;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_acc_succ: i64 = 0;
    let mut tot_amt = 0.0;
    let mut tot_cfs: usize = 0;
    let cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
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
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_amt += input_account.outstanding_balance_inr;
        tot_acc_encntrd += 1;
        let cashflows_for_account_result = log_measurements!(
            diag_log,
            [format!(
                "Type: GenCFs, Identifier: {}",
                input_account.account_number
            )],
            derive_cashflows(
                &mut input_account,
                config_params,
                log,
                *config_params.day_convention()
            )
        );
        if cashflows_for_account_result.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.account_number,
                cashflows_for_account_result.err().expect("Unable to unwrap error.");
            );
            continue;
        }
        let cashflows = cashflows_for_account_result.expect("Unable to generate cashflows.");

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: `{}`",
                input_account.account_number
            )],
            create_account_with_cashflows(input_account, cashflows)
        );

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.account_number
            )],
            writer.write(account_with_cashflows)
        );
    }
    writer.close();

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_succ,
        tot_acc_encntrd - tot_acc_succ,
        tot_amt,
        tot_amt,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}
