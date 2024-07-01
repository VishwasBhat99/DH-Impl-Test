pub mod aggr_key;
pub mod cfinput;
pub mod io;

use ftp_parameters::FtpParameters;
use macros;
use sdb_io::buf_file_wrtr;
use std::env::current_dir;
use std::io::prelude::*;

pub mod aggr_bal_no_lock;
pub mod aggr_bal_with_lock;

pub fn process_records(ftp_parameters: &mut FtpParameters) {
    let mut op_line = String::new();
    let mut report_string = String::new();

    if ftp_parameters.cp.is_matched_term_lock() {
        let (op_line_acc, report_string_acc) =
            aggr_bal_with_lock::aggr_bal_with_lock(ftp_parameters);
        op_line = op_line_acc;
        report_string = report_string_acc;
    } else {
        let (op_line_acc, report_string_acc) =
            aggr_bal_no_lock::aggr_bal_with_no_lock(ftp_parameters);
        op_line = op_line_acc;
        report_string = report_string_acc;
    }

    let mut out_writer = match buf_file_wrtr(&ftp_parameters.cp.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file `{}` on location `{}` : {}",
            &ftp_parameters.cp.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match out_writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully written output file."),
        Err(error) => panic!(
            "Unable to write output lines to file `{}`: {}.",
            &ftp_parameters.cp.output_file_path(),
            error
        ),
    };

    log_info!(ftp_parameters.log, "{}", report_string);
    println!("{}", report_string);
}
