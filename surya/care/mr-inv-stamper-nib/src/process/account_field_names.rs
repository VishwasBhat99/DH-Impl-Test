use sdb_io;
use serde_json;
use std::io::Read;
#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub deal_id: String,
    pub src_sys_code: String,
    pub llg_id: String,
    pub isin: String,
    pub inv_type: String,
    pub inv_desc: String,
    pub category: String,
    pub face_val_hcy: String,
    pub book_val_hcy: String,
    pub market_val_hcy: String,
    pub ccy_id: String,
    pub exch_rate: String,
    pub gl_cd_1: String,
    pub gl_cd_2: String,
    pub gl_cd_3: String,
    pub issuer_id: String,
    pub issuer_name: String,
    pub issuer_type: String,
    pub sub_date: String,
    pub val_or_settle_date: String,
    pub mat_date: String,
    pub res_days: String,
    pub coup_rate: String,
    pub coup_freq_code: String,
    pub coup_basis: String,
    pub guranteed_by: String,
    pub applied_rating_id: String,
    pub internal_rating: String,
    pub external_rating_agenecy: String,
    pub external_rating: String,
    pub inst_purpose: String,
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
