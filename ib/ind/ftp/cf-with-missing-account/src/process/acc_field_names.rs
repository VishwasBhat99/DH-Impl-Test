use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReqFieldNames {
    pub lookup_value: String,
    pub outstanding_amount: String,
}

impl ReqFieldNames {
    pub fn new_from_path(_path: &str) -> ReqFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the required fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: ReqFieldNames =
            serde_json::from_str(&buf[..]).expect("Required fields file was not well-formatted");
        req_fields
    }
}
