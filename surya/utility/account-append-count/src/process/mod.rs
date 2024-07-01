use super::configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

pub fn write_suffix_data(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let mut acc_enc = 0;
    let mut acc_skip = 0;

    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);

    // init output writer
    let mut output_file = match sdb_io::buf_file_wrtr(&config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(_) => {
            panic!(
                "Could not create output file: `{}`.",
                config_params.output_file_path(),
            );
        }
    };
    let mut processed_acc_num: HashMap<String, i64> = HashMap::new();
    let pos = config_params.acc_pos();
    for line in reader.lines() {
        acc_enc += 1;
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                log_info!(
                    logger,
                    "Unable to read the line: {}, error: {}",
                    acc_enc + 1,
                    error
                );
                acc_skip += 1;
                continue;
            }
        };
        let mut fields: Vec<&str> = acc_info.split(config_params.delimiter()).collect();
        let account_number = fields[pos - 1];

        processed_acc_num
            .entry(account_number.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let acc_no = match processed_acc_num.get(account_number) {
            Some(count) => format!("{}-{}", account_number, count),
            None => format!("{}", account_number),
        };
        fields[pos - 1] = &acc_no;
        let mut op_line = fields.iter().fold(String::new(), |acc, &arg| acc + arg + config_params.delimiter());
        op_line.pop();
        op_line.push_str("\n");
        output_file
            .write_all(&op_line.as_bytes())
            .expect("Unable to write to output file!");
    }
    output_file.flush().unwrap();
    println!("Total account encountered: {}", acc_enc);
    println!("Total account processed: {}", acc_enc - acc_skip);
    let health_report = HealthReport::new(acc_enc, acc_enc - acc_skip, acc_skip, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
