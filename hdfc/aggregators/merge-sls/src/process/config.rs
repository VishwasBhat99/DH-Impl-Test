use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub output_file_path: String,
    pub exchange_file_path: String,
    pub master_file_path: String,
    pub llg_details_end: usize,
    pub bucket_end: usize,
    pub cf_type: usize,
    pub llg_id: usize,
    pub item_type: usize,
    pub level_1: usize,
    pub level_2: usize,
    pub level_3: usize,
    pub level_4: usize,
    pub base_currency: String,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub currency: String,
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
