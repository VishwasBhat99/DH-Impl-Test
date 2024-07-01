use configuration_parameters::ConfigurationParameters;
use process::derive_fields::get_op_line;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
mod derive_fields;
use macros;

struct MCLRData {
    pegged_flag: String,
    repricing_plan: String,
    peg_review_date: String,
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let balm_mclr = match new_buf_rdr(config_params.balm_mclr()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.balm_mclr(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let gam_datafile = match new_buf_rdr(config_params.gam_datafile()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.gam_datafile(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut balm_mclr_hm: HashMap<String, MCLRData> = HashMap::new();

    for (line_num, lines) in balm_mclr.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.balm_mclr(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        //Field 1:ACCOUNT_NUMBER, Field 8:REPRICING_PLAN
        balm_mclr_hm.insert(
            fields[0].to_string(),
            MCLRData {
                pegged_flag: fields[6].trim().to_string(),
                repricing_plan: fields[7].trim().to_string(),
                peg_review_date: fields[3].to_string(),
            },
        );
    }
    let mut op_line = String::new();
    for (line_num, lines) in gam_datafile.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.gam_datafile(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let foracid = fields[1].to_string();
        match balm_mclr_hm.get(&foracid) {
            Some(data) => {
                op_line.push_str(&get_op_line(
                    fields[0].to_string(),
                    fields[1].to_string(),
                    data.pegged_flag.to_owned(),
                    data.repricing_plan.to_owned(),
                    data.peg_review_date.to_owned(),
                ));
            }
            None => {
                log_debug!(
                    logger,
                    "Acc num not found for Foracid: {} in record with ACID: {} ",
                    fields[0],
                    fields[1]
                );
            }
        }
    }
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
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
            config_params.output_file_path(),
            error,
        ),
    }
}
