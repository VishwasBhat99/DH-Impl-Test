use serde_derive::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub curr_rth_file_path: String,
    pub rate_code: String,
    pub home_crcy: String,
    pub is_exchange_rate_applied: bool,
    pub def_multiplier: f64,
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
