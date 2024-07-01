use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub coa_master_file: String,
    pub finnone_prod_to_div_mapping_file: String,
    pub lnm_alternate_accs_file: String,
    pub npa_prev_month_file: String,
    pub npa_prev_month_sli_file: String,
    pub npa_as_on_month_file: String,
    pub npa_as_on_month_sli_file: String,
    pub npa_prev_year_file: String,
    pub npa_prev_year_sli_file: String,
    pub write_off_prev_month_file: String,
    pub write_off_as_on_month_file: String,
    pub write_off_prev_year_file: String,
    pub division_mapping_file: String,
    pub sheet_names_file: String,
    pub stamper_files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub source: String,
    pub stamper_file_path: String,
}

pub fn get_files(path: &str) -> Files {
    let mut file = sdb_io::open_file_read(path)
        .unwrap_or_else(|_| panic!("Couldn't Open Config-JSON File: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Error reading config-file as string");
    let files_config: Files = serde_json::from_str(&buf[..])
        .unwrap_or_else(|_| panic!("Config JSON File: `{}` was not well-formatted.", path));
    files_config
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SheetNames {
    pub coa_master: Option<String>,
    pub finnone_prod_to_div_mapping: Option<String>,
    pub lnm_alternate_accs: Option<String>,
    pub npa_prev_month: Option<String>,
    pub npa_prev_month_sli: Option<String>,
    pub npa_as_on_month: Option<String>,
    pub npa_as_on_month_sli: Option<String>,
    pub npa_prev_year: Option<String>,
    pub npa_prev_year_sli: Option<String>,
    pub write_off_prev_month: Option<String>,
    pub write_off_as_on_month: Option<String>,
    pub write_off_prev_year: Option<String>,
    pub division_mapping: Option<String>,
}

impl SheetNames {
    pub fn new_from_path(_path: &str) -> SheetNames {
        let mut file = sdb_io::open_file_read(_path)
            .unwrap_or_else(|_| panic!("Couldn't Open Sheet-Names-JSON File: {}", _path));
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Error reading sheet-names-file as string");
        let sheet_names: SheetNames = serde_json::from_str(&buf[..]).unwrap_or_else(|_| {
            panic!("Sheet-Names JSON File: `{}` was not well-formatted.", _path)
        });
        sheet_names
    }
}
