use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct AccountData {
    pub code: String,
    pub desc: String,
    pub amount: f64,
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
