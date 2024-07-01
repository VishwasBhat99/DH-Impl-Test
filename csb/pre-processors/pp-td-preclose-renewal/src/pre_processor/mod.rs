use self::derive_fields::get_renewal_op_line;
use self::derive_fields::get_premat_op_line;
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use macros;
use std::io::prelude::*;
use std::io::BufReader;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let preclose_input_file = match new_buf_rdr(config_param.input_preclose_file_path()) {
        Ok(preclose_input_file) => preclose_input_file,
        Err(error) => panic!("{}", error),
    };
    let renewal_input_file = match new_buf_rdr(config_param.input_renewal_file_path()) {
        Ok(renewal_input_file) => renewal_input_file,
        Err(error) => panic!("{}", error),
    };
    let preclose_reader = BufReader::new(preclose_input_file);
    let renewal_reader = BufReader::new(renewal_input_file);
    let start_derive_timer = SystemTime::now();
    let mut output_line = String::new();
    let mut tot_acc_encntrd:i64 = 0;
    let mut skp_acc:i64 = 0;
    let mut writer = BufWriter::new(output_file);

    for line in preclose_reader.lines().skip(1) {
        let mut acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        acc_info.pop();
        acc_info.remove(0);

        let  fields: Vec<&str> = acc_info.split("|").collect();
        tot_acc_encntrd += 1;

        if fields.len() != 12 {
            skp_acc += 1;
            continue;
        } 

        let temp_output_line = get_premat_op_line(&mut output_line, config_param.as_on_date, fields);
        output_line.push_str(&temp_output_line);
        output_line.push_str("TDP");
        output_line.push('\n');
    }
    for line in renewal_reader.lines().skip(1) {
        let mut acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        acc_info.pop();
        acc_info.remove(0);
        let  fields: Vec<&str> = acc_info.split("|").collect();

        tot_acc_encntrd += 1;

        if fields.len() != 12 {
            skp_acc += 1;
            continue;
        } 

        let temp_output_line = get_renewal_op_line(&mut output_line, config_param.as_on_date, fields);
        output_line.push_str(&temp_output_line);
        output_line.push_str("TDR");
        output_line.push('\n');
    }

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    match writer.write_all(output_line.as_bytes()) {
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
