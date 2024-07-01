use self::derive_fields::get_op_line;
use calamine::{open_workbook, Reader, Xls};
use configuration_parameters::ConfigurationParameters;
use macros;
use pre_processor::reconcilation::ReconKey;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_derive_timer = SystemTime::now();
    let mut op_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut skip_rec: i64 = 0;

    let mut input_file: Xls<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for mut row in reader.rows() {
            tot_acc_encntrd += 1;
            if row[7].to_string().parse::<f64>().is_err() {
                skip_rec += 1;
                continue;
            }
            op_line.push_str(&get_op_line(&mut row, *config_param.as_on_date(), &log));

            let recon_key = ReconKey::new(
                row[6].to_string(),
                config_param.gl_type().to_string(),
                "".to_string(), // TODO: missing gl code
            );
            let amt: f64 = row[28].to_string().parse::<f64>().unwrap_or(0.0)
                / row[27].to_string().parse::<f64>().unwrap_or(1.0);
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
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
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }
    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut recon_op_line = String::new();
    for (key, value) in recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            config_param.input_file_name(),
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push_str("\n");
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error,
        ),
    };
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec,
        skip_rec,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
