use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_no: String,
    pub cust_id: String,
    pub cust_name: String,
    pub pdt_code: String,
    pub scheme_id: String,
    pub booking_dt: String,
    pub value_dt: String,
    pub mat_dt: String,
    pub mis1: String,
    pub mis2: String,
    pub mis3: String,
    pub ccy: String,
    pub org_amt: String,
    pub cur_os_amt: String,
    pub benchmark: String,
    pub rt_type: String,
    pub bm_rate: String,
    pub int_rt: String,
    pub alm_line: String,
    pub ia_line: String,
    pub concat: String,
    pub division: String,
    pub npa_type: String,
    pub raw_bm: String,
    pub final_bm: String,
    pub cashflows: String,
    pub rate_flag: String,
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
