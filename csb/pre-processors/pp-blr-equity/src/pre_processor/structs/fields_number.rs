use super::super::macros;
use chrono::NaiveDate;
use serde::Serialize;
use slog::Logger;
use std::env::current_dir;
use std::fs::File;
use std::io::Error;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct FieldsConfig {
    #[serde(rename(serialize = "Fields", deserialize = "Fields"))]
    pub field: Field,
    #[serde(rename(serialize = "Std_dev", deserialize = "Std_dev"))]
    pub std_dev: StdDev,
    #[serde(rename(serialize = "Symbol Name", deserialize = "Symbol Name"))]
    pub symbol_name: String,
    #[serde(rename(serialize = "Filter on Symbol", deserialize = "Filter on Symbol"))]
    pub filter_on_symbol: bool,
    #[serde(rename(serialize = "Series Name", deserialize = "Series Name"))]
    pub series_name: String,
    #[serde(rename(serialize = "delimiter", deserialize = "delimiter"))]
    pub delimiter: String,
}
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Field {
    #[serde(rename(serialize = "Date", deserialize = "Date"))]
    pub date: i64,
    #[serde(rename(serialize = "Symbol", deserialize = "Symbol"))]
    pub symbol: i64,
    #[serde(rename(serialize = "Series", deserialize = "Series"))]
    pub series: i64,
    #[serde(rename(serialize = "Open Price", deserialize = "Open Price"))]
    pub open_price: i64,
    #[serde(rename(serialize = "High Price", deserialize = "High Price"))]
    pub high_price: i64,
    #[serde(rename(serialize = "Low Price", deserialize = "Low Price"))]
    pub low_price: i64,
    #[serde(rename(serialize = "Last Traded Price", deserialize = "Last Traded Price"))]
    pub last_traded_price: i64,
    #[serde(rename(serialize = "Close Price", deserialize = "Close Price"))]
    pub close_price: i64,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct StdDev {
    pub use_sample: bool,
}

pub fn reader_json(file_path: &str, log: &Logger) -> FieldsConfig {
    let fields_value = match File::open(file_path) {
        Ok(v) => match serde_json::from_reader(v) {
            Ok(json) => {
                let fields_config: FieldsConfig =
                    serde_json::from_value(json).expect("unable to get fields, check json format");
                fields_config
            }
            Err(er) => {
                log_error!(log, "error: {:?}", er);
                panic!("file: {}, json format not valid", file_path);
            }
        },
        Err(er) => {
            log_error!(log, "error: {:?}", er);
            panic!(
                "Could not found file `{}` on location `{}` : {}.",
                file_path,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                er
            )
        }
    };
    fields_value
}
