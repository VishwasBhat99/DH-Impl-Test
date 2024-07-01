use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub output_file_path: String,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct File {
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub req_fields_file_path: String,
    pub int_basis: String,
    pub from_period: String,
    pub to_period: String,
    pub cust_master_file_path: String,
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
