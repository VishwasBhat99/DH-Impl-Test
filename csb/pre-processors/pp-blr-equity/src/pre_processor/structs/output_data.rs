use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use pre_processor::structs::account::Account;

#[derive(Debug)]
pub struct OutputData {
    pub name: String,
    pub desc: String,
    pub face_value: String,
    pub op_date: NaiveDate,
    pub op_bal: f64,
    pub high_bal: f64,
    pub high_date: NaiveDate,
    pub low_bal: f64,
    pub low_date: NaiveDate,
    pub close_date: NaiveDate,
    pub close_bal: f64,
    pub std_dev: f64,
}

impl OutputData {
    pub fn new(
        config_param: &ConfigurationParameters,
        first_acount: &Account,
        last_acount: &Account,
        symbol_name: &str,
        series_name: &str,
    ) -> OutputData {
        OutputData {
            name: symbol_name.to_string(),
            desc: series_name.to_string(),
            face_value: config_param.face_value().to_string(),
            op_date: first_acount.date,
            op_bal: first_acount.open_price,
            high_bal: first_acount.high_price,
            high_date: first_acount.date,
            low_bal: first_acount.low_price,
            low_date: first_acount.date,
            close_date: last_acount.date,
            close_bal: last_acount.close_price,
            std_dev: 0.0,
        }
    }
}
