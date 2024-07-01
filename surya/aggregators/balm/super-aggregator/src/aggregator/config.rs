use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub rules_file_path: String,
    pub req_fields_file_path: String,
    pub is_account_level_exchange_rate: bool,
    pub is_consolidated: bool,
    pub is_rep_mandatory: bool,
    pub is_non_maturity: bool,
    pub is_npa: String,
    pub npa_values: Vec<String>,
    pub default_overdue_llg_code: i32,
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
