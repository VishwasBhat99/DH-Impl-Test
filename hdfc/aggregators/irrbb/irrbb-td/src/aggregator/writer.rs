use sdb_io::buf_file_wrtr;
use std::fs::File;
use std::io::BufWriter;

pub fn get_new_writer(llg: i32, output_file_path: &str) -> BufWriter<File> {
    let full_path = format!("{}{}.txt", output_file_path, llg);
    match buf_file_wrtr(&full_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file '{}'. Error: {:?}.",
            full_path, error
        ),
    }
}
