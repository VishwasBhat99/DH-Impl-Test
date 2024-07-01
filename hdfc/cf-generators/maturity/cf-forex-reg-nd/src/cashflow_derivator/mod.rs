use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use slog::Logger;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod der_cashflows;

use calamine::{open_workbook_auto, Reader};
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use statics::*;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_pay_leg_amt_in_inp = DEFAULT_FLOAT;
    let mut tot_pay_leg_amt_in_op = DEFAULT_FLOAT;
    let mut tot_rec_leg_amt_in_inp = DEFAULT_FLOAT;
    let mut tot_rec_leg_amt_in_op = DEFAULT_FLOAT;
    let mut skpd_acc = DEFAULT_INT;

    let start_time = SystemTime::now();

    let op_file_path_pay = get_full_file_path(config_params.output_file_path(), "pay");
    let op_file_path_rec = get_full_file_path(config_params.output_file_path(), "rec");
    let (reader, mut writer_pay, mut writer_rec) = create_io_workers(
        config_params.input_file_path(),
        &op_file_path_pay,
        &op_file_path_rec,
        log,
    );

    let mut reader_iterator = reader.into_iter();

    let mut ref_map: HashMap<String, _> = HashMap::new();
    let mut ref_excel = open_workbook_auto(config_params.ref_file_path())
        .expect("Unable to open Reference Master File.");
    if let Some(Ok(reader)) = ref_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(1) {
            ref_map.insert(row[5].to_string(), "FOREX_REG".to_string());
        }
    }

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

        tot_acc_with_cfs += 1;
        if let Some(dt) = input_account.mat_date_of_trade {
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
                input_account.mat_date_of_trade,
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
        if !ref_map.contains_key(&input_account.contract_typology) {
            skpd_acc += 1;
            log_error!(
                log,
                "Trade Id: `{}` exluded due to the Invalid Contract.",
                input_account.trade_id,
            );
            continue;
        }
        let amt: f64 = input_account.mtm_in_usd_forward;
        let mut ccy = input_account.ndf_settlementcurrency.to_string();
        let mut cf_type = "I";

        // for pay side {Having amt > 0 considered in pay side}
        if amt >= DEFAULT_FLOAT {
            tot_pay_leg_amt_in_inp += amt;
            let cashflows = log_measurements!(
                diag_log,
                [format!(
                    "Type: DeriveCFs, Identifier: {}",
                    input_account.trade_id
                )],
                derive_cashflows(&mut input_account, *config_params.as_on_date(), amt, log)
            );

            tot_cfs += cashflows.len();

            let a_w_cf = log_measurements!(
                diag_log,
                [format!(
                    "Type: CreateAccWithCFs, Identifier: {}",
                    input_account.trade_id
                )],
                create_account_with_cashflows(input_account.clone(), cashflows, ccy, cf_type)
            );

            tot_pay_leg_amt_in_op += a_w_cf.tot_prin_amt;
            log_measurements!(
                diag_log,
                [format!(
                    "Type: WriteAccWithCFs, Identifier: {}",
                    a_w_cf.trade_id
                )],
                writer_pay.write(a_w_cf)
            );
        }
        // for receive side {Having amt < 0 considered in receive side}
        else {
            tot_rec_leg_amt_in_inp += amt.abs();
            cf_type = "O";
            let cashflows = log_measurements!(
                diag_log,
                [format!(
                    "Type: DeriveCFs, Identifier: {}",
                    input_account.trade_id
                )],
                derive_cashflows(&mut input_account, *config_params.as_on_date(), amt, log)
            );

            tot_cfs += cashflows.len();
            ccy = input_account.sell_currency.to_string();
            let a_w_cf = log_measurements!(
                diag_log,
                [format!(
                    "Type: CreateAccWithCFs, Identifier: {}",
                    input_account.trade_id
                )],
                create_account_with_cashflows(input_account, cashflows, ccy, cf_type)
            );

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
        tot_acc_encntrd += 1;
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
         Total Rec Leg amount in input: -{:.2} \n\
         Total Rec Leg amount in output: -{:.2}",
        tot_acc_encntrd,
        skpd_acc,
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

    let health_stat = HealthReport::new(
        tot_acc_with_cfs,
        tot_acc_with_cfs - skpd_acc,
        skpd_acc,
        tot_pay_leg_amt_in_inp + tot_rec_leg_amt_in_inp,
        tot_pay_leg_amt_in_op + tot_rec_leg_amt_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
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
