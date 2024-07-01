use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub ccy: String,
    pub dim_1: String,
    pub dim_2: String,
    pub dim_3: String,
    pub dim_4: String,
    pub dim_5: String,
    pub dim_6: String,
    pub dim_7: String,
    pub dim_8: String,
    pub dim_9: String,
    pub dim_10: String,
    pub dim_11: String,
    pub dim_12: String,
    pub dim_13: String,
    pub dim_14: String,
    pub dim_15: String,
    pub acc_strt_dt: String,
    pub mat_dt: String,
    pub next_rep_dt: String,
    pub tot_amt: String,
    pub cashflows: String,
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
