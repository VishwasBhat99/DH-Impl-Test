pub mod alm_master;
pub mod cust_master;
pub mod dyn_bucket;
pub mod extra_fields;
pub mod input_account;
pub mod loan_additional;
pub mod npa;

use calamine::DataType;
use rbdate::{datevalue_to_naive_date, NaiveDate};

fn get_data(data: &str) -> String {
    if data.is_empty() {
        String::from("NA")
    } else {
        String::from(data)
    }
}

fn get_master_data(data: &str) -> String {
    if data.is_empty() {
        String::from("NONE")
    } else {
        String::from(data)
    }
}

fn get_date(data: &DataType) -> String {
    let date = data.to_string().replace("\u{a0}", " ");
    if NaiveDate::parse_from_str(&date, "%d-%m-%Y").is_err() && !data.is_empty() {
        return datevalue_to_naive_date(&date)
            .expect("Cannot convert date value to date")
            .to_string();
    } else {
        match NaiveDate::parse_from_str(&date, "%d-%m-%Y") {
            Ok(date) => date.to_string(),
            Err(_) => "".to_string(),
        }
    }
}
