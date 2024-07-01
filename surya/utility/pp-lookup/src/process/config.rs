use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct File {
    pub input_file_path_1: String,
    pub input_file_path_2: String,
    pub metadata_file_path_1: String,
    pub metadata_file_path_2: String,
    pub req_fields_1: Vec<String>,
    pub req_fields_2: Vec<String>,
    pub output_file_path: String,
    pub inp1_lookup_key: String,
    pub inp2_lookup_key: String,
    pub delimiter: String,
    pub is_header_req: bool,
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
