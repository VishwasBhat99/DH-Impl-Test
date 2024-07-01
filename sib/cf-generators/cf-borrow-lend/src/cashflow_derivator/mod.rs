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
use cashflow_derivator::account_with_cashflows::Cashflow;
use cashflow_derivator::der_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use protobuf::Clear;
use std::time::SystemTime;

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = 0;
    let mut prev_acc_num: String = "".to_string();
    let mut tot_acc_succ: i64 = 0;
    let mut tot_amt = 0.0;
    let mut tot_cfs: usize = 0;

    let mut cfs: Vec<Cashflow> = Vec::new();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut a_w_cf = AccountWithCashflows::new();
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
            tot_cfs += cfs.len();
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            tot_acc_succ += 1;
            a_w_cf.clear();
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_amt += input_account.outstanding_amount;
        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.deal_number
            )],
            derive_cashflows(&mut input_account, config_params, log)
        );

        if prev_acc_num.is_empty() {
            tot_acc_encntrd += 1;
            prev_acc_num = input_account.deal_number.to_string();
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
        } else if prev_acc_num != input_account.deal_number {
            tot_acc_encntrd += 1;
            tot_cfs += cfs.len();
            write_cashflows(&mut writer, log, diag_log, &a_w_cf);
            tot_acc_succ += 1;
            a_w_cf.clear();
            cfs.clear();
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
            prev_acc_num = input_account.deal_number.to_string();
        } else {
            cfs.push(cashflows);
            a_w_cf = append_cashflows(diag_log, &input_account, &cfs);
        }
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
