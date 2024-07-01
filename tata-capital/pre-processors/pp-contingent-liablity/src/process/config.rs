use std::io::Read;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigData {
    pub code: String,
    pub excel_path: String,
    pub amount_field: i64,
    pub cf_date_field: i64,
    pub look_up_field: i64,
    pub sheet_name : String,
    pub look_up_value: String,
    pub identifier_field: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub config_file_1: String,
    pub config_file_2: Vec<ConfigData>,
}

pub fn get_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read config file.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: Files =
        serde_json::from_str(&buf[..]).expect("Config json file was not well-formatted.");
    files_config
}

