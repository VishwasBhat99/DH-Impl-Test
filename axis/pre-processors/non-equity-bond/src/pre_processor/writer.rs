use macros;
use pre_processor::Logger;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

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
