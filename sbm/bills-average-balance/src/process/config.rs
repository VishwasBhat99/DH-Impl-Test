use std::io::Read;

use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BillsFields {
    pub bills_daily_file_path: String,
    pub acct_id_index: i64,
    pub amt_index: i64,
    pub ccy_code_index: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BALMFCField {
    pub balm_fc_eit_file_path: String,
    pub entity_id_index: i64,
    pub int_rate_index: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub bills_fields: BillsFields,
    pub balm_fc_eit_fields: BALMFCField,
    pub local_ccy: String,
    pub is_exchange_rate_applied: bool,
    pub rate_code: String,
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
