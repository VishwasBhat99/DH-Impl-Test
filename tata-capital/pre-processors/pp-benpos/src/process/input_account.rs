use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct NcdCpData {
    pub isin_id: String,
    pub first_holder_name: String,
    pub first_holder_pan: String,
    pub category: String,
    pub amount: f64,
    pub mat_date: NaiveDate,
}

impl NcdCpData {
    pub fn new_from_excel(config_params: &ConfigurationParameters, data: &[DataType]) -> NcdCpData {
        let mat_date = get_str_from_xlsx(data, 11);
        NcdCpData {
            isin_id: get_str_from_xlsx(data, 3),
            first_holder_name: get_str_from_xlsx(data, 5),
            first_holder_pan: get_str_from_xlsx(data, 6),
            category: get_str_from_xlsx(data, 7),
            amount: get_str_from_xlsx(data, 9).parse::<f64>().unwrap_or(0.0),
            mat_date: get_date_from_string(mat_date, config_params),
        }
    }
}
pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
pub fn get_date_from_excel(
    config_params: &ConfigurationParameters,
    data: &[DataType],
    index: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| panic!("Could not get data at column-no: `{}` ", index + 1,))
                .to_string()
                .replace(".0000000", ""),
        )
        .unwrap_or(*config_params.as_on_date())
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
