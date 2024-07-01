use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub master_lookup_keys: String,
    pub input_lookup_keys: String,
    pub lookup_value: String,
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
