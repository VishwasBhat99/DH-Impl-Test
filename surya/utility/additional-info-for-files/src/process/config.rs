use sdb_io;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<InputFile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputFile {
    pub master_name: String,
    pub input_file: String,
    pub sheet_name: String,
}

pub fn get_all_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read files config.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: Files =
        serde_json::from_str(&buf[..]).expect("Files config json file was not well-formatted.");
    files_config
}
