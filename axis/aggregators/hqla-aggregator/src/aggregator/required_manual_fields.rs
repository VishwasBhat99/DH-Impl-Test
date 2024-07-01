use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReqManualFields {
    pub slr_gsec_maintained: String,
    pub slr_required: String,
    pub lending_to_nbfchfc: String,
    pub fallcr_ceiling: String,
}

impl ReqManualFields {
    pub fn new_from_path(_path: &str) -> ReqManualFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the required manual fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_man_fields: ReqManualFields = serde_json::from_str(&buf[..])
            .expect("Required manual fields file is not well-formatted");
        req_man_fields
    }
}
