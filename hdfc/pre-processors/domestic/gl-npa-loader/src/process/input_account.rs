use crate::configuration_parameters::ConfigurationParameters;
use calamine::DataType;
use rbdate::NaiveDate;

#[derive(Debug, Clone, Default)]
pub struct NpaData {
    pub src_system:String,
    pub accref_num:String,
    pub spec_prov:f64,
    pub tot_prov:f64,
    pub net_npa:f64,
    pub npa_date:String,
    pub asst_class:String

}

impl NpaData {
    pub fn new(config_params: &ConfigurationParameters,
        input_file: &str,
        data: &[&str],
        row: usize) -> NpaData {
        NpaData {
            src_system: get_str(input_file, data, 0, row),
            accref_num: get_str(input_file, data, 1, row),
            spec_prov: get_str(input_file, data, 2, row).parse::<f64>().unwrap_or(0.0),
            tot_prov: get_str(input_file, data, 3, row).parse::<f64>().unwrap_or(0.0),
            net_npa: get_str(input_file, data, 4, row).parse::<f64>().unwrap_or(0.0),
            npa_date:get_str(input_file, data, 5, row),
            asst_class: get_str(input_file, data, 6, row)
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
pub fn get_date(
    config_params: &ConfigurationParameters,
    input_file: &str,
    data: &[&str],
    index: usize,
    row: usize,
) -> NaiveDate {
    let date_parser = rbdate::DateParser::new("%d-%m-%Y".to_string(), false);
    date_parser
        .parse_opt(
            &data
                .get(index)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                        index + 1,
                        row,
                        input_file,
                    )
                })
                .replace('.', ""),
        )
        .unwrap_or(*config_params.as_on_date())
}

