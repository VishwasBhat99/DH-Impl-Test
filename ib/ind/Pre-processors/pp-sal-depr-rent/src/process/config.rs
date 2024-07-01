use std::io::Read;
extern crate serde_derive;
use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub sal_input_file: String,
    pub depr_input_file: String,
    pub rent_input_file: String,
    pub soldim_input_file: String,
    pub al_pl_file: String,
    pub ogl_file: String,
    pub sal_sheet_name: String,
    pub depr_sheet_name: String,
    pub rent_sheet_name: String,
    pub al_pl_sheet_name: String,
    pub sal_code: String,
    pub depr_code: String,
    pub sal_desc: String,
    pub depr_desc: String,
    pub rent_desc: String,
    pub zone_code: String,
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
