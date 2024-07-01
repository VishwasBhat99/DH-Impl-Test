use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Default, Clone)]

pub struct CustData {
    pub flag_value: String,
    pub condition: String,
    pub txt_desc_flag: String,
}

impl CustData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize,
    ) -> CustData {
        CustData {
            flag_value: get_str(input_file, data, 0, row),
            condition: get_str(input_file, data, 1, row),
            txt_desc_flag: get_str(input_file, data, 2, row),
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
