use super::{macros, Logger};
use csv::{Error, Reader, ReaderBuilder, Trim};
use std::io::prelude::*;
use std::{env::current_dir, fs::File};

pub fn read_file(file_path: &str) -> Reader<File> {
    match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .flexible(true)
        .trim(Trim::All)
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

pub fn create_file(input_file: &str) -> File {
    let file_handle = File::create(input_file).expect("Error creating output file");
    file_handle
}

pub fn output_writer(out_file: &mut File, op_line: String) {
    out_file
        .write_all(op_line.as_bytes())
        .expect("could not write into output file");
}

pub fn read_sec_file(file_path: &str) -> Reader<File> {
    match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .flexible(true)
        .trim(Trim::All)
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
