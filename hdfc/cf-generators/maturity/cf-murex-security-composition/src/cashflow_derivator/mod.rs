mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod append_write_cashflows;
mod cashflow_appender;
mod derive_cashflows;
mod tenor_calculations;

use self::account_with_cashflows::AccountWithCashflows;
use self::append_write_cashflows::append_cashflows;
use self::append_write_cashflows::create_io_workers;
use self::append_write_cashflows::write_cashflows;
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::derive_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use rbdate::*;
use slog::Logger;
use std::time::SystemTime;

pub fn derive(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut prev_acc_num: String = String::new();
    let mut total_accounts_encountered: i64 = 0;
    let mut total_accounts_with_cashflows: i64 = 0;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_ip = 0.0;
    let mut tot_prin_in_op = 0.0;
    let mut tot_int_in_ip = 0.0;
    let mut tot_int_in_op = 0.0;

    let mut cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    reader_iterator.next();
    let mut nxt_rep_dt: Option<NaiveDate>;
    let mut lst_rep_dt: Option<NaiveDate>;
    let mut a_w_cf = AccountWithCashflows::new();
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            total_cfs += cfs.len();
            tot_prin_in_op += a_w_cf.prin_amt;
            tot_int_in_op += a_w_cf.int_amt;
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse `record`.");
        total_accounts_encountered += 1;
        tot_prin_in_ip += input_account.prin_amt;
        tot_int_in_ip += input_account.int_amt;

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.deal_no
            )],
            derive_cashflows(&mut input_account, *config_params.as_on_date(), log)
        );

        if prev_acc_num.is_empty() {
            total_accounts_encountered += 1;
            total_accounts_with_cashflows += 1;
            prev_acc_num = input_account.deal_no.to_string();
            nxt_rep_dt = input_account.nxt_rep_dt;
            lst_rep_dt = input_account.lst_rep_dt;
            cfs.push(cashflows);
            a_w_cf = append_cashflows(
                diag_log,
                &input_account,
                nxt_rep_dt,
                lst_rep_dt,
                *config_params.as_on_date(),
                &cfs,
            );
        } else if prev_acc_num != input_account.deal_no {
            total_accounts_encountered += 1;
            total_accounts_with_cashflows += 1;
            total_cfs += cfs.len();
            tot_prin_in_op += a_w_cf.prin_amt;
            tot_int_in_op += a_w_cf.int_amt;
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            a_w_cf.clear();
            cfs.clear();
            cfs.push(cashflows);
            nxt_rep_dt = input_account.nxt_rep_dt;
            lst_rep_dt = input_account.lst_rep_dt;
            a_w_cf = append_cashflows(
                diag_log,
                &input_account,
                nxt_rep_dt,
                lst_rep_dt,
                *config_params.as_on_date(),
                &cfs,
            );
            prev_acc_num = input_account.deal_no.to_string();
        } else {
            nxt_rep_dt = input_account.nxt_rep_dt;
            lst_rep_dt = input_account.lst_rep_dt;
            cfs.push(cashflows);
            a_w_cf = append_cashflows(
                diag_log,
                &input_account,
                nxt_rep_dt,
                lst_rep_dt,
                *config_params.as_on_date(),
                &cfs,
            );
        }
    }

    writer.close();

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2} \n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = HealthReport::new(
        total_accounts_encountered,
        total_accounts_with_cashflows,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        total_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}
