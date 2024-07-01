use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod der_cashflows;

use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use macros;
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use std::time::SystemTime;

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;

    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let tot_int_in_ip = DEFAULT_FLOAT;
    let mut skpd_acc = DEFAULT_INT;

    let start_time = SystemTime::now();

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );

    let mut reader_iterator = reader.into_iter();
    log_debug!(log, "Skipping Header.");
    reader_iterator.next();

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
        tot_acc_encntrd += 1;

        if input_account.frwrd_delta_ccy == "" {
            skpd_acc += 1;
            log_error!(
                log,
                "Record skipped as `forward_delta_ccy` not found for account: `{}`.",
                input_account.trade_id
            );
            continue;
        }

        if let Some(dt) = input_account.delivery_dt {
            if dt < *config_params.as_on_date() {
                skpd_acc += 1;
                log_error!(
                    log,
                    "Record skipped as `delivery_date`({}) is less than `as_on_date`({}) for account: `{}`.",
                    dt,
                    config_params.as_on_date(),
                    input_account.trade_id,
                );
                continue;
            }
        }

        if let Some(amt) = input_account.frwrd_delta {
            tot_prin_in_ip += amt;
        }

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
            create_account_with_cashflows(input_account, cashflows)
        );

        tot_prin_in_op += a_w_cf.tot_prin_amt;
        tot_int_in_op += a_w_cf.tot_int_amt;

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
         Accounts Skipped {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_encntrd - skpd_acc,
        skpd_acc,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
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
