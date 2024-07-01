use std::collections::{HashMap, HashSet};

use crate::configuration_parameters::{self, ConfigurationParameters};
use rbdate::NaiveDate;

use super::config::BillsFields;

#[derive(Debug, Clone, Default)]
pub struct BillsFile {
    pub bills_id: String,
    pub bills_amt_inr: f64,
    pub acct_crncy_code: String,
}

impl BillsFile {
    pub fn new(
        bills_file_details: BillsFields,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> BillsFile {
        let acct_id_index = bills_file_details.acct_id_index as usize;
        let amt_index = bills_file_details.amt_index as usize;
        let acct_ccy_index = bills_file_details.ccy_code_index as usize;
        BillsFile {
            bills_id: get_str(input_file, input_acc, acct_id_index - 1, row),
            bills_amt_inr: get_str(input_file, input_acc, amt_index - 1, row)
                .parse::<f64>()
                .expect(&format!(
                    "Error getting amt in input file :{} for input acct: {:?}",
                    input_file, input_acc
                )),
            acct_crncy_code: get_str(input_file, input_acc, acct_ccy_index - 1, row),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExchangeRateData {
    pub rtlist_num: String,
    pub rtlist_date: NaiveDate,
    pub fxd_crncy_code: String,
    pub var_crncy_code: String,
    pub rate_code: String,
    pub fxd_crncy_unit: f64,
}

impl ExchangeRateData {
    pub fn new(
        config_params: &ConfigurationParameters,
        input_file: &str,
        input_acc: &[&str],
        row: usize,
    ) -> ExchangeRateData {
        ExchangeRateData {
            rtlist_num: get_str(input_file, input_acc, 0, row),
            rtlist_date: get_date(config_params, input_file, input_acc, 1, row),
            fxd_crncy_code: get_str(input_file, input_acc, 2, row),
            var_crncy_code: get_str(input_file, input_acc, 3, row),
            rate_code: get_str(input_file, input_acc, 4, row),
            fxd_crncy_unit: get_str(input_file, input_acc, 7, row)
                .parse::<f64>()
                .unwrap_or(1.0),
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
    let date_parser = rbdate::DateParser::new("%d-%m-%y".to_string(), false);
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

pub fn get_nearest_date(set: &HashSet<NaiveDate>, target_date: NaiveDate) -> NaiveDate {
    if set.contains(&target_date) {
        target_date
    } else {
        let mut smaller: Option<NaiveDate> = None;
        let mut larger: Option<NaiveDate> = None;

        for &date in set.iter() {
            if date < target_date {
                smaller = Some(date);
            } else if date > target_date {
                larger = Some(date);
                break;
            }
        }

        match (smaller, larger) {
            (Some(s), Some(l)) if (target_date - s) <= (l - target_date) => s,
            (_, Some(l)) => l,
            (Some(s), _) => s,
            _ => target_date, // Return the target date if no nearest date found
        }
    }
}
