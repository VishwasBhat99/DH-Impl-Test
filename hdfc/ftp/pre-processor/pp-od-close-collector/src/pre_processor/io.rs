use sdb_io::{buf_file_wrtr, new_buf_rdr};
use std::io::{BufReader, BufWriter};
use std::{env::current_dir, fs::File};

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            file_path,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    }
}

pub fn read_file(file_path: &str) -> BufReader<File> {
    match new_buf_rdr(file_path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            file_path,
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
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
