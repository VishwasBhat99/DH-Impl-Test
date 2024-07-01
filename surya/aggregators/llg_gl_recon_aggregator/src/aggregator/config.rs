use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub input_file_path: String,
    pub req_fields_file_path: String,
    pub metadata_file_path: String,
    pub rules_file_path: String,
    pub default_llg_code: i32,
    pub default_gl_code: String,
    pub is_consolidated: bool,
    pub source_name: String,
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
impl File {
    pub fn input_file_path(&self) -> &str {
        &self.input_file_path
    }
    pub fn req_fields_file_path(&self) -> &str {
        &self.req_fields_file_path
    }
    pub fn metadata_file_path(&self) -> &str {
        &self.metadata_file_path
    }
    pub fn rules_file_path(&self) -> &str {
        &self.rules_file_path
    }
    pub fn default_llg_code(&self) -> i32 {
        self.default_llg_code
    }
    pub fn default_gl_code(&self) -> &str {
        &self.default_gl_code
    }
    pub fn is_consolidated(&self) -> bool {
        self.is_consolidated
    }
    pub fn source_name(&self) -> &str {
        &self.source_name
    }
}
