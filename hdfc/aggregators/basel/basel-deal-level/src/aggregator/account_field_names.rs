use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub short_name: String,
    pub long_name: String,
    pub ccy: String,
    pub nativ_amt: String,
    pub cons_amt: String,
    pub isin: String,
    pub inv_type: String,
    pub portfolio_type: String,
    pub value_dt: String,
    pub maturity_dt: String,
    pub issuer_type: String,
    pub issuer_name: String,
    pub rating_agency_cd: String,
    pub rating_cd: String,
    pub resid_tenor: String,
    pub coupon_rate: String,
    pub add_class1: String,
    pub add_class2: String,
    pub add_class3: String,
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
