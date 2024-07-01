use self::derive_fields::get_output_line;
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use simple_csv::SimpleCsvReader;
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod manual_handler;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let input_file_1 = match new_buf_rdr(config_param.input_file_path_1()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path_1(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let input_file_2 = match new_buf_rdr(config_param.input_file_path_2()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path_2(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let input_file_3 = match new_buf_rdr(config_param.input_file_path_3()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path_3(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration for read timer.");
    info!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );
    let mut op_line: String = String::new();
    let mut tot_acc_encntrd_1: i64 = 0;
    let mut acc_pro_suc_1: i64 = 0;
    let mut is_header: bool = true;
    for (line_num, lines) in SimpleCsvReader::new(input_file_1).enumerate() {
        let fields = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path_1(),
                line_num + 1,
                error
            ),
        };
        tot_acc_encntrd_1 += 1;
        if fields.len() == 55 {
            if is_header && fields[2].parse::<i64>().is_err() {
                is_header = false;
                tot_acc_encntrd_1 -= 1;
                continue;
            }
            op_line.push_str(&get_output_line(&fields, config_param.as_on_date(), log));
            acc_pro_suc_1 += 1;
        }
    }

    is_header = true;
    let mut tot_acc_encntrd_2: i64 = 0;
    let mut acc_pro_suc_2: i64 = 0;
    for (line_num, lines) in SimpleCsvReader::new(input_file_2).enumerate() {
        let fields = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path_2(),
                line_num + 1,
                error
            ),
        };
        tot_acc_encntrd_2 += 1;
        if fields.len() == 55 {
            if is_header && fields[2].parse::<i64>().is_err() {
                is_header = false;
                tot_acc_encntrd_2 -= 1;
                continue;
            }
            op_line.push_str(&get_output_line(&fields, config_param.as_on_date(), log));
            acc_pro_suc_2 += 1;
        }
    }

    is_header = true;
    let mut tot_acc_encntrd_3: i64 = 0;
    let mut acc_pro_suc_3: i64 = 0;
    for (line_num, lines) in SimpleCsvReader::new(input_file_3).enumerate() {
        let fields = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path_3(),
                line_num + 1,
                error
            ),
        };
        tot_acc_encntrd_3 += 1;
        if fields.len() == 55 {
            if is_header && fields[2].parse::<i64>().is_err() {
                is_header = false;
                tot_acc_encntrd_3 -= 1;
                continue;
            }
            op_line.push_str(&get_output_line(&fields, config_param.as_on_date(), log));
            acc_pro_suc_3 += 1;
        }
    }

    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}",
            config_param.output_file_path(),
            error
        ),
    }

    let tot_acc_encntrd = tot_acc_encntrd_1 + tot_acc_encntrd_2 + tot_acc_encntrd_3;
    let acc_proc_suc = acc_pro_suc_1 + acc_pro_suc_2 + acc_pro_suc_3;
    log_debug!(
        log,
        "Input: `1`, Total Accounts: `{}`, Account Processed: `{}`.",
        tot_acc_encntrd_1,
        acc_pro_suc_1
    );
    log_debug!(
        log,
        "Input: `2`, Total Accounts: `{}`, Account Processed: `{}`.",
        tot_acc_encntrd_2,
        acc_pro_suc_2
    );
    log_debug!(
        log,
        "Input: `3`, Total Accounts: `{}`, Account Processed: `{}`.",
        tot_acc_encntrd_3,
        acc_pro_suc_3
    );
    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        acc_proc_suc,
        tot_acc_encntrd - acc_proc_suc,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
