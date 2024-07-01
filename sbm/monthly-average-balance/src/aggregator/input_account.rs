use rbdate::NaiveDate;
use statics::*;

use crate::configuration_parameters::{self, ConfigurationParameters};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InputAccount {
    pub acc_no: String,
    pub amt: String,
    pub int_rt: String,
    pub end_eod_date: String,
    pub eod_date: String,
    pub acct_crncy_code: String,
}

#[derive(Debug, Default)]

pub struct ExchangeRateData {
    pub rtlist_num: String,
    pub rtlist_date: NaiveDate,
    pub fxd_crncy_code: String,
    pub var_crncy_code: String,
    pub rate_code: String,
    pub fxd_crncy_unit: f64,
}

#[derive(Debug)]
pub struct InputParsedAccount {
    pub acc_no: String,
    pub amt: f64,
    pub int_rt: f64,
    pub end_eod_date: NaiveDate,
    pub eod_date: NaiveDate,
    pub acct_crncy_code: String,
}

impl InputAccount {
    pub fn parse(&self) -> InputParsedAccount {
        InputParsedAccount {
            acc_no: self.acc_no.to_string(),
            amt: self.amt.parse().unwrap_or(DEFAULT_FLOAT),
            int_rt: self.int_rt.parse().unwrap_or(DEFAULT_FLOAT),
            end_eod_date: NaiveDate::parse_from_str(&self.end_eod_date, "%d-%m-%Y")
                .expect("Error getting `end_eod_date` while parsing input record."),
            eod_date: NaiveDate::parse_from_str(&self.eod_date, "%d-%m-%Y")
                .expect("Error getting `eod_date` while parsing input record."),
            acct_crncy_code: self.acct_crncy_code.to_string(),
        }
    }
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
        .unwrap_or(config_params.as_on_date())
}
