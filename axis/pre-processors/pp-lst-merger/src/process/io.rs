use process::{BufWriter, File};
use sdb_io::buf_file_wrtr;

pub fn get_writer(file_path: &str) -> BufWriter<File> {
    match buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}`: {}", file_path, error),
    }
}
