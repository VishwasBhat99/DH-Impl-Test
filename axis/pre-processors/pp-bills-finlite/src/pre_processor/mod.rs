extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::{BillSCFM, BillsGL};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
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
    let mut op_line: String = String::new();
    let mut writer = BufWriter::new(output_file);

    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.scfm_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.scfm_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let gl_file = match new_buf_rdr(config_param.gl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file: `{}` on location `{}` : {}.",
            config_param.gl_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut gl_hashmap: HashMap<String, BillsGL> = HashMap::new();
    for (line_num, lines) in gl_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gl_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        gl_hashmap.insert(
            fields[1].to_string(),
            BillsGL {
                gl_sub_head_code: fields[0].to_string(),
                foracid: fields[1].to_string(),
                cust_id: fields[2].to_string(),
                schm_code: fields[3].to_string(),
                schm_type: fields[4].to_string(),
                clr_bal_amt: fields[5].to_string(),
                un_clr_bal_amt: fields[6].to_string(),
                acct_crncy_code: fields[7].to_string(),
            },
        );
    }

    for (line_num, lines) in reader.deserialize().enumerate() {
        let scfm_data: BillSCFM = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.scfm_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        //Process the output line only if BILLS_SCFM_FORACID=BILLS_FORACID.
        if gl_hashmap.contains_key(&scfm_data.scfm_foracid) {
            let gl_data = gl_hashmap
                .get(&scfm_data.scfm_foracid)
                .expect("Cannot fetch gl data from GL file.")
                .to_owned();

            let op_rec = get_op_line(&scfm_data, &gl_data);
            op_line.push_str(op_rec.as_str());
            op_line.push('\n');
        } else {
            log_debug!(
                log,
                "SCFM FORACID not found for :{} in GL file.",
                scfm_data.scfm_foracid
            );
        }
    }

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
