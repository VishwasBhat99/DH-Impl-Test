use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccField {
    pub fields: Vec<FieldsName>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FieldsName {
    pub name: String,
    pub r#type: String,
}

impl AccField {
    pub fn new_from_path(_path: &str) -> AccField {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccField = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
