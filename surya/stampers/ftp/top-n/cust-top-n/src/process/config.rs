use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]

pub struct Files {
    pub files: Vec<Config>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub input_file_path: Vec<String>,
    pub top_n_customer: i64,
    pub is_cust_id_null: bool,
    pub is_aorl_null: bool,
    pub exclude_llg: Vec<String>,
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