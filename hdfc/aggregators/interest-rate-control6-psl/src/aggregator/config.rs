use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub summary_file_path: String,
    pub psl_master_file: String,
    pub psl_sheet_name: String,
    pub drilldown_file_path: String,
    pub input_files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub prev_mth_src_file: String,
    pub cur_mth_src_file: String,
    pub req_fields_file_path: String,
    pub account_metadata_file_path: String,
    pub source_system: String,
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
