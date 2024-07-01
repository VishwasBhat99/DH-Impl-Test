use self::account_with_cashflows_writer::AccountWithCashflowsWriter;
use self::cashflow_appender::create_account_with_cashflows;
use crate::cashflow_derivator::account_reader::input_account::InputAccount;
use crate::cashflow_derivator::account_reader::InputAccountReader;
use crate::cashflow_derivator::der_cashflows::{derive_adj_cashflows, derive_cashflows};
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::statics::*;
use cashflow_derivator::account_with_cashflows::Cashflow;
use slog::Logger;
use std::collections::HashMap;
use std::time::SystemTime;

mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;
mod der_cashflows;

pub fn derive(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut tot_acc_encntrd: i64 = DEFAULT_INT;
    let mut tot_acc_with_cfs: i64 = DEFAULT_INT;
    let mut tot_cfs: usize = 0;
    let mut tot_amt = DEFAULT_FLOAT;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_op = DEFAULT_FLOAT;
    let mut tot_int_in_ip = DEFAULT_FLOAT;

    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader.into_iter();
    let mut adj_map: HashMap<String, f64> = HashMap::new();
    let inp_acc_map: &mut HashMap<String, InputAccount> = &mut HashMap::new();
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: `{}`",
                tot_acc_encntrd,
            )],
            reader_iterator.next()
        );
        tot_acc_encntrd += 1;
        if account_opt.is_none() {
            break;
        }

        let mut input_account = account_opt.expect("Unable to parse InputAccount struct.");
        tot_amt += input_account.prin_ost_bal;
        if let Some(amt) = input_account.cf_amt {
            if input_account.component == "INTEREST" {
                tot_int_in_ip += amt;
            } else if input_account.component == "PRINCIPAL" {
                tot_prin_in_ip += amt;
                adj_map
                    .entry(input_account.cust_no.to_string())
                    .and_modify(|amt| *amt -= input_account.cf_amt.unwrap_or(DEFAULT_FLOAT))
                    .or_insert(
                        input_account.prin_ost_bal - input_account.cf_amt.unwrap_or(DEFAULT_FLOAT),
                    );
            } else {
                error!(
                    log,
                    "Invalid component: {} for Account: {}",
                    input_account.component,
                    input_account.cust_no
                )
            }
        }

        inp_acc_map
            .entry(input_account.cust_no.to_string())
            .or_insert(input_account.to_owned());

        let cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: DeriveCFs, Identifier: {}",
                input_account.cust_no
            )],
            derive_cashflows(&mut input_account, &config_params, log)
        );

        if cashflows.is_err() {
            log_error!(
                log,
                "Cashflows not generated for account: `{}`. Error: {}",
                input_account.cust_no,
                cashflows.err().expect("Unable to unwrap error.");
            );
            continue;
        }

        let cashflows = cashflows.expect("Unable to generate cashflows.");

        tot_acc_with_cfs += 1;
        tot_cfs += cashflows.len();

        let account_with_cashflows = log_measurements!(
            diag_log,
            [format!(
                "Type: CreateAccWithCFs, Identifier: `{}`",
                input_account.cust_no
            )],
            create_account_with_cashflows(input_account, cashflows)
        );

        if account_with_cashflows.component == "PRINCIPAL" {
            tot_prin_in_op += account_with_cashflows.cf_amt;
        } else if account_with_cashflows.component == "INTEREST" {
            tot_int_in_op += account_with_cashflows.cf_amt;
        }
        log_measurements!(
            diag_log,
            [format!(
                "Type: WriteAccWithCFs, Identifier: {}",
                account_with_cashflows.cust_no
            )],
            writer.write(account_with_cashflows)
        );
    }

    if config_params.adj_cf_flag() {
        for (custno, amount) in adj_map.iter() {
            let mut account = inp_acc_map
                .get(&custno.to_string())
                .expect("Invalid Account")
                .to_owned();
            let cashflows = log_measurements!(
                diag_log,
                [format!("Type: DeriveCFs, Identifier: {}", custno)],
                derive_adj_cashflows(&mut account, amount, &config_params, log)
            );

            tot_cfs += 1;
            let mut cfs: Vec<Cashflow> = Vec::new();
            cfs.push(cashflows);

            let account_with_cashflows = log_measurements!(
                diag_log,
                [format!("Type: CreateAccWithCFs, Identifier: `{}`", custno)],
                create_account_with_cashflows(account, cfs)
            );
            tot_prin_in_op += account_with_cashflows.cf_amt;

            log_measurements!(
                diag_log,
                [format!(
                    "Type: WriteAccWithCFs, Identifier: {}",
                    account_with_cashflows.cust_no
                )],
                writer.write(account_with_cashflows)
            );
        }
    }
    writer.close();

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total Outstanding Balance: {}\n\
         Total principal amount in input: {:.2} \n\
         Total principal amount in output: {:.2}\n\
         Total interest in input: {:.2}\n\
         Total interest in output: {:.2}",
        tot_acc_encntrd,
        tot_acc_with_cfs,
        (tot_cfs + tot_acc_with_cfs as usize),
        tot_amt,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_int_in_ip,
        tot_int_in_op
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);

    let health_stat = health_report::HealthReport::new(
        tot_acc_with_cfs,
        tot_acc_with_cfs,
        0,
        tot_prin_in_ip,
        tot_prin_in_op,
        tot_cfs as i64,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn create_io_workers(
    input_path: &str,
    output_path: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(input_path, log);
    let writer = AccountWithCashflowsWriter::new(output_path, log);

    (reader, writer)
}
