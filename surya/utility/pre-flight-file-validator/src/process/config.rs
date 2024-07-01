use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileDetails {
    pub input_file_path: String,
    pub health_check_report_path: String,
    pub file_type: String,
    pub field_separator: Option<String>,
    pub excel_sheet_name: Option<String>,
    pub duplication_check_keys: Option<Vec<i64>>,
    pub header_count: Option<i64>,
    pub footer_count: Option<i64>,
    pub expected_column_count: Option<i64>,
    pub exclude_char_in_footer: Option<Vec<i64>>,
    pub amount_col_no: Option<i64>,
    pub key_col_no: Option<Vec<i64>>,
    pub data_check: Option<DataCheck>,
    pub date_fields_formats: Option<Vec<DateCheck>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DataCheck {
    pub values_in: Option<Vec<ColDetails>>,
    pub values_not_in: Option<Vec<ColDetails>>,
    pub data_type: Option<Vec<ColDetails>>,
}
impl DataCheck {
    pub fn def() -> DataCheck {
        DataCheck {
            values_in: Some(Vec::new()),
            values_not_in: Some(Vec::new()),
            data_type: Some(Vec::new()),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ColDetails {
    pub col_no: Vec<i64>,
    pub col_values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DateCheck {
    pub field_column: String,
    pub date_format: String,
}

pub fn get_files(path: &str) -> FileDetails {
    let mut file = sdb_io::open_file_read(path).expect("Cannot read files config.");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Cannot read input as string");
    let files_config: FileDetails =
        serde_json::from_str(&buf[..]).expect("Config json file was not well-formatted.");
    files_config
}
