use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFiles {
    pub files: Vec<ConfigFile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub input_file_path: String,
    pub exrt_file_path: String,
    pub metadata_file_path: String,
    pub req_fields_file_path: String,
    pub amb_file_path: String,
    pub source: String,
    pub is_consolidated: bool,
    pub is_cf_req: bool,
    pub is_int_amt_consolidated: bool,
}

pub fn get_files(path: &str) -> ConfigFiles {
    let mut file = open_file(path);
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: ConfigFiles =
        serde_json::from_str(&buf[..]).expect("Files config json file was not well-formatted.");
    files_config
}
