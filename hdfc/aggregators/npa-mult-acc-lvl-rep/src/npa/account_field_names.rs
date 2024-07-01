use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_no: String,
    pub ccy: String,
    pub prod_code: String,
    pub scheme_id: String,
    pub mis1: String,
    pub mis2: String,
    pub mis3: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub concat: String,
    pub npa_flag: String,
    pub div: String,
    pub alm_line: String,
    pub ia_line: String,
    pub psl_code: String,
    pub amt_as_per_src: String,
    pub yield_rate: String,
    pub open_date: String,
    pub mat_date: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account required fields file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account required fields json file was not well-formatted");
        req_fields
    }
}
