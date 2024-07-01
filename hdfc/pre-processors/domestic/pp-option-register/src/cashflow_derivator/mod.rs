use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use slog::Logger;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod der_cashflows;

use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::time::SystemTime;

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_prin_in_inp: f64 = DEFAULT_FLOAT;
    let mut tot_prin_in_op: f64 = DEFAULT_FLOAT;
    let mut tot_cfs: usize = 0;

    let mut skpd_acc = DEFAULT_INT;

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
                tot_acc_encntrd
            )],
            reader_iterator.next()
        );
        tot_acc_encntrd += 1;
        if account_opt.is_none() {
            skpd_acc += 1;
            break;
        }
        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        if let Some(dt) = input_account.cf_dt {
            if dt <= *config_params.as_on_date() {
                skpd_acc += 1;
                log_debug!(
                    log,
                    "`cashflow_date`({}) is less than or equal to `as_on_date`({}) for account: `{}`.",
                    dt,
                    config_params.as_on_date(),
                    input_account.trade_id,
                );
                continue;
            }
        } else {
            log_error!(
                log,
                "`cashflow date`: `({:#?})` is not well-formatted for account: `{}`.",
                input_account.del_dt,
                input_account.trade_id,
            );
            skpd_acc += 1;
            continue;
        }

        if config_params.entity() != input_account.entity {
            skpd_acc += 1;
            log_debug!(
                log,
                "Entity: `{}` not in `{}` for trade_id: `{}`.",
                input_account.entity,
                config_params.entity(),
                input_account.trade_id,
            );
            continue;
        }

        let prin_amt: f64 = input_account.flowamount;
        tot_prin_in_inp += prin_amt;
        let cf_type = if prin_amt < 0.0 { "L" } else { "A" };
        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.trade_id
            )],
            derive_cashflows(&mut input_account, *config_params.as_on_date(), log)
        );

        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();

        let a_w_cf = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.trade_id
            )],
            create_account_with_cashflows(input_account.clone(), cashflows, cf_type)
        );

        tot_prin_in_op += a_w_cf.tot_prin_amt;

        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                a_w_cf.trade_id
            )],
            writer.write(a_w_cf)
        );
    }
    writer.close();

    let end_time = SystemTime::now();
    let tot_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts Processed: {}\n\
         Accounts Skipped: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total principal amount in input: {:.2}\n\
         Total principal amount in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_encntrd - skpd_acc,
        skpd_acc,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_inp,
        tot_prin_in_op,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skpd_acc,
        skpd_acc,
        tot_prin_in_inp,
        tot_prin_in_op,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
