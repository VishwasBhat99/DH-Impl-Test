use calamine::DataType;
use chrono::NaiveDate;

use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Default)]
pub struct RepData {
    pub account_id: String,
    pub next_repriced_date: NaiveDate,
}
impl RepData {
    pub fn new_from_xlsx(data: &[DataType], config_params: &ConfigurationParameters) -> RepData {
        let rep_date_str = get_str_from_xlsx(data, 3);
        RepData {
            account_id: get_str_from_xlsx(data, 0),
            next_repriced_date: get_date_from_string(rep_date_str, config_params),
        }
    }
}

pub fn get_str_from_xlsx(data: &[DataType], index: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            )
        })
        .to_string()
        .replace("\n", " ")
        .trim()
        .to_string()
}
pub fn get_date_from_string(data: String, config_params: &ConfigurationParameters) -> NaiveDate {
    let next_rep_date =
        rbdate::datevalue_to_naive_date(&data).unwrap_or(*config_params.as_on_date());
    return next_rep_date;
}
