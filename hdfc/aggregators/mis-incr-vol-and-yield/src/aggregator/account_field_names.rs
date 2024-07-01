use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_no: String,
    pub acc_open_dt: String,
    pub ccy: String,
    pub prod_code: String,
    pub scheme_id: String,
    pub mis_1: String,
    pub mis_2: String,
    pub mis_3: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub concat: String,
    pub npa_flag: String,
    pub division: String,
    pub alm_line: String,
    pub ia_line: String,
    pub alco_map: String,
    pub psl_code: String,
    pub mat_dt: String,
    pub inr_rate: String,
    pub custom1: String,
    pub custom2: String,
    pub tot_amt: String,
    pub cashflows: String,
}

impl AccFieldNames {
    pub fn new_from_path(path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
