extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub balm_fc_td: String,
    pub balm_fc_td_closed: String,
    pub income_master_current_month: String,
    pub income_master_previous_month: String,
    pub td_daily_files: String,
    pub balm_fc_gsp: String,
    pub exchange_rate: String,
    pub local_currency: String,
    pub pnl_bacid_position: usize,
    pub output_file_path: String,
}

pub fn read_config_file(file_path: &str) -> Config {
    let mut file = File::open(file_path)
        .unwrap_or_else(|_| panic!("Failed to open cofig file: {}", file_path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", file_path));

    let config: Config = serde_json::from_str(&contents)
        .unwrap_or_else(|_| panic!("Failed to parse config file: {}", file_path));

    config
}
