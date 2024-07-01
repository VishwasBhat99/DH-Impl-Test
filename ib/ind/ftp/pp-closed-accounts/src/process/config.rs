use std::io::Read;
extern crate serde_derive;
extern crate serde_json;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub key_column: String,
    pub amount_column: String,
    pub expected_column_count: String,
    pub date_field_columns: Option<Vec<String>>,
    pub field_separator: Option<String>,
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
