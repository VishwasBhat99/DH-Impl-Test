use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReqFields {
    pub scheme_id: usize,
    pub from_tenure: usize,
    pub mc_status: usize,
    pub rate_pct: usize,
    pub to_tenure: usize,
    pub tenure_in: usize,
    pub stream_desc: usize,
}

impl ReqFields {
    pub fn new_from_path(_path: &str) -> ReqFields {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: ReqFields = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
