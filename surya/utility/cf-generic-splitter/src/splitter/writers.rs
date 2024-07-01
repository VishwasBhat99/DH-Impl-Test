use macros;
use sdb_io::buf_file_wrtr;
use splitter::Logger;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn get_new_writer(source: String, output_file_path: &str) -> BufWriter<File> {
    let full_path = format!("{}{}.txt", output_file_path, source);
    match buf_file_wrtr(&full_path, None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file '{}'. Error: {:?}.",
            full_path, error
        ),
    }
}

pub fn write_data(writer: &mut BufWriter<File>, data: String, logger: &Logger) {
    // Write the account data
    let data = data.as_bytes();
    match writer.write(data) {
        Ok(_val) => {}
        Err(err) => {
            log_info!(logger, "Error writing to output file. Error: {}", err);
        }
    }
}
