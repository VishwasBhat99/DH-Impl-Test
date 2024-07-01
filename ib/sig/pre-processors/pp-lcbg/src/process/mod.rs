use self::account::*;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
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

    //Reading Master Data File
    let master = File::open(&config_params.master_file_path()).expect("Could Not Read Master File");
    let master_reader = BufReader::new(master);

    let mut master_map: HashMap<String, MasterData> = HashMap::new();
    for line in master_reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from master file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        let master_data = MasterData::new(fields.to_owned());
        let concat = MasterData::get_concat(master_data.to_owned());
        master_map.insert(concat, master_data);
    }

    //Reading Input Master File
    let input = File::open(&config_params.input_file_path()).expect("Could Not Read Input File");
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
        let ref_no = input_fields[1].to_string();
        if input_fields.len() != 39 {
            skip_rec_count += 1;
            log_error!(log,
                "Cannot read line {} from input file for ref_no: {:?} due to incorrect column count {:?}",
                line_no + 1,
                ref_no,
                input_fields.len());
            continue;
        }
        let mut account = Account::get_data(input_fields, &config_params);
        let concat = format!(
            "{}{}{}{}",
            account.acod, account.customer_id, account.ref_type, account.cust_type
        );
        if !master_map.contains_key(&concat.to_string()) {
            log_debug!(log, "Concat `{}` not found in Master Data", concat);
        }
        account.a7 = concat.to_string();
        account.a8 = master_map
            .get(&concat.to_string())
            .unwrap_or(&MasterData::def())
            .vs_param
            .to_string();
        account.a9 = master_map
            .get(&concat.to_string())
            .unwrap_or(&MasterData::def())
            .vg_param
            .to_string();
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
