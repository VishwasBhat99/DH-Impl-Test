use crate::calamine::open_workbook_auto;
use std::fs::File;
use std::io::Write;
use std::str;

pub fn open_file(output_file: &str) -> calamine::Sheets {
    let workbook = open_workbook_auto(output_file).expect("Error while opening the input file");
    workbook
}

pub fn write_file(out_data: Vec<String>, out_file: &mut File) {
    for i in 0..out_data.len() {
        writeln!(out_file, "{}|{}", i + 1, out_data[i])
            .expect("Error while writing into output file");
    }
}

pub fn create_file(output_file: &str) -> File {
    let file_handle = File::create(output_file).expect("Error creating output file");
    file_handle
}
