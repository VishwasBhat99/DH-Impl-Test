use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub process_type: String,
    pub numslab_file_path: String,
    pub prdslab_file_path: String,
    pub srcmapslab_file_path: String,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub acc_skip_rules_path: Option<String>,
    pub acc_id: Option<String>,
    pub process_field: String,
    pub is_dim_input_stamped: Vec<bool>,
    pub dims_fields: Vec<Vec<String>>,
    pub dims_type: Vec<String>,
    pub amt: String,
    pub int_rt: String,
    pub ccy: String,
    pub exchange_rate: String,
    pub is_account_level_exchange_rate: bool,
    pub is_consolidated: bool,
    pub is_negative: bool,
    pub exchange_rate_file_path: String,
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
