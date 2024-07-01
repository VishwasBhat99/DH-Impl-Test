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

pub fn buff_reader(path: &str) -> BufReader<File> {
    match new_buf_rdr(path) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            path,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    }
}
