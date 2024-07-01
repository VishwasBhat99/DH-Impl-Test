use super::{macros, Default, Logger};
use csv::{Error, Reader, ReaderBuilder, Trim};
use sdb_io::buf_file_wrtr;
use std::fs::File;
use std::io::BufWriter;

pub fn read_file(file_path: &str) -> Reader<File> {
    match ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .trim(Trim::All)
        .delimiter(b'|')
        .from_path(file_path)
    {
        Ok(read) => read,
        Err(error) => panic!("Could not found file `{}` : {}.", file_path, error),
    }
}

pub fn extract_lines<T: Default>(
    line_num: usize,
    lines: Result<T, Error>,
    file_path: &str,
    log: &Logger,
) -> T {
    match lines {
        Ok(line) => line,
        Err(error) => {
            log_error!(
                log,
                "Unable to read file `{}` at line number: `{}` : {}",
                file_path,
                line_num + 1,
                error
            );
            Default::default()
        }
    }
}

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` : {}", file_path, error),
    }
}
