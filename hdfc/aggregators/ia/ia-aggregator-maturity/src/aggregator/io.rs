use super::{macros, Default, Logger};
use csv::{Error, Reader, ReaderBuilder, Trim};
use std::{env::current_dir, fs::File};

pub fn read_file(file_path: &str) -> Reader<File> {
    match ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .trim(Trim::All)
        .delimiter(b'|')
        .from_path(file_path)
    {
        Ok(read) => read,
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

pub fn extract_lines<T: Default>(
    line_num: usize,
    lines: Result<T, Error>,
    file_path: &str,
    log: &Logger,
) -> T {
    match lines {
        Ok(line) => line,
        Err(error) => {
            log_debug!(
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
