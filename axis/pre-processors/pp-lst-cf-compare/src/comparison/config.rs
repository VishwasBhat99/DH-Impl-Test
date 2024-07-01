use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFields {
    pub keys: Vec<AccFields>,
    pub comparison_fields: Vec<AccFields>,
    pub cashflow: CashflowFields,
    pub decimal_places: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFields {
    pub cf_field_name: String,
    pub lst_field_name: String,
    pub data_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CashflowFields {
    pub cf_field_name: String,
    pub lst_principal_field: String,
    pub lst_interest_field: String,
    pub lst_date_field: String,
}

pub fn get_config_fields(path: &str) -> ConfigFields {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read Config file.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: ConfigFields =
        serde_json::from_str(&buf[..]).expect("Config json file was not well-formatted.");
    files_config
}
