use serde_derive::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub input_sheet_name: String,
    pub key_num: i64,
    pub col_to_skip: usize,
    pub row_to_skip: Vec<usize>,
}

pub fn get_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read files config.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: Files =
        serde_json::from_str(&buf[..]).expect("Files config json file was not well-formatted.");
    files_config
}
