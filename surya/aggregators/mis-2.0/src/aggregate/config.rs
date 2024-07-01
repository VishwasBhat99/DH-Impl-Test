use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub process_type: String,
    pub dim1_file_path: String,
    pub dim2_file_path: String,
    pub dim3_file_path: String,
    pub nwd_codes_file_path: String,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub acc_skip_rules_path: String,
    pub process_field: String,
    pub dim1_fields: Vec<String>,
    pub dim2_fields: Vec<String>,
    pub dim3_fields: Vec<String>,
    pub dim1_type: String,
    pub dim2_type: String,
    pub dim3_type: String,
    pub amt: String,
    pub int_rt: String,
    pub ccy: String,
    pub exchange_rate: String,
    pub is_account_level_exchange_rate: bool,
    pub is_consolidated: bool,
    pub is_negative: bool,
    pub exchange_rate_file_path: String,
    pub numslab_file_path: String,
    pub numslab_sheet_name: String,
    pub mat_date: String,
    pub account_open_date: String,
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
