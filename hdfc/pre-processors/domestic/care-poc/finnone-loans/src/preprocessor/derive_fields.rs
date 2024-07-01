extern crate chrono;

use rbdate::{num_days_start_to_end, NaiveDate};
use slog::Logger;
use std::io::{self, *};
use std::{collections::HashMap, collections::HashSet, fs::File, io::BufRead};

use crate::configuration_parameter::ConfigurationParameters;
use crate::macros;

use super::output_account::{get_writer, OutputData};

pub fn no_of_days(start_date: NaiveDate, end_date: NaiveDate) -> i64 {
    num_days_start_to_end(start_date, end_date)
}

fn convert_date_format(input_date: &str, logger: &Logger) -> NaiveDate {
    match NaiveDate::parse_from_str(input_date, "%d/%m/%Y") {
        Ok(date) => date,
        Err(error) => {
            log_error!(logger, "Failed to parse date `{}`: {}", input_date, error);
            panic!("Failed to parse date `{}`: {}", input_date, error);
        }
    }
}

pub fn derive_output(
    stg_non_sec_exposure_fns_reader: io::BufReader<File>,
    pan_map: &HashMap<String, String>,
    customer_classification_map: &HashMap<String, String>,
    npa_map: &HashMap<String, (String, String)>,
    product_description_map: &HashMap<String, String>,
    collateral_map: &HashMap<String, String>,
    restructured_set: &HashSet<String>,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) {
    let def_val = "NA".to_string();
    let def_num_value = "0".to_string();

    let def_tuple = ("NA".to_string(), "0".to_string());
    let mut op_writer = get_writer(config_params.output_file_path());
    for (_index, line) in stg_non_sec_exposure_fns_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split("~|").collect();

        // Extract PAN lookup value by removing "FN" prefix
        let pan_lookup = &input_fields[39][2..]; // Remove the first two characters "FN"

        // Retrieve PAN value from pan_map, with a default value def_val if the key is not found
        let pan_value = pan_map.get(pan_lookup).unwrap_or(&def_val);

        let customer_code = customer_classification_map
            .get(pan_value)
            .unwrap_or(&def_val);
        let npa_value = npa_map.get(&input_fields[39][2..]).unwrap_or(&def_tuple);
        let npa_val = npa_value.1.parse::<f64>().unwrap_or(0.0);
        let provision_percentage_value = if npa_val == 0.0 {
            0.0
        } else {
            input_fields[10].parse::<f64>().unwrap_or(0.0) / npa_val
        };

        let result_res = if restructured_set.contains(&input_fields[39][2..]) {
            "Y"
        } else {
            "N"
        };

        let product_code_value = if product_description_map.contains_key(input_fields[24]) {
            input_fields[24]
        } else {
            &def_val
        };
        let product_description_value = product_description_map
            .get(input_fields[24])
            .unwrap_or(&def_val);
        let collateral_value = collateral_map
            .get(input_fields[39])
            .unwrap_or(&def_num_value);
        let maturity_date = convert_date_format(&input_fields[30], logger);
        let acc_open_date = convert_date_format(&input_fields[27], logger);
        let maturity_tenor = no_of_days(acc_open_date.clone(), maturity_date);

        let as_on_date = *config_params.as_on_date();
        let residual_tenor = no_of_days(as_on_date, maturity_date);

        let input_7 = convert_date_format(&input_fields[30], logger);
        let input_15 = convert_date_format(&input_fields[29], logger);

        let op = OutputData {
            account_id: input_fields[39].to_string(),
            customer_id: input_fields[20].to_string(),
            group_id: input_fields[38].to_string(),
            outstanding_amount: input_fields[10].to_string(),
            outstanding_amount_lcy: input_fields[10].to_string(),
            ccy: input_fields[19].to_string(),
            maturity_date: input_7.format("%d-%m-%Y").to_string(),
            gl_code: "".to_string(),
            pan_number: pan_value.to_string(),
            customer_classification_code: customer_code.to_string(),
            npa: npa_value.0.clone(),
            provision_amount: npa_value.1.clone(),
            provision_percentage: provision_percentage_value.to_string(),
            restructured_flag: result_res.to_string(),
            sanction_date: input_15.format("%d-%m-%Y").to_string(),
            product_code: product_code_value.to_string(),
            product_description: product_description_value.to_string(),
            ltv: "".to_string(),
            residential_mortgage_flag: "".to_string(),
            sub_sector: "".to_string(),
            group_level_total_exposure: "".to_string(),
            rating_agency: "".to_string(),
            rating: "".to_string(),
            bank_category: "".to_string(),
            cet_ratio: "".to_string(),
            guaranteed_by: "".to_string(),
            collateral: collateral_value.to_string(),
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            residual_tenor: residual_tenor.to_string(),
            maturity_tenor: maturity_tenor.to_string(),
        };
        let _result = writeln!(op_writer, "{}", op.format_with_separator());
    }
}
