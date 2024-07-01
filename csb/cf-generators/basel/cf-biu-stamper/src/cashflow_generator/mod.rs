mod account_reader;
mod account_with_cashflows;
mod account_with_cashflows_writer;
mod cashflow_appender;

use self::cashflow_appender::append_data;
use cashflow_generator::account_reader::InputAccountReader;
use cashflow_generator::account_with_cashflows_writer::AccountWithCashflowsWriter;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct BiuData {
    pub t1: String,
    pub t2: String,
    pub t3: String,
    pub t4: String,
}

pub fn generate(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut total_accounts_encountered: i64 = DEFAULT_INT;

    let start_generate_timer = SystemTime::now();
    let (reader, mut writer) = create_io_workers(
        config_params.input_file_path(),
        config_params.output_file_path(),
        log,
    );
    let mut reader_iterator = reader;
    // read biu master file
    let biu_master_file = match new_buf_rdr(config_params.biu_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.biu_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut biu_master: HashMap<String, BiuData> = HashMap::new();
    for (line_num, lines) in biu_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let biu_line = BiuData {
            t1: fields[1].to_string(),
            t2: fields[2].to_string(),
            t3: fields[3].to_string(),
            t4: fields[4].to_string(),
        };
        biu_master.insert(fields[0].to_string(), biu_line);
    }
    // read cust total balance file
    let ttl_bal_file = match new_buf_rdr(config_params.ttl_bal_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.biu_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut ttl_bal_master: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in ttl_bal_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.output_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        //amount field is updated to 1 instead of 0th field
        let amt = fields[1].parse().unwrap_or(DEFAULT_FLOAT);
        ttl_bal_master.insert(fields[0].to_string(), amt);
    }
    let mut tot_rec = 0;
    let skp_rec = 0;
    let mut tot_amt = 0.0;
    loop {
        let account_opt = log_measurements!(
            diag_log,
            [format!(
                "Type: ReadParseInputAccount, Identifier: `{}`",
                total_accounts_encountered
            )],
            reader_iterator.next()
        );

        if account_opt.is_none() {
            break;
        }
        tot_rec += 1;
        let input_account = account_opt.expect("Unable to parse `input records`.");
        total_accounts_encountered += 1;
        let biu_no_data = BiuData {
            t1: "NONE".to_string(),
            t2: "NONE".to_string(),
            t3: "NONE".to_string(),
            t4: "NONE".to_string(),
        };
        let biu_data = match biu_master.get(&input_account.cust_id) {
            Some(val) => val,
            None => &biu_no_data,
        };
        let t1 = &biu_data.t1;
        let t2 = &biu_data.t2;
        let t3 = &biu_data.t3;
        let t4 = &biu_data.t4;
        let total_deposits = match ttl_bal_master.get(&input_account.cust_id) {
            Some(val) => *val,
            None => DEFAULT_FLOAT,
        };
        tot_amt += input_account.lcy_amount;

        let account_data = append_data(
            input_account,
            t1.to_string(),
            t2.to_string(),
            t3.to_string(),
            t4.to_string(),
            total_deposits,
        );
        writer.write(account_data);
    }
    writer.close();

    let end_generate_timer = SystemTime::now();
    let total_duration = end_generate_timer
        .duration_since(start_generate_timer)
        .expect("Could not calculate total duration for BIU Stamper.");
    println!("Total time taken by BIU Stamper: {:?}", total_duration);

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
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
