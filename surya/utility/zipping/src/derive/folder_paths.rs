use sdb_io;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct filePaths {
    pub file_paths: Vec<String>,
}

pub fn get_folder(path: &str) -> filePaths {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read files config.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let folder_paths: filePaths =
        serde_json::from_str(&buf[..]).expect("Files config json file was not well-formatted.");
    folder_paths
}
