use super::{macros, Default, Logger};
use csv::{Error, Reader, ReaderBuilder, Trim};
use sdb_io::buf_file_wrtr;
use std::io::BufWriter;
use std::{env::current_dir, fs::File};

pub fn read_file(file_path: &str, def_path: &str, log: &Logger) -> Reader<File> {
    match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .flexible(true)
        .trim(Trim::All)
        .from_path(file_path)
    {
        Ok(read) => read,
        Err(error) => {
            log_error!(
                log,
                "Could not found file `{}` on location `{}` : {}.",
                file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            );
            ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b'|')
                .from_path(def_path)
                .expect("Error while getting empty file path.")
        }
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
