use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    pub files: Vec<Config>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WtVal {
    pub amt: Vec<String>,
    pub multiplier: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WtFields {
    pub field_name: String,
    pub wt_values: WtVal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub aggr_keys: Vec<String>,
    pub aggr_values: Vec<AmountFields>,
    pub wt_avg_fields: Vec<WtFields>,
    pub base_currency: String,
    pub currency_field: String,
    pub op_fields: Vec<AmountFields>,
    pub input_file_path: String,
    pub metadata_file_path: String,
    pub rule_file_path: Option<String>,
    pub default_llg_code: Option<String>,
    pub is_rules_applied: Option<bool>,
    pub negative_llgs: Option<Vec<String>>,
    pub abs_llgs:Option<Vec<String>>,
    pub display_zero_assigned_value:Option<bool>,
    pub default_ccy:Option<String>,
    pub default_country:Option<String>,
    pub delimiter:Option<char>
}

pub fn get_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read config file.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: Files =
        serde_json::from_str(&buf[..]).expect("Config json file was not well-formatted.");
    files_config
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct AmountFields {
    pub field_name: String,
    pub operator: Vec<String>,
}
