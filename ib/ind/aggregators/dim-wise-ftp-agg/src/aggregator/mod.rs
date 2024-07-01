use self::io::*;
use self::structs::{StamperKey, StamperValues};
use aggregator::config::ConfigData;
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use hashbrown::HashMap;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use slog::Logger;
use std::fs;
use std::io::Write;
use std::time::SystemTime;

mod config;
mod io;
mod structs;

pub fn generatesummary(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let mut tot_acc_encntrd = 0;
    let mut skp_acc = 0;
    let mut ttl_amt: f64 = 0.0;
    let keys = ConfigData::new_from_path(config_params.config_file_path());
    let output_path = config_params.output_file_path().to_string();
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let as_on_month = config_params.as_on_date().month();
    let asondate = config_params.as_on_date().format("%d-%m-%Y").to_string();
    let mut stamper_key_val_map: HashMap<StamperKey, StamperValues> = HashMap::new();
    let mut stamper_key_aorl_map: HashMap<StamperKey, String> = HashMap::new();
    let start_time = SystemTime::now();

    for file in &keys.stamper_files {
        let reader = match fs::read_to_string(&file) {
            Ok(file) => file,
            Err(error) => panic!(
            "Could not found input file: `{}` : {}.",
            &file,
            error
            ),
        };
        let mut index = 0;
        for line in reader.lines().skip(1) {
            index += 1;
            tot_acc_encntrd += 1;
            let fields = line.split("|").collect::<Vec<&str>>();
            if fields.len() != 60 {
                log_warn!(
                    logger,
                    "Number of fields is not equal to 60 at line {} in file {}",
                    index+1,
                    &file
                );
                skp_acc += 1;
                continue;
            }
            let stamper_month = date_parser.parse(fields[0]).month();

            if stamper_month != as_on_month {
                skp_acc += 1;
                log_debug!(
                    logger,
                    "Found stamper month not equal to as_on_month for acc_no: {}",
                    fields[1]
                );
                continue;
            }

            if fields[37] == "null" || fields[37] == "NULL" || fields[37] == "" {
                if keys.is_aorl_null == "Y" {
                    log_debug!(
                        logger,
                        "Found AorL Null at line {} in file {}",
                        index+1,
                        &file
                    );
                    skp_acc += 1;
                    continue;
                } else {
                    panic!("Found AorL as null in {} and AorL flag is N", &file)
                }
            }

            if fields[43] == "null" || fields[43] == "NULL" || fields[43] == "" {
                log_warn!(
                    logger,
                    "Found RLG item id NULL at line {} in file {}",
                    index+1,
                    &file
                );
                skp_acc += 1;
                continue;
            }

            if keys.dims.contains(&"DIM1".to_string()) {
                if fields[38] == "null" || fields[38] == "NULL" || fields[38] == "" {
                    log_warn!(
                        logger,
                        "Found DIM1 item id NULL at line {} in file {}",
                        index+1,
                        &file
                    );
                    skp_acc += 1;
                    continue;
                }
                dim_wise_aggr(
                    &config_params,
                    logger,
                    38 as usize,
                    _diag_logger,
                    &fields,
                    &mut stamper_key_val_map,
                    &mut stamper_key_aorl_map,
                    "DIM1".to_string(),
                    &keys,
                    index,
                    &file,
                );
            }
            if keys.dims.contains(&"DIM2".to_string()) {
                if fields[39] == "null" || fields[39] == "NULL" || fields[39] == "" {
                    log_warn!(
                        logger,
                        "Found DIM2 item id NULL at line {} in file {}",
                        index+1,
                        &file
                    );
                    skp_acc += 1;
                    continue;
                }
                dim_wise_aggr(
                    &config_params,
                    logger,
                    39 as usize,
                    _diag_logger,
                    &fields,
                    &mut stamper_key_val_map,
                    &mut stamper_key_aorl_map,
                    "DIM2".to_string(),
                    &keys,
                    index,
                    &file,
                );
            }
            if keys.dims.contains(&"DIM3".to_string()) {
                if fields[40] == "null" || fields[40] == "NULL" || fields[40] == "" {
                    log_warn!(
                        logger,
                        "Found DIM3 item id NULL at line {} in file {}",
                        index+1,
                        &file
                    );
                    skp_acc += 1;
                    continue;
                }
                dim_wise_aggr(
                    &config_params,
                    logger,
                    40 as usize,
                    _diag_logger,
                    &fields,
                    &mut stamper_key_val_map,
                    &mut stamper_key_aorl_map,
                    "DIM3".to_string(),
                    &keys,
                    index+1,
                    &file,
                );
            }
            if keys.dims.contains(&"DIM4".to_string()) {
                if fields[41] == "null" || fields[41] == "NULL" || fields[41] == "" {
                    log_warn!(
                        logger,
                        "Found DIM4 item id NULL at line {} in file {}",
                        index+1,
                        &file
                    );
                    skp_acc += 1;
                    continue;
                }
                dim_wise_aggr(
                    &config_params,
                    logger,
                    41 as usize,
                    _diag_logger,
                    &fields,
                    &mut stamper_key_val_map,
                    &mut stamper_key_aorl_map,
                    "DIM4".to_string(),
                    &keys,
                    index,
                    &file,
                );
            }

            if keys.dims.contains(&"RL1".to_string()) {
                if fields[43] == "null" || fields[43] == "NULL" || fields[43] == "" {
                    log_warn!(
                        logger,
                        "Found RL1 item id NULL at line {} in file {}",
                        index+1,
                        &file
                    );
                    skp_acc += 1;
                    continue;
                }
                dim_wise_aggr(
                    &config_params,
                    logger,
                    43 as usize,
                    _diag_logger,
                    &fields,
                    &mut stamper_key_val_map,
                    &mut stamper_key_aorl_map,
                    "RL1".to_string(),
                    &keys,
                    index,
                    &file,
                );
            }
        }
        log_info!(logger, "File processed present at path {}", &file);
    }
    let mut op_writer = get_writer(&output_path);

    for (key, val) in stamper_key_val_map.iter() {
        let mut ftp_rate_final = val.avg_ftp_rate;
        let mut int_rate_final = val.avg_int_rate;
        if keys.weighted_int_rt_req == "Y" {
            if val.avg_bal == 0.0 {
                int_rate_final = 0.0;
            }
            if val.ftp_amt == 0.0 {
                ftp_rate_final = 0.0;
            }
        } else {
            if val.avg_bal == 0.0 {
                int_rate_final = 0.0;
                ftp_rate_final = 0.0;
            }
        }
        let aorl = stamper_key_aorl_map.get(key).unwrap();
        let op_line = format!(
            "{}|{}|{}|RL1|{}|{}|{}|{}|{}|{}|{}",
            asondate,
            key.dim_id,
            key.dim_item_id,
            key.rlg_item_id,
            aorl,
            val.avg_bal,
            int_rate_final,
            val.int_amt,
            ftp_rate_final,
            val.ftp_amt
        );
        writeln!(op_writer, "{}", op_line).expect("Unable to generate summary file.");
    }

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}

