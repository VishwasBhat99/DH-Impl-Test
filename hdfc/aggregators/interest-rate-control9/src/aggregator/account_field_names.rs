use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub acc_no: String,
    pub cust_id: String,
    pub pdt_code: String,
    pub cust_name: String,
    pub division: String,
    pub ccy: String,
    pub os_amt: String,
    pub value_dt: String,
    pub int_rt: String,
    pub mat_dt: String,
    pub acc_open_dt: String,
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
