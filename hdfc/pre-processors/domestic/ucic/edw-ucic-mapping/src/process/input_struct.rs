use chrono::NaiveDate;

use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Default, Clone)]

pub struct InputData {
    pub cod_cust_id: String,
    pub ucic: String,
    pub dat_cust_open: NaiveDate,
    pub dat_last_mnt: NaiveDate,
    pub required_data: String,
}

impl InputData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> InputData {
        InputData {
            cod_cust_id: get_str(input_file, input_acc, 0, row),
            ucic: get_str(input_file, input_acc, 6, row),
            dat_cust_open: get_date(config_params, input_file, input_acc, 9, row),
            dat_last_mnt: get_date(config_params, input_file, input_acc, 10, row),
            required_data: OtherFields(input_file, input_acc, row),
        }
    }
}
pub fn OtherFields(input_file: &str, data: &[&str], row: usize) -> String {
    let nature_of_bus = get_str(input_file, data, 1, row);
    let txt_bus_desc = get_str(input_file, data, 2, row);
    let flg_cust_typ = get_str(input_file, data, 3, row);
    let txt_cust_typ = get_str(input_file, data, 4, row);
    let pan = get_str(input_file, data, 5, row);
    let nam_cust_full = get_str(input_file, data, 7, row);
    let flag_blocked = get_str(input_file, data, 8, row);
    let concat = format!(
        "{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}",
        nature_of_bus, txt_bus_desc, flg_cust_typ, txt_cust_typ, pan, nam_cust_full, flag_blocked
    );
    return concat;
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
    let date_parser = rbdate::DateParser::new("%d-%m-%Y %H:%M:%S".to_string(), false);
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