pub fn dim_wise_aggr(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    dim_index: usize,
    _diag_logger: &Logger,
    fields: &Vec<&str>,
    stamper_key_val_map: &mut HashMap<StamperKey, StamperValues>,
    stamper_key_aorl_map: &mut HashMap<StamperKey, String>,
    dim_val: String,
    keys: &ConfigData,
    index: i64,
    file: &String,
) {
    let mut stmp_key = StamperKey::new();
    stmp_key.dim_id = dim_val;
    stmp_key.dim_item_id = fields[dim_index].to_string();
    stmp_key.rlg_item_id = fields[43].to_string();

    if stamper_key_aorl_map.contains_key(&stmp_key) {
        let aorl_check = stamper_key_aorl_map.get(&stmp_key).unwrap();
        if aorl_check != &fields[37].to_string() {
            panic!(
                "Found two different AorL for same RLG at line {} in file {}",
                index+1, &file
            );
        }
    } else {
        stamper_key_aorl_map.insert(stmp_key.to_owned(), fields[37].to_string());
    }

    let avg_bal = fields[6].to_string().parse::<f64>().unwrap_or(0.0);
    let int_rt = fields[7].to_string().parse::<f64>().unwrap_or(0.0);
    let int_amt = fields[9].to_string().parse::<f64>().unwrap_or(0.0);
    let ftp_rt = fields[33].to_string().parse::<f64>().unwrap_or(0.0);
    let ftp_amt = fields[36].to_string().parse::<f64>().unwrap_or(0.0);
    let mut avg_int_rt = 0.0;
    let mut avg_ftp_rt = 0.0;
    if stamper_key_val_map.contains_key(&stmp_key) {
        let stmp_val = stamper_key_val_map.get(&stmp_key).unwrap();
        let stmp_avg_bal = stmp_val.avg_bal;
        let stmp_int_rt = stmp_val.int_rate;
        let stmp_ftp_rt = stmp_val.ftp_rate;
        let stmp_ftp_amt = stmp_val.ftp_amt;
        let stmp_int_amt = stmp_val.int_amt;
        let stmp_sum_prod_int_rt_avg_bal = stmp_val.sum_prod_int_rt_avg_bal;
        let stmp_sum_prod_ftp_rt_ftp_amt = stmp_val.sum_prod_ftp_rt_ftp_amt;
        let mut updt_stmp_val = StamperValues::new();

        updt_stmp_val.avg_bal = avg_bal + stmp_avg_bal;
        updt_stmp_val.int_rate = int_rt + stmp_int_rt;
        updt_stmp_val.ftp_amt = ftp_amt + stmp_ftp_amt;
        updt_stmp_val.int_amt = int_amt + stmp_int_amt;
        updt_stmp_val.ftp_rate = ftp_rt + stmp_ftp_rt;
        updt_stmp_val.sum_prod_int_rt_avg_bal = (int_rt * avg_bal) + stmp_sum_prod_int_rt_avg_bal;
        updt_stmp_val.sum_prod_ftp_rt_ftp_amt = (ftp_amt * ftp_rt) + stmp_sum_prod_ftp_rt_ftp_amt;

        if keys.weighted_int_rt_req == "Y" {
            avg_int_rt = (updt_stmp_val.sum_prod_int_rt_avg_bal) / updt_stmp_val.avg_bal;
            avg_ftp_rt = (updt_stmp_val.sum_prod_ftp_rt_ftp_amt) / updt_stmp_val.ftp_amt;
        } else {
            avg_int_rt = (updt_stmp_val.int_amt / updt_stmp_val.avg_bal) * 1200.0;
            avg_ftp_rt = (updt_stmp_val.ftp_amt / updt_stmp_val.avg_bal) * 1200.0;
        }

        updt_stmp_val.avg_ftp_rate = avg_ftp_rt;
        updt_stmp_val.avg_int_rate = avg_int_rt;

        stamper_key_val_map.remove(&stmp_key);
        stamper_key_val_map.insert(stmp_key, updt_stmp_val);
    } else {
        if keys.weighted_int_rt_req == "Y" {
            avg_int_rt = (avg_bal * int_rt) / avg_bal;
            avg_ftp_rt = (ftp_amt * ftp_rt) / ftp_amt;
        } else {
            avg_int_rt = (int_amt / avg_bal) * 1200.0;
            avg_ftp_rt = (ftp_amt / avg_bal) * 1200.0;
        }

        let mut stmp_val_new = StamperValues::new();
        stmp_val_new.avg_bal = avg_bal;
        stmp_val_new.int_amt = int_amt;
        stmp_val_new.ftp_amt = ftp_amt;
        stmp_val_new.avg_ftp_rate = avg_ftp_rt;
        stmp_val_new.avg_int_rate = avg_int_rt;
        stmp_val_new.ftp_rate = ftp_rt;
        stmp_val_new.int_rate = int_rt;
        stmp_val_new.sum_prod_int_rt_avg_bal = avg_bal * int_rt;
        stmp_val_new.sum_prod_ftp_rt_ftp_amt = ftp_amt * ftp_rt;

        stamper_key_val_map.insert(stmp_key, stmp_val_new);
    }
}
