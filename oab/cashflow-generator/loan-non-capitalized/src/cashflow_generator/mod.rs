use slog::Logger;
mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod der_cashflows;

use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::append_cashflows;
use self::append_write_cashflows::create_io_workers;
use self::append_write_cashflows::write_cashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use cashflow_generator::der_cashflows::{derive_cashflows, generate_adjustment_cashflow};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use rbdate::NaiveDate;
use statics::*;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut prev_acc_num: String = "".to_string();
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut sum_prin_acc = DEFAULT_FLOAT;
    let mut tot_os_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;
    let start_derive_timer = SystemTime::now();
    let mut cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithCashflows::new();
    let mut nxt_rep_dt: Option<NaiveDate> = None;
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
            if sum_prin_acc != a_w_cf.outstanding_bal {
                let diff = a_w_cf.outstanding_bal - sum_prin_acc;
                a_w_cf.total_principal_amount += diff;
                generate_adjustment_cashflow(
                    &a_w_cf,
                    &mut sum_prin_acc,
                    &mut cfs,
                    config_params.precision(),
                    log,
                );
            }
            a_w_cf.cashflows = protobuf::RepeatedField::from_vec(cfs.clone());
            tot_cfs += cfs.len();
            tot_int_in_op += a_w_cf.total_interest_amount;
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.account_id
            )],
            derive_cashflows(&mut input_account, &config_params, log)
        );
        tot_int_in_ip += cashflows.interest_amount;
        tot_prin_in_ip += cashflows.principal_amount;
        if prev_acc_num == "" {
            tot_acc_encntrd += 1;
            tot_os_in_ip += input_account.outstanding_bal.unwrap_or(0.0).abs();
            tot_prin_in_op += input_account.outstanding_bal.unwrap_or(0.0).abs();
            sum_prin_acc += cashflows.principal_amount;
            tot_acc_with_cfs += 1;
            prev_acc_num = input_account.account_id.to_string();
            nxt_rep_dt = input_account.next_repr_date;
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, nxt_rep_dt, &cfs);
        } else if prev_acc_num != input_account.account_id {
            tot_acc_encntrd += 1;
            tot_os_in_ip += input_account.outstanding_bal.unwrap_or(0.0);
            tot_prin_in_op += input_account.outstanding_bal.unwrap_or(0.0).abs();
            tot_acc_with_cfs += 1;
            if sum_prin_acc != a_w_cf.outstanding_bal {
                a_w_cf.total_principal_amount += a_w_cf.outstanding_bal - sum_prin_acc;
                generate_adjustment_cashflow(
                    &a_w_cf,
                    &mut sum_prin_acc,
                    &mut cfs,
                    config_params.precision(),
                    log,
                );
            }
            a_w_cf.cashflows = protobuf::RepeatedField::from_vec(cfs.clone());
            tot_cfs += cfs.len();
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            cfs.clear();
            sum_prin_acc = DEFAULT_FLOAT;
            sum_prin_acc += cashflows.principal_amount;
            cfs.push(cashflows);
            nxt_rep_dt = input_account.next_repr_date;
            a_w_cf = append_cashflows(diag_log, &input_account, nxt_rep_dt, &cfs);
            tot_int_in_op += a_w_cf.total_interest_amount;
            prev_acc_num = input_account.account_id.to_string();
        } else {
            if nxt_rep_dt < input_account.next_repr_date {
                nxt_rep_dt = input_account.next_repr_date;
            }
            sum_prin_acc += cashflows.principal_amount;
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, nxt_rep_dt, &cfs);
        }
    }

    writer.close();

    let end_derive_timer = SystemTime::now();
    let tot_duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total duration for derive timer.");
    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Duration: {:?}\n\
         Total Outstanding in input: {:?}\n\
         Total cashflow amount in input: {:.2} \n\
         Total cashflow amount in output: {:.2}\n\
         Total cashflow adjusted amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        tot_cfs,
        tot_duration,
        tot_os_in_ip,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_os_in_ip - tot_prin_in_ip,
        tot_int_in_ip,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path());
}
