use super::{macros, Logger};
use sdb_io::new_buf_rdr;
use std::io::BufReader;
use std::{env::current_dir, fs::File};

pub fn read_file(file_path: &str, log: &Logger) -> BufReader<File> {
    match new_buf_rdr(file_path) {
        Ok(file) => file,
        Err(error) => {
            let err_msg = format!(
                "Could not found file `{}` on location `{}` : {}.",
                file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            );
            log_error!(log, "{}", err_msg);
            panic!("{}", err_msg);
        }
    }
}

pub fn extract_lines(
    line_num: usize,
    lines: Result<String, std::io::Error>,
    file_path: &str,
) -> String {
    match lines {
        Ok(line) => line,
        Err(error) => panic!(
            "Unable to read file `{}` at line number: `{}` : {}",
            file_path,
            line_num + 1,
            error
        ),
    }
}
