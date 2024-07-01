use crate::process::Fields;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub fn read_file(file_path: &str) -> BufReader<File> {
    let file_handle = File::open(file_path).expect("Error while opening the input file.");
    let buf_reader = BufReader::new(file_handle);
    buf_reader
}

pub fn read(file_path: &str) -> Vec<String> {
    let file_handle = File::open(file_path).expect("Error while opening the OpsAccData file.");
    let buf_reader = BufReader::new(file_handle);
    let mut open_acc_file_data = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("Error while reading a line from OpsAccData file.");
        let client_code = line.to_string();
        open_acc_file_data.push(client_code);
    }
    open_acc_file_data
}

pub fn create_file(file: &str) -> File {
    let file_handle = File::create(file).expect("Error while creating a output file.");
    return file_handle;
}

pub fn write_file(fields: Fields, output_file: &mut File, line: String) {
    writeln!(
        output_file,
        "{}|{}|{}|{}|{}",
        line, fields.T1, fields.T2, fields.T3, fields.T4
    )
    .expect("Error while writing into the output file.");
}
