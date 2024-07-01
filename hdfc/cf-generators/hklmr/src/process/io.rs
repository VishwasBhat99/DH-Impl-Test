use super::struct_data::Data;
use macros;
use protobuf::Message;
use slog::Logger;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_file(log: &Logger, file_writer: &mut BufWriter<File>, record: Data) {
    let output_bytes = match record.write_length_delimited_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            log_error!(
                log,
                "Couldn't parse to bytes. Aborting execution. Account: '{}' Error: {:?}.",
                record.lmr_field,
                e
            );
            return;
        }
    };

    file_writer.write(&output_bytes).unwrap();
}
