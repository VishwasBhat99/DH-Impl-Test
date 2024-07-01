use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_id: String,
    pub agency: String,
    pub agency_rating: String,
    pub cashflows: String,
    pub currency: String,
    pub next_rep_date: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the required fields File");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Required fields json file was not well-formatted");
        req_fields
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultValues {
    pub default_llg_code: i32,
    pub default_spread: String,
    pub default_bucket: i64,
    pub default_agency_name: String,
    pub default_agency_rating: String,
    pub default_balm_rating: String,
    pub base_ccy: String,
    pub consol_ccy: String,
    pub default_overdue_llg_code: Option<i32>,
}

impl DefaultValues {
    pub fn new_from_path(_path: &str) -> DefaultValues {
        let mut file = sdb_io::open_file_read(_path).expect("Cannot open the default values file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let def_values: DefaultValues = serde_json::from_str(&buf[..])
            .expect("Default values json file was not well-formatted");
        def_values
    }
}
