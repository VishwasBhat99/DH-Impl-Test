use sdb_io::buf_file_wrtr;
use std::io::prelude::*;
use std::io::BufWriter;
use std::{
    env::current_dir,
    fs::{File, OpenOptions},
};

pub fn get_append_writer(file_path: &str) -> BufWriter<File> {
    let file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(error) => panic!(
            "Error while appending data to `{}` file : `{}`.",
            file_path, error
        ),
    };
    BufWriter::new(file)
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

pub fn output_writer(writer: &mut BufWriter<File>, output_lines: String, file_path: &str) {
    match writer.write_all(output_lines.as_bytes()) {
        Ok(_) => println!("Successfully written data on `{}`.", file_path),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            file_path, error
        ),
    };
}
