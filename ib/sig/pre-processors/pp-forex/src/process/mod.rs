use self::account::{format_output, Account};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::{BufReader, BufWriter, Write};
use std::time::SystemTime;

mod account;

pub fn process(config_params: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut tot_acc_encntrd: i64 = 0;
    let mut skip_rec_count: i64 = 0;
    let mut writer = BufWriter::new(output_file);

    //Reading Input Master File
    let input = File::open(config_params.input_file_path()).expect("Could Not Read Input File");
    let input_reader = BufReader::new(input);

    for (line_no, lines) in input_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        let acc_info: String = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_no + 1,
                    error
                );
                Default::default()
            }
        };
        let input_fields: Vec<&str> = acc_info.split('|').collect();
        let gl_acc_id = input_fields[1].to_string();
        if input_fields.len() != 23 {
            skip_rec_count += 1;
            log_error!(log,
                "Cannot read line {} from input file for branch: {:?} due to incorrect column count {:?}",
                line_no + 1,
                gl_acc_id,
                input_fields.len());
            continue;
        }
        let mut account = Account::get_data(input_fields, &config_params);
        if account.major_ccy != "S" {
            account.a1 = account.ccy_amt.to_owned();
            account.a2 = account.ccy1_amt.to_owned();
            account.a7 = account.ccy.to_owned();
            account.a8 = account.ccy1.to_owned();
        }
        let output_line = format_output(&account);
        writer
            .write_all(output_line.as_bytes())
            .expect("Error writing default account data to output path !!");
    }

    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_derive_timer)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);

    // Generate Health Check Report
    let health_report = health_report::HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec_count,
        skip_rec_count,
        0.0,
        0.0,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file_path());
}
