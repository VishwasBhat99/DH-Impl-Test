use sdb_io::buf_file_wrtr;
use std::io::BufWriter;
use std::{env::current_dir, fs::File};

pub fn get_new_writer(source: String, output_file_path: &str) -> BufWriter<File> {
    let writer_path = format!("{}_{}.txt", output_file_path, source);
    get_writer(&writer_path)
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
