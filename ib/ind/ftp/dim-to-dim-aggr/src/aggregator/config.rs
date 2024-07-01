extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InputStamperFiles {
    pub stamper_file_paths: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub stamper_files: InputStamperFiles,
    pub output_file_path: String,
    pub is_aorl_null: String,
    pub dimid: String,
    pub dim_item_id_position: usize,
    pub rlgid: String,
    pub rlg_item_id_position: usize,
    pub is_weighted_rate_required: bool,
}

pub fn read_config_file(file_path: &str) -> Config {
    let mut file = File::open(file_path)
        .unwrap_or_else(|_| panic!("Failed to open config file: {}", file_path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", file_path));

    let config: Config = serde_json::from_str(&contents)
        .unwrap_or_else(|_| panic!("Failed to parse config file: {}", file_path));

    config
}
