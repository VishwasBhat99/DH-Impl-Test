use crate::configuration_parameter::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, DataType, Reader};
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod derive_fields;
use derive_fields::derive_output;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let stg_non_sec_exposure_fns = match File::open(&config_params.stg_non_sec_exposure_fn()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.stg_non_sec_exposure_fn(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.stg_non_sec_exposure_fn(),
                error
            );
        }
    };

    let finnone_extracts = match File::open(&config_params.finnone_extract()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.finnone_extract(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.finnone_extract(),
                error
            );
        }
    };

    let finnone_npas = match File::open(&config_params.finnone_npa()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.finnone_npa(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.finnone_npa(),
                error
            );
        }
    };

    let stg_company_detailss = match File::open(&config_params.stg_company_details()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.stg_company_details(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.stg_company_details(),
                error
            );
        }
    };

    let restructured_mergeds = match File::open(&config_params.restructured_merged()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.restructured_merged(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.restructured_merged(),
                error
            );
        }
    };

    let fn_collaterals = match File::open(&config_params.fn_collateral()) {
        Ok(file) => file,
        Err(error) => {
            log_error!(
                logger,
                "Failed to open file `{}`: {}",
                &config_params.fn_collateral(),
                error
            );
            panic!(
                "Failed to open file `{}`: {}",
                &config_params.fn_collateral(),
                error
            );
        }
    };

    let stg_non_sec_exposure_fns_reader = BufReader::new(stg_non_sec_exposure_fns);
    let finnone_extracts_reader = BufReader::new(finnone_extracts);
    let finnone_npas_reader = BufReader::new(finnone_npas);
    let stg_company_detailss_reader = BufReader::new(stg_company_detailss);
    let restructured_mergeds_reader = BufReader::new(restructured_mergeds);
    let fn_collaterals_reader = BufReader::new(fn_collaterals);

    let mut pan_map: HashMap<String, String> = HashMap::new();
    let mut customer_classification_map: HashMap<String, String> = HashMap::new();
    let mut npa_map: HashMap<String, (String, String)> = HashMap::new();
    let mut product_description_map: HashMap<String, String> = HashMap::new();
    let mut collateral_map: HashMap<String, String> = HashMap::new();

    let mut restructured_set: HashSet<String> = HashSet::new();

    for line in stg_company_detailss_reader.lines() {
        let line = match line {
            Ok(data) => data,
            Err(error) => {
                log_error!(
                    logger,
                    "Error reading lines from stg_company_details file: {}",
                    error
                );
                continue;
            }
        };
        let line_split: Vec<&str> = line.split("~").collect();
        customer_classification_map.insert(line_split[20].to_string(), line_split[14].to_string());
    }

    for line in finnone_npas_reader.lines() {
        let line = line.expect("Not reading lines from finnone_npas_reader file");
        let line_split: Vec<&str> = line.split("|").collect();
        npa_map.insert(
            line_split[3].to_string(),
            (line_split[4].to_string(), line_split[5].to_string()),
        );
    }

    for line in fn_collaterals_reader.lines() {
        let line = line.expect("Not reading lines from fn_collaterals_reader");
        let line_split: Vec<&str> = line.split("|").collect();
        collateral_map.insert(line_split[1].to_string(), line_split[6].to_string());
    }
    for line in restructured_mergeds_reader.lines() {
        let line = line.expect("Not reading lines from restructured_mergeds file");
        let line_split: Vec<&str> = line.split("~|").collect();
        restructured_set.insert(line_split[1].to_string());
    }

    for line in finnone_extracts_reader.lines() {
        let record_str = match line {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        let fields: Vec<&str> = record_str.split('|').collect();
        pan_map.insert(fields[2].to_string(), fields[20].to_string());
    }

    let mut master_file_path = open_workbook_auto(&config_params.finnone_master())
        .expect("Unable to open the repricing master xlsx file.");
    log_info!(
        logger,
        "Sheets present in Master-File: `{:?}`",
        master_file_path.sheet_names()
    );
    if !master_file_path
        .sheet_names()
        .contains(&config_params.rep_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Master-File: `{}`",
            config_params.rep_sheet_name(),
            config_params.finnone_extract()
        );
    }
    log_info!(
        logger,
        "Reading Sheet: `{}` from Master-File",
        config_params.rep_sheet_name(),
    );
    if let Some(Ok(master_file_reader)) =
        master_file_path.worksheet_range(config_params.rep_sheet_name())
    {
        for (_row_no, row) in master_file_reader.rows().enumerate() {
            let acct_id = get_str_from_xlsx(row, 0, logger);
            let balm_l2 = get_str_from_xlsx(row, 6, logger);
            product_description_map.insert(acct_id, balm_l2);
        }
    } else {
        println!(
            "No worksheet {} found in the Excel of file. name Finnone_master ",
            config_params.rep_sheet_name()
        );
    }
    derive_output(
        stg_non_sec_exposure_fns_reader,
        &pan_map,
        &customer_classification_map,
        &npa_map,
        &product_description_map,
        &collateral_map,
        &restructured_set,
        &config_params,
        logger,
    );
}

pub fn get_str_from_xlsx(data: &[DataType], index: usize, logger: &Logger) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            log_error!(
                logger,
                "Could not get data at column-no: `{}` for row: `{:?}`",
                index + 1,
                data
            );
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
