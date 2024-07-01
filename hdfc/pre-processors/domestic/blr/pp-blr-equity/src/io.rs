use super::pre_processor::structs::output_data::OutputData;
use super::*;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

pub fn reader(file_path: &str, log: &Logger) -> BufReader<File> {
    match File::open(file_path) {
        Ok(read) => BufReader::new(read),
        Err(er) => {
            log_error!(
                log,
                "Could not foind file `{}` on location `{}` : {}.",
                file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                er
            );
            panic!(
                "Could not find file `{}` on location `{}` : {}.",
                file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                er
            )
        }
    }
}

pub fn write_output(output_data: &OutputData, file_path: &str) {
    let output_line = format!(
        "Name\
    |Desc|\
    FaceValue|\
    Pending Date of a Month|\
    Opening Bal\
    |Highest Date in a Month|\
    Highest Bal\
    |Lowest Date in a Month|\
    Lowest Bal|\
    Closing Date of a Month|\
    Closing Bal|\
    Std Dev\
    \n{}|{}|{}|{}|{:.5}|{}|{:.5}|{}|{:.5}|{}|{:.5}|{:.5}",
        output_data.name,
        output_data.desc,
        output_data.face_value,
        output_data.op_date.format("%d-%m-%Y"),
        output_data.op_bal,
        output_data.high_date.format("%d-%m-%Y"),
        output_data.high_bal,
        output_data.low_date.format("%d-%m-%Y"),
        output_data.low_bal,
        output_data.close_date.format("%d-%m-%Y"),
        output_data.close_bal,
        output_data.std_dev,
    );
    let mut writer = get_writer(file_path);
    output_writer(&mut writer, output_line, file_path);
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
