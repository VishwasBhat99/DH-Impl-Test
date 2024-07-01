use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReqFields {
    pub isin_detail: usize,
    pub isin_type: usize,
    pub face_amt: usize,
    pub issue_date: usize,
    pub outstd_amt: usize,
    pub mat_date: usize,
    pub coupons: usize,
    pub isin_price: usize,
    pub open_price: usize,
    pub high_price: usize,
    pub low_price: usize,
    pub close_price: usize,
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
