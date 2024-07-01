use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub col_id: String,
    pub exp_acc_no: String,
    pub cust_no: String,
    pub col_type: String,
    pub tot_col_val_lcy: String,
    pub col_ccy: String,
    pub current_col_val_lcy: String,
    pub mat_date_col: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path)
            .expect("Cannot open the account req fields metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}
