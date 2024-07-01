use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub input_files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub file_path: String,
    pub file_type: String,
    pub position_considered: Vec<usize>,
    pub position_changed: Vec<Vec<usize>>,
    pub output_path: String,
    pub no_lines_skipped: Vec<usize>,
    pub separator: String,
    pub sheet_names: Vec<String>,
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
