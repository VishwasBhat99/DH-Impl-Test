extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::{Benchmark, InputAccount, IntRateData, NPAData, TblCodes};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;
mod input_account;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut writer = BufWriter::new(output_file);
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut int_rate_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.int_rate_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.int_rate_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut benchmark_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.benchmark_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.benchmark_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut tbl_code_reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.tbl_code_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.tbl_code_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let npa_file = match new_buf_rdr(config_param.npa_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.npa_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let config_file = match new_buf_rdr(config_param.config_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.config_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut int_rate_map: HashMap<String, IntRateData> = HashMap::new();
    for (line_num, lines) in int_rate_reader.deserialize().enumerate() {
        let int_rate_data: IntRateData = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.int_rate_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        int_rate_map.insert(int_rate_data.acid.to_owned(), int_rate_data);
    }
    let mut benchmark_map: HashMap<String, Benchmark> = HashMap::new();
    let mut benchmark_foracid: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in benchmark_reader.deserialize().enumerate() {
        let benchmark_data: Benchmark = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.benchmark_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        benchmark_foracid.insert(benchmark_data.foracid.to_owned(), "N".to_string());
        benchmark_map.insert(benchmark_data.acid.to_owned(), benchmark_data);
    }
    let mut tblcodes_set: HashSet<String> = HashSet::new();
    for (line_num, lines) in tbl_code_reader.deserialize().enumerate() {
        let tbl_code_data: TblCodes = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.tbl_code_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tblcodes_set.insert(tbl_code_data.int_tbl_code);
    }
    let mut npa_map: HashMap<String, NPAData> = HashMap::new();
    for (line_num, lines) in npa_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.npa_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        npa_map.insert(
            fields[1].to_string(),
            NPAData {
                npa_classification: fields[8].to_string(),
                cust_hlth_code: fields[12].to_string(),
                cust_npa_class: fields[17].to_string(),
                final_npa_class: fields[18].to_string(),
                npa_amount: fields[10].to_string(),
            },
        );
    }
    let mut config_vec: Vec<String> = Vec::new();
    for (_line_num, line) in config_file.lines().enumerate() {
        config_vec.push(line.unwrap_or_else(|_| "".to_string()));
    }
    for (line_num, lines) in reader.deserialize().enumerate() {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        tot_acc_encntrd += 1;
        let temp_string = get_op_line(
            &input_account,
            &int_rate_map,
            &benchmark_map,
            &benchmark_foracid,
            &tblcodes_set,
            &npa_map,
            &config_vec,
        );
        op_line.push_str(temp_string.as_str());
        op_line.push('\n');
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(op_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
