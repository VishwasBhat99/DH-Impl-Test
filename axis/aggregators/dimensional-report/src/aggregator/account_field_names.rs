use sdb_io;
use serde_json;
use std::{collections::HashMap, fs, io::Read};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccFieldNames {
    pub amt_column_no_start_203: String,
    pub dimid1: String,
    pub dimid2: String,
    pub dimid3: String,
    pub dimid4: String,
    pub dimid5: String,
    pub amt_column: String,
    pub cashflows: String,
    pub ccy_column: String,
    pub country: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}

pub fn metadata_reader(_path: &str) -> HashMap<String, i32> {
    let mut metadata_map: HashMap<String, i32> = HashMap::new();
    // let mut field_names = Vec::new();
    let metadata_reader = fs::read_to_string(_path).expect("Failed to read metadata file!");
    let mut line_count = 1;
    for line in metadata_reader.lines() {
        if line.contains("name") {
            let fields: Vec<&str> = line.split(':').collect();
            let mut name = fields[1].to_string();
            name.pop();
            name.pop();
            name = name[2..].to_string();
            metadata_map.insert(name, line_count);
            line_count += 1;
        }
    }
    metadata_map
}
