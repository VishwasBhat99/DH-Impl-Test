use configuration_parameters::ConfigurationParameters;
use macros;
use slog::Logger;
use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct TenorKey {
    pub tenor: String,
    pub pay_freq: String,
    pub int_freq: String,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct BMKey {
    pub bm: String,
    pub bm_freq: String,
    pub bm_res_days: i32,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct BMRates {
    pub bm: String,
    pub tenor: String,
    pub uom: String,
}

pub fn read_config_files(
    config_params: &ConfigurationParameters,
    _logger: &Logger,
    diag_logger: &Logger,
) -> (
    Vec<f64>,
    HashMap<BMKey, f64>,
    HashMap<TenorKey, f64>,
    HashMap<BMRates, Vec<f64>>,
    HashMap<String, String>,
) {
    // Read Amount Disbursement by Day File into Vec
    let dis_day_reader = sdb_io::new_buf_rdr(config_params.disbursement_by_day_file_path())
        .expect("Cannot open Disbursement by Day File!");
    let mut dis_day_map: Vec<f64> = Vec::with_capacity(31);
    let mut dis_day_weight_check = 0.0;
    for (_, lines) in dis_day_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from amt disbursement by day file!");
        let line_info: Vec<&str> = line.split('|').collect();
        if line_info[0]
            == config_params
                .as_on_date()
                .succ()
                .format("%d-%m-%Y")
                .to_string()
            && line_info[1] == config_params.prj_coa()
        {
            for idx in 2..33 {
                let weightage: f64 = line_info[idx].parse().unwrap_or(0.0);
                dis_day_map.push(weightage);
                dis_day_weight_check += weightage;
            }
        } else {
            continue;
        }
    }
    log_debug!(diag_logger, "Day Disbursement Mapping{:?}", dis_day_map);
    if dis_day_weight_check != 100.0 {
        panic!(
            "Weightage for disbursement by day is not correct,current mapped value : {}% Check Log for more info.",
            dis_day_weight_check
        );
    }

    // Read Amount Disbursement by Tenor File into HashMap
    let dis_tenor_reader = sdb_io::new_buf_rdr(config_params.disbursement_by_tenor_file_path())
        .expect("Cannot open Disbursement by Tenor File!");
    let mut dis_tenor_map: HashMap<TenorKey, f64> = HashMap::new();
    let mut dis_tenor_weight_check = 0.0;
    for (_, lines) in dis_tenor_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from amt disbursement by tenor file!");
        let line_info: Vec<&str> = line.split('|').collect();
        if line_info[0]
            == config_params
                .as_on_date()
                .succ()
                .format("%d-%m-%Y")
                .to_string()
            && line_info[1] == config_params.prj_coa()
        {
            let info = TenorKey {
                tenor: line_info[2].to_string() + line_info[3],
                pay_freq: line_info[4].to_string(),
                int_freq: line_info[5].to_string(),
            };
            let tenor_weightage: f64 = line_info[6].parse().unwrap_or(0.0);
            if tenor_weightage != 0.0 {
                dis_tenor_weight_check += tenor_weightage;
                dis_tenor_map.insert(info, tenor_weightage);
            }
        } else {
            continue;
        }
    }
    log_debug!(
        diag_logger,
        "Tenor Disbursement Mapping: {:?}",
        dis_tenor_map
    );
    if dis_tenor_weight_check != 100.0 {
        panic!(
            "Weightage for disbursement by tenor is not correct,current mapped value : {}% Check Log for more info.",
            dis_tenor_weight_check
        );
    }

    // Read Amount Disbursement by BM File into HashMap
    let dis_bm_reader = sdb_io::new_buf_rdr(config_params.disbursement_by_bm_file_path())
        .expect("Cannot open Disbursement by bm File!");
    let mut dis_bm_map: HashMap<BMKey, f64> = HashMap::new();
    let mut dis_bm_weight_check = 0.0;
    for (_, lines) in dis_bm_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from amt disbursement by bm file!");
        let line_info: Vec<&str> = line.split('|').collect();
        if line_info[0]
            == config_params
                .as_on_date()
                .succ()
                .format("%d-%m-%Y")
                .to_string()
            && line_info[1] == config_params.prj_coa()
        {
            let info = BMKey {
                bm: line_info[2].to_string(),
                bm_freq: line_info[3].to_string(),
                bm_res_days: line_info[4].parse().unwrap_or(0),
            };
            let bm_weightage: f64 = line_info[5].parse().unwrap_or(0.0);
            if bm_weightage != 0.0 {
                dis_bm_weight_check += bm_weightage;
                dis_bm_map.insert(info, bm_weightage);
            }
        } else {
            continue;
        }
    }
    log_debug!(diag_logger, "BM Disbursement Mapping: {:?}", dis_bm_map);
    if dis_bm_weight_check != 100.0 {
        panic!(
            "Weightage for disbursement by bm is not correct,current mapped value : {}% Check Log for more info.",
            dis_bm_weight_check
        );
    }

    // Read BM Rate Mapping File in HashMap
    let bm_rate_map_reader =
        sdb_io::new_buf_rdr(config_params.bm_rates_file_path()).expect("Cannot open BM Rate File!");
    let mut bm_rate_map: HashMap<BMRates, Vec<f64>> = HashMap::new();
    for (_, lines) in bm_rate_map_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from BM Rate file!");
        let line_info: Vec<&str> = line.split('|').collect();
        if line_info[0]
            == config_params
                .as_on_date()
                .succ()
                .format("%d-%m-%Y")
                .to_string()
        {
            let info = BMRates {
                bm: line_info[1].to_string(),
                tenor: line_info[2].to_string(),
                uom: line_info[3].to_string(),
            };
            let mut rate_values: Vec<f64> = Vec::new();
            for idx in 4..64 {
                rate_values.push(line_info[idx].parse().unwrap_or(0.0));
            }
            bm_rate_map.insert(info, rate_values);
        }
    }
    log_debug!(diag_logger, "BM Rate Mapping: {:?}", bm_rate_map);

    // Read COA Master File in HashMap
    let coa_map_reader = sdb_io::new_buf_rdr(config_params.coa_master_file_path())
        .expect("Cannot open COA Master File!");
    let mut coa_map: HashMap<String, String> = HashMap::new();
    for (_, lines) in coa_map_reader.lines().enumerate() {
        let line = lines.expect("Cannot read data from coa master file!");
        let line_info: Vec<&str> = line.split('|').collect();
        coa_map.insert(line_info[0].to_string(), line_info[1].to_string());
    }
    log_debug!(diag_logger, "COA Mapping: {:?}", coa_map);

    (dis_day_map, dis_bm_map, dis_tenor_map, bm_rate_map, coa_map)
}
