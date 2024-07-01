use sdb_io;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub cust_id: String,
    pub out_bal: Option<String>,
    pub limit_bal: Option<String>,
    pub currency: String,
    pub is_consolidated: Option<bool>,
    pub cashflows: Option<String>,
}

pub fn get_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path)
        .unwrap_or_else(|_| panic!("Error reading Config-Json File: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read Config Json Data as String");
    let files_config: Files =
        serde_json::from_str(&buf[..]).expect("Config Json File is not Well-Formatted.");
    files_config
}
