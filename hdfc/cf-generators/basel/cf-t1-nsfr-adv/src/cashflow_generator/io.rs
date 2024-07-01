use sdb_io::*;
use std::{fs::File, io::BufReader};

pub fn read_file(file_path: &str) -> BufReader<File> {
    match new_buf_rdr(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Could not found file `{}` : `{}`.", file_path, error),
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
