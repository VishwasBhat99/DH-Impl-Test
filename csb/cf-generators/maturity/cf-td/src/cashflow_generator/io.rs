use sdb_io::{buf_file_wrtr, new_buf_rdr};
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufWriter};

pub fn buf_reader(file_path: &str) -> BufReader<File> {
    match new_buf_rdr(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Unable to open file at path '{}'. Error: {}", file_path, e);
        }
    }
}

pub fn buf_file_writer(path: &str, suffix: &str, buffer_size: Option<usize>) -> BufWriter<File> {
    let mut full_path = path.to_string();
    full_path.push_str(suffix);

    match buf_file_wrtr(&full_path, buffer_size) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file '{}'. Error: {:?}.", full_path, error),
    }
}

pub fn flush_contents(mut writer: BufWriter<File>, writer_name: &str) {
    let flush_result = writer.flush();
    if flush_result.is_err() {
        // TODO: Also include the filepath of the writer if possible.
        panic!(
            "Program failed when flushing contents of writer: '{}'. Error: {}. Aborting execution.",
            writer_name,
            flush_result
                .err()
                .expect("Unexpected error occured while flushing buffer.")
        );
    }
}
