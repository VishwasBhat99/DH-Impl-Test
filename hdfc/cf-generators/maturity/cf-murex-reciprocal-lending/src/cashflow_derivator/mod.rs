mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod appender;
mod cashflow_appender;
mod derive_cashflows;

use self::appender::get_op_line;
use cashflow_derivator::account_reader::InputAccountReader;
use cashflow_derivator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use cashflow_derivator::cashflow_appender::create_account_with_cashflows;
use cashflow_derivator::derive_cashflows::derive_cashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::timestamp;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use statics::*;
use std::env::current_dir;
use std::io::prelude::*;

pub fn derive(config_param: ConfigurationParameters, log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;
    let mut total_accounts_with_cashflows: i64 = DEFAULT_INT;
    let mut total_cfs: usize = 0;
    let mut tot_prin_in_ip = DEFAULT_FLOAT;
    let mut tot_prin_in_op = DEFAULT_FLOAT;

    let (reader, mut writer) = create_io_workers(
        config_param.input_file_path(),
        config_param.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    let mut op_line = String::new();
    loop {
        let account_opt = reader_iterator.next();
        if account_opt.is_none() {
            break;
        }

        let input_account = account_opt.expect("Unable to parse `record`.");
        total_accounts_encountered += 1;

        let mut cf_type: String;
        let cf_amt = input_account.sanc_amt - input_account.util_amt;
        let mut cf_dt: i64;

        if input_account.cf_sub_type.to_uppercase() == "SANCTION"
            && input_account.typ.to_uppercase() == "LEND"
        {
            // For Outflow
            tot_prin_in_ip += cf_amt;
            cf_type = "O".to_string();
            cf_dt = timestamp(*config_param.as_on_date());
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account.clone(), cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);

            // For Inflow
            tot_prin_in_ip += cf_amt;
            cf_type = "I".to_string();
            cf_dt = if let Some(dt) = input_account.ed_dt {
                timestamp(dt)
            } else {
                log_error!(
                    log,
                    "`end_date` not well-formatted for cf_sub_type: `{}`.",
                    input_account.cf_sub_type
                );
                timestamp(*config_param.as_on_date())
            };
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account, cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);
        } else if input_account.cf_sub_type.to_uppercase() == "UTIL"
            && input_account.typ.to_uppercase() == "LEND"
        {
            // For Outflow
            tot_prin_in_ip += cf_amt;
            cf_type = "O".to_string();
            cf_dt = if let Some(dt) = input_account.st_dt {
                timestamp(dt)
            } else {
                log_error!(
                    log,
                    "`start_date` not well-formatted for cf_sub_type: `{}`.",
                    input_account.cf_sub_type
                );
                timestamp(*config_param.as_on_date())
            };
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account.clone(), cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);

            // For Inflow
            tot_prin_in_ip += cf_amt;
            cf_type = "I".to_string();
            cf_dt = if let Some(dt) = input_account.ed_dt {
                timestamp(dt)
            } else {
                log_error!(
                    log,
                    "`end_date` not well-formatted for cf_sub_type: `{}`.",
                    input_account.cf_sub_type
                );
                timestamp(*config_param.as_on_date())
            };
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account, cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);
        } else if input_account.cf_sub_type.to_uppercase() == "SANCTION"
            && input_account.typ.to_uppercase() == "BORROW"
        {
            // For Inflow
            tot_prin_in_ip += cf_amt;
            cf_type = "I".to_string();
            cf_dt = timestamp(*config_param.as_on_date());
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account.clone(), cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);

            // For Outflow
            tot_prin_in_ip += cf_amt;
            cf_type = "O".to_string();
            cf_dt = if let Some(dt) = input_account.ed_dt {
                timestamp(dt)
            } else {
                log_error!(
                    log,
                    "`end_date` not well-formatted for cf_sub_type: `{}`.",
                    input_account.cf_sub_type
                );
                timestamp(*config_param.as_on_date())
            };
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account, cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);
        } else if input_account.cf_sub_type.to_uppercase() == "UTIL"
            && input_account.typ.to_uppercase() == "BORROW"
        {
            // For Inflow
            tot_prin_in_ip += cf_amt;
            cf_type = "I".to_string();
            cf_dt = if let Some(dt) = input_account.st_dt {
                timestamp(dt)
            } else {
                log_error!(
                    log,
                    "`start_date` not well-formatted for cf_sub_type: `{}`.",
                    input_account.cf_sub_type
                );
                timestamp(*config_param.as_on_date())
            };
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account.clone(), cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);

            // For Outflow
            tot_prin_in_ip += cf_amt;
            cf_type = "O".to_string();
            cf_dt = if let Some(dt) = input_account.ed_dt {
                timestamp(dt)
            } else {
                log_error!(
                    log,
                    "`end_date` not well-formatted for cf_sub_type: `{}`.",
                    input_account.cf_sub_type
                );
                timestamp(*config_param.as_on_date())
            };
            let cashflows = derive_cashflows(&input_account, cf_amt, cf_dt, log);
            total_accounts_with_cashflows += 1;
            total_cfs += cashflows.len();

            op_line.push_str(&get_op_line(
                input_account.clone(),
                cf_type.to_string(),
                cf_dt,
                cf_amt,
            ));
            let a_w_cf = create_account_with_cashflows(input_account, cashflows, cf_type);
            tot_prin_in_op += cf_amt;
            writer.write(a_w_cf);
        } else {
            log_error!(
                log,
                "`type` : `{}` is not well-formated for cf_sub_type: `{}`.",
                input_account.typ,
                input_account.cf_sub_type
            )
        }
    }

    writer.close();

    let mut op_file = String::new();
    op_file.push_str(config_param.output_file_path());
    op_file.push_str(".txt");
    let mut writer = match buf_file_wrtr(&op_file, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            op_file,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            op_file, error,
        ),
    }

    let report_string = format!(
        "Accounts Encountered: {}\n\
         Accounts With Cashflows: {}\n\
         Total Cashflows: {}\n\
         Total outstanding amount in input: {:.2} \n\
         Total outstanding amount in output: {:.2}",
        total_accounts_encountered,
        total_accounts_with_cashflows,
        total_cfs,
        tot_prin_in_ip,
        tot_prin_in_op,
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
    health_stat.gen_health_rpt(config_param.output_file_path())
}

pub fn create_io_workers(
    inp_file: &str,
    op_file: &str,
    log: &Logger,
) -> (InputAccountReader, AccountWithCashflowsWriter) {
    let reader = InputAccountReader::new(inp_file, log);
    let writer = AccountWithCashflowsWriter::new(op_file, log);

    (reader, writer)
}
