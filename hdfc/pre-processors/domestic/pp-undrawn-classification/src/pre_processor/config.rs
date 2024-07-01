use std::io::Read;

use self::serde_derive::Deserialize;
use self::serde_derive::Serialize;
extern crate serde;
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    pub files: Vec<Config>,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct UbsData {
    pub description: String,
    pub f_nf: String,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct LnmData {
    pub v_exposure_id: String,
    pub f_nf: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SheetName {
    pub lnm_sheet_name: String,
    pub ubs_sheet_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub lcr_master_sheet_names: SheetName,
    pub lcr_master_basel_sheet_name: String,
    pub odfd_sheet_name: String,
    pub template_undrawn_sheet_name: SheetName,
}

pub fn get_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read config file.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: Files =
        serde_json::from_str(&buf[..]).expect("Config json file was not well-formatted.");
    files_config
}
