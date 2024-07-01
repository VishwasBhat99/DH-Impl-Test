use std::fs::File;

pub fn create_file(input_file: &str) -> File {
    let file_handle = File::create(input_file).expect("Error creating output file");
    file_handle
}
