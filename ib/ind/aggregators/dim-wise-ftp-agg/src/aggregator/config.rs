use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigData {
    pub stamper_files: Vec<String>,
    pub dims: Vec<String>,
    pub weighted_int_rt_req: String,
    pub is_aorl_null: String,
}

impl ConfigData {
    pub fn new_from_path(path: &str) -> ConfigData {
        let mut file = sdb_io::open_file_read(path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let config_data: ConfigData = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        config_data
    }
}
