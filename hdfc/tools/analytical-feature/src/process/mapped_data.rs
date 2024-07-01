use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_cust_id_to_cust_type(
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> HashMap<String, String> {
    let file = File::open(&config_params.edw_alm_customer_file())
        .expect("Could Not Read edw-alm-customer File");

    let reader = BufReader::new(file);

    let mut cust_id_map_cust_type: HashMap<String, String> = HashMap::new();

    for (index, line) in reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read").to_string();
        let fields: Vec<&str> = line.split("~#~").collect();
        if fields.len() >= 2 {
            cust_id_map_cust_type.insert(fields[0].to_string(), fields[1].to_string());
        } else {
            log_error!(
                logger,
                "fields are less in the line-no.{} in edw-alm-customer File. line is '{}'.",
                (index + 1).to_string(),
                line
            );
        }
    }

    cust_id_map_cust_type
}

pub fn get_cust_id_to_division(
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> HashMap<String, String> {
    let file = File::open(&config_params.biu_file()).expect("Could Not Read biu File");

    let reader = BufReader::new(file);

    let mut cust_id_map_division: HashMap<String, String> = HashMap::new();

    for (index, line) in reader.lines().enumerate().skip(1) {
        let line = line.expect("Could Not Read").to_string();
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() >= 6 {
            cust_id_map_division.insert(fields[0].to_string(), fields[5].to_string());
        } else {
            log_error!(
                logger,
                "fields are less in the line-no.{} in biu File. line is '{}'.",
                (index + 1).to_string(),
                line
            );
        }
    }

    cust_id_map_division
}

pub fn get_lcr_classification_to_lcr_run_off(
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> HashMap<String, String> {
    let file = File::open(&config_params.runoff_file()).expect("Could Not Read runoff File");

    let reader = BufReader::new(file);

    let mut lcr_classification_to_lcr_run_off: HashMap<String, String> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Could Not Read").to_string();
        let fields: Vec<&str> = line.split('|').collect();
        if fields.len() >= 2 {
            lcr_classification_to_lcr_run_off.insert(fields[0].to_string(), fields[1].to_string());
        } else {
            log_error!(
                logger,
                "fields are less in the line-no.{} in runoff File. line is '{}'.",
                (index + 1).to_string(),
                line
            );
        }
    }

    lcr_classification_to_lcr_run_off
}

pub fn get_cust_type_to_cust_type_desc(
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> HashMap<String, String> {
    let mut cust_type_map_cust_type_desc: HashMap<String, String> = HashMap::new();

    let mut excel_file: Xlsx<_> = open_workbook(&config_params.master_prod_cust_type_file())
        .expect("Could not open master-prod-cust-type file.");
    if let Some(Ok(r)) =
        excel_file.worksheet_range(&config_params.master_prod_cust_type_file_sheet())
    {
        for (index, row) in r.rows().enumerate().skip(1) {
            if row.len() >= 3 {
                cust_type_map_cust_type_desc.insert(row[0].to_string(), row[2].to_string());
            } else {
                log_error!(
                    logger,
                    "fields are less in the line-no.{} in master_prod_cust_type File.",
                    (index + 1).to_string()
                );
            }
        }
    }

    cust_type_map_cust_type_desc
}
