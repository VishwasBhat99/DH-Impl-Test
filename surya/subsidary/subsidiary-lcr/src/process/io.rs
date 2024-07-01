use std::fs::File;
use std::io::BufReader;
use std::io::Write;

pub fn read_file(file_path: &str) -> BufReader<File> {
    let file_handle = File::open(file_path).expect("Error while opening the input file.");
    let buf_reader = BufReader::new(file_handle);
    buf_reader
}

pub fn create_file(input_file: &str) -> File {
    let file_handle = File::create(input_file).expect("Error creating output file");
    file_handle
}

pub fn output_writer(out_file: &mut File, op_line: String) {
    out_file
        .write_all(op_line.as_bytes())
        .expect("could not write into output file");
}
