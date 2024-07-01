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
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;
use std::time::SystemTime;

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_rec: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;

    let mut tot_pay_leg_amt_in_inp = DEFAULT_FLOAT;
    let mut tot_pay_leg_amt_in_op = DEFAULT_FLOAT;
    let mut tot_rec_leg_amt_in_inp = DEFAULT_FLOAT;
    let mut tot_rec_leg_amt_in_op = DEFAULT_FLOAT;

    let start_time = SystemTime::now();

    let op_fl_path_pay = get_full_file_path(config_params.output_file_path(), "pay");
    let op_fl_path_rec = get_full_file_path(config_params.output_file_path(), "rec");
    let (reader, mut writer_pay, mut writer_rec) = create_io_workers(
        config_params.input_file_path(),
        &op_fl_path_pay,
        &op_fl_path_rec,
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

        if account_opt.is_none() {
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_acc_encntrd += 1;

        if let Some(dt) = input_account.cf_dt {
            if dt <= *config_params.as_on_date() {
                skp_rec += 1;
                log_debug!(
                    log,
                    "`cashflow date`: `{:?}` is less than or equal to `as on date`: `{:?}` for trade id: `{}`.",
                    input_account.cf_dt,
                    config_params.as_on_date(),
                    input_account.trade_id,
                );
                continue;
            }
        } else {
            log_error!(
                log,
                "`cashflow date`: `{:?}` is not well-formatted for trade id: `{}`.",
                input_account.cf_dt,
                input_account.trade_id
            );
            continue;
        }

        if config_params.entity() != input_account.entity {
            continue;
        }

        let mut pay_leg_amt: f64 = DEFAULT_FLOAT;
        let mut rec_leg_amt: f64 = DEFAULT_FLOAT;
        let cf_amt = input_account.flowamount.clone();

        let int_rt: f64 = if cf_amt < 0.0 {
            pay_leg_amt = cf_amt.abs();
            input_account.pay_int_rt
        } else {
            rec_leg_amt = cf_amt;
            input_account.rec_int_rt
        };

        tot_pay_leg_amt_in_inp += pay_leg_amt;
        tot_rec_leg_amt_in_inp += rec_leg_amt;

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.trade_id
            )],
            derive_cashflows(&mut input_account, log)
        );

        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();

        let a_w_cf = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: {}",
                input_account.trade_id
            )],
            create_account_with_cashflows(input_account, cashflows, int_rt)
        );

        if cf_amt < 0.0 {
            tot_pay_leg_amt_in_op += a_w_cf.tot_prin_amt;
            log_measurements!(
                diag_log,
                [format!(
                    "Type: WriteAccWithCFs, Identifier: {}",
                    a_w_cf.trade_id
                )],
                writer_pay.write(a_w_cf)
            );
        } else {
            tot_rec_leg_amt_in_op += a_w_cf.tot_prin_amt;
            log_measurements!(
                diag_log,
                [format!(
                    "Type: WriteAccWithCFs, Identifier: {}",
                    a_w_cf.trade_id
                )],
                writer_rec.write(a_w_cf)
            );
        }
    }
    writer_pay.close();
    writer_rec.close();

    let end_time = SystemTime::now();
    let tot_duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Skipped Accounts: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total Pay Leg amount in input: {:.2} \n\
         Total Pay Leg amount in output: {:.2} \n\
         Total Rec Leg amount in input: {:.2} \n\
         Total Rec Leg amount in output: {:.2}",
        tot_acc_encntrd,
        skp_rec,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_pay_leg_amt_in_inp,
        tot_pay_leg_amt_in_op,
        tot_rec_leg_amt_in_inp,
        tot_rec_leg_amt_in_op,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_rec,
        skp_rec,
        tot_pay_leg_amt_in_inp + tot_rec_leg_amt_in_inp,
        tot_pay_leg_amt_in_op + tot_rec_leg_amt_in_op,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn create_io_workers(
    input_path: &str,
    output_path_pay: &str,
    output_path_rec: &str,
    log: &Logger,
) -> (
    InputAccountReader,
    AccountWithCashflowsWriter,
    AccountWithCashflowsWriter,
) {
    let reader = InputAccountReader::new(input_path, log);
    let writer_pay = AccountWithCashflowsWriter::new(output_path_pay, log);
    let writer_rec = AccountWithCashflowsWriter::new(output_path_rec, log);

    (reader, writer_pay, writer_rec)
}

fn get_full_file_path(file_path: &str, suffix: &str) -> String {
    let mut full_file_path: String = String::new();
    full_file_path.push_str(file_path);
    full_file_path.push('-');
    full_file_path.push_str(suffix);
    full_file_path
}
