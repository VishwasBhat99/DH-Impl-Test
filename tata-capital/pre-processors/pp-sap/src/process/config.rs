use std::io::Read;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_1: String,
    pub input_file_2: String,
    pub input_file_3: String,
    pub input_file_4: String,
    pub input_file_5: String,
    pub input_file_6: String,
    pub input_file_7: String,
    pub input_file_8: String,
    pub input_file_9: String,
    pub input_file_10: String,
    pub input_file_11: String,
    pub file7_sheet_name: String,
    pub file8_sheet_name: String,
    pub file9_sheet_name: String,
    pub file10_sheet_name: String,
}

pub fn get_files(path: &str) -> File {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read files config.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: File =
        serde_json::from_str(&buf[..]).expect("Files config json file was not well-formatted.");
    files_config
}
