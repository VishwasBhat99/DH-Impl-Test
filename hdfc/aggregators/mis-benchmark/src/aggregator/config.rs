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
    pub exrt_file_path: String,
    pub metadata_file_path: String,
    pub required_fields_file_path: String,
    pub source: String,
    pub is_consolidated: bool,
    pub is_maturity: bool,
    pub is_negative: bool,
    pub acc_skip_rules_path: String,
    pub is_exclusion_rules_required: bool,
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
