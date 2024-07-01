use sdb_io;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Read;

#[derive(Eq, PartialEq, Hash)]
pub struct LookupAcc {
    pub acc_no: String,
    pub source_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputPositions {
    pub acc_num_position: usize,
    pub ftp_rate_position: usize,
    pub ftp_amount_ccy_position: usize,
    pub ftp_amount_hcy_position: usize,
    pub adj_rate_position: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub override_file_path: String,
    pub stamper_file_path: String,
    pub extracted_file_path: String,
    pub output_file_path: String,
    pub stamper_delimiter: String,
    pub override_delimiter: String,
    pub source_name: String,
    pub extracted_file_positions: InputPositions,
    pub stamper_file_positions: InputPositions,
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
