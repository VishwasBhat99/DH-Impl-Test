use self::account::Account;
use self::account_appender::create_account_without_cashflows;
use self::account_writer::AccountWithoutCashflows;
use self::derive_fields::get_output_line;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::io::{prelude::*, BufReader};
use std::time::SystemTime;

mod account_appender;
mod derive_fields;
mod account;
mod account_writer;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut writer = AccountWithoutCashflows::new(config_param.output_file_path(), log);

    let start_process_timer = SystemTime::now();
    let mut ttl_bal = DEFAULT_FLOAT;
    let mut output_acc_info: String = String::new();
    let mut ttl_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_acc: i64 = DEFAULT_INT;
    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Error while creating file: `{}`. : {:?}",
            config_param.output_file_path(),
            error
        ),
    };

    let mut output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };

    for (line_num, line) in BufReader::new(input_file).lines().enumerate() {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!(
                    "Error while reading the file: `{}` at line number: `{}`. : {:?}",
                    config_param.input_file(),
                    line_num + 1,
                    error
                );
            }
        };
        let fields: Vec<&str> = record.split("|").collect();
        ttl_acc_encntrd += 1;
        if fields.len() != 3 {
            skp_acc += 1;
            continue;
        }

        let mut amt = fields[1].parse().unwrap_or(DEFAULT_FLOAT);
        ttl_bal += amt;
        let mut accounts: Vec<Account> = Vec::new();
        accounts.push(create_account_without_cashflows(
            &fields,
            amt,
            config_param.currency(),
        ));
        output_acc_info.push_str(get_output_line(&fields, amt, config_param.currency()).as_str());

        amt *= -1.0;
        accounts.push(create_account_without_cashflows(
            &fields,
            amt,
            config_param.currency(),
        ));
        output_acc_info.push_str(get_output_line(&fields, amt, config_param.currency()).as_str());
        write!(output_file, "{}", output_acc_info).expect("Error while writing output line.");

        for acc in accounts.iter() {
            writer.write(acc.clone());
        }
        output_acc_info.clear();
    }
    writer.close();

    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for deriving fields and writing output.");
    debug!(
        diag_log,
        "Total Duration for deriving fields and writing output: {:?}.", duration
    );

    println!("Total Balance: {:.2}", ttl_bal);

    let health_report = HealthReport::new(
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        ttl_bal,
        ttl_bal,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
