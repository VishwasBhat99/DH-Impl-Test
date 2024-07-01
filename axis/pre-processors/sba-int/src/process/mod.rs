use crate::process::structs::IvsIntVal;

use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
mod structs;
use self::structs::{BalmIcvIvsKey, BalmIcvVal, BalmIvsVal, SBAInt};
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    let tot_amt = 0.0;

    let mut final_out_sba_map: HashMap<String, SBAInt> = HashMap::new();
    let itc_input = File::open(&config_params.input_file_itc()).expect("Could Not open ITC file");
    let icv_input = File::open(&config_params.input_file_icv()).expect("Could Not open ICV file");
    let ivs_input = File::open(&config_params.input_file_ivs()).expect("Could not open IVS File.");
    let gam_input = File::open(&config_params.input_file_gam()).expect("Could not open GAM File.");
    let gam_input_reader = BufReader::new(gam_input);

    //GAM
    let start_gam_reader = SystemTime::now();
    for (_index, line) in gam_input_reader.lines().enumerate() {
        let line = line.expect("Could not read line in GAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let mut new_record = SBAInt::new();
        if input_fields[15].trim() == "SBA" {
            new_record.schm_type = "SBA".to_string();
            new_record.acct_crncy_code = input_fields[17].to_string();
            new_record.out_bal_amt = input_fields[50].parse::<f64>().unwrap_or(0.0);
            if input_fields[3].parse::<f64>().unwrap_or(0.0) >= 0.0 {
                new_record.int_slab_dr_cr_flg = 'C'.to_string();
            } else {
                new_record.int_slab_dr_cr_flg = 'D'.to_string();
            }
        }
        final_out_sba_map.insert(input_fields[0].to_string(), new_record);
    }
    let end_gam_reader = SystemTime::now();
    let total_gam_duration = end_gam_reader
        .duration_since(start_gam_reader)
        .expect("Could not calculate total duration for processing GAM.");
    log_debug!(
        logger,
        "Reading and processing GAM File, Total Duration: {:?}.",
        total_gam_duration
    );

    //ITC
    let itc_input_reader = BufReader::new(itc_input);
    let start_itc_reader = SystemTime::now();
    let mut int_tbl_code_srl_num: HashMap<String, f64> = HashMap::new();
    for (_index, line) in itc_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line in ITC file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        //Lookup key is acid=entity_id.
        if final_out_sba_map.contains_key(input_fields[0]) {
            let new_record = final_out_sba_map
                .get_mut(input_fields[0].trim())
                .expect("Cannot fetch ITC entity ID in GAM file.");
            let itc_int_tbl_code_srl_num = int_tbl_code_srl_num
                .get(input_fields[0].trim())
                .unwrap_or(&0.0);
            if new_record.int_slab_dr_cr_flg == "C"
                && new_record.itc_lchg_time <= date_parser.parse(input_fields[15])
                && *itc_int_tbl_code_srl_num < input_fields[17].parse::<f64>().unwrap_or(0.0)
            {
                int_tbl_code_srl_num.insert(
                    input_fields[0].to_string(),
                    input_fields[17].parse::<f64>().unwrap_or(0.0),
                );
                new_record.int_tbl_code = input_fields[2].to_string();
                new_record.cust_cr_pref_pcnt = input_fields[5].parse::<f64>().unwrap_or(0.0);
                new_record.id_cr_pref_pnt = input_fields[7].parse::<f64>().unwrap_or(0.0);
                new_record.min_int_pcnt_cr = input_fields[9].parse::<f64>().unwrap_or(0.0);
                new_record.max_int_pcnt_cr = input_fields[11].parse::<f64>().unwrap_or(0.0);
                new_record.itc_lchg_time = date_parser.parse(input_fields[15]);
            }
        }
    }

    let end_itc_reader = SystemTime::now();
    let total_itc_duration = end_itc_reader
        .duration_since(start_itc_reader)
        .expect("Could not calculate total duration for itc.");
    log_debug!(
        logger,
        "Reading and processing ITC File, Total Duration: {:?}.",
        total_itc_duration
    );

    //ICV
    let icv_input_reader = BufReader::new(icv_input);
    let start_icv_reader = SystemTime::now();
    let mut balm_icv_data: HashMap<BalmIcvIvsKey, BalmIcvVal> = HashMap::new();
    for (_index, line) in icv_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let new_balm_icv = BalmIcvIvsKey {
            int_tbl_code: input_fields[5].trim().to_string(),
            crncy_code: input_fields[9].trim().to_string(),
        };
        let mut new_balm_icv_val = BalmIcvVal {
            int_tbl_ver_num: input_fields[12]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
            base_pcnt_cr: input_fields[1]
                .trim()
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0),
            int_version: input_fields[0]
                .trim()
                .to_string()
                .parse::<i64>()
                .unwrap_or(0),
        };
        let mut update_balm_icv_val;
        if balm_icv_data.contains_key(&new_balm_icv) {
            update_balm_icv_val = balm_icv_data
                .get_mut(&new_balm_icv)
                .expect("Cannot fetch value from ICV file.")
                .to_owned();
            if update_balm_icv_val.int_tbl_ver_num < new_balm_icv_val.int_tbl_ver_num
                && update_balm_icv_val.int_version <= new_balm_icv_val.int_version
            {
                update_balm_icv_val = new_balm_icv_val;
            }
            balm_icv_data.insert(new_balm_icv, update_balm_icv_val);
        } else {
            balm_icv_data.insert(new_balm_icv, new_balm_icv_val);
        }
    }

    let end_icv_reader = SystemTime::now();
    let total_icv_duration = end_icv_reader
        .duration_since(start_icv_reader)
        .expect("Could not calculate total duration for icv.");
    log_debug!(
        logger,
        "Reading and processing ICV File, Total Duration: {:?}.",
        total_icv_duration
    );

    //IVS

    let ivs_input_reader = BufReader::new(ivs_input);
    let start_ivs_reader = SystemTime::now();
    let mut balm_ivs_data: HashMap<BalmIcvIvsKey, BalmIvsVal> = HashMap::new();
    for (_index, line) in ivs_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        if input_fields[14].trim() == 'C'.to_string() {
            let new_balm_ivs = BalmIcvIvsKey {
                int_tbl_code: input_fields[12].trim().to_string(),
                crncy_code: input_fields[11].trim().to_string(),
            };
            let ivs_val: IvsIntVal = IvsIntVal {
                nrml_int_pcnt: input_fields[1]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0),
                begin_slab_amt: input_fields[10]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0),
                end_slab_amt: input_fields[17]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .unwrap_or(0.0),
            };
            let new_balm_ivs_val = BalmIvsVal {
                int_tbl_ver_num: input_fields[13]
                    .trim()
                    .to_string()
                    .parse::<i64>()
                    .unwrap_or(0),
                int_val: vec![ivs_val],
            };
            let update_balm_ivs_val;
            if balm_ivs_data.contains_key(&new_balm_ivs) {
                update_balm_ivs_val = balm_ivs_data
                    .get_mut(&new_balm_ivs)
                    .expect("Cannot fetch value from IVS file.");
                if update_balm_ivs_val.int_tbl_ver_num < new_balm_ivs_val.int_tbl_ver_num {
                    *update_balm_ivs_val = new_balm_ivs_val;
                } else if update_balm_ivs_val.int_tbl_ver_num == new_balm_ivs_val.int_tbl_ver_num {
                    update_balm_ivs_val
                        .int_val
                        .push(new_balm_ivs_val.int_val[0]);
                }
            } else {
                balm_ivs_data.insert(new_balm_ivs, new_balm_ivs_val);
            }
        }
    }
    let end_ivs_reader = SystemTime::now();
    let total_ivs_duration = end_ivs_reader
        .duration_since(start_ivs_reader)
        .expect("Could not calculate total duration for IVS.");
    log_debug!(
        logger,
        "Reading and processing IVS File, Total Duration: {:?}.",
        total_ivs_duration
    );

    let start_update_write_reader = SystemTime::now();
    //Update ICV and IVS data.
    for (key, mut value) in final_out_sba_map {
        let new_balm_icv_ivs_key = BalmIcvIvsKey {
            crncy_code: value.acct_crncy_code.trim().to_owned(),
            int_tbl_code: value.int_tbl_code.trim().to_owned(),
        };

        if balm_icv_data.contains_key(&new_balm_icv_ivs_key) {
            let new_balm_icv_val = balm_icv_data
                .get(&new_balm_icv_ivs_key)
                .expect("Could not fetch ICV data.");
            value.int_version = new_balm_icv_val.int_version.to_string();
            value.int_tbl_ver_num = new_balm_icv_val.int_tbl_ver_num.to_string();
            value.base_pcnt = new_balm_icv_val.base_pcnt_cr;
        } else {
            log_debug!(
                logger,
                "Could not find struct for an ICV in ICV={:?}",
                new_balm_icv_ivs_key
            );
        }

        let new_balm_ivs_val;
        if balm_ivs_data.contains_key(&new_balm_icv_ivs_key) {
            new_balm_ivs_val = balm_ivs_data
                .get(&new_balm_icv_ivs_key)
                .expect("Could not fetch IVS data.");
            for (val) in new_balm_ivs_val.int_val.iter() {
                if value.out_bal_amt >= val.begin_slab_amt && value.out_bal_amt < val.end_slab_amt {
                    value.nrml_int_pcnt = val.nrml_int_pcnt;
                    break;
                }
            }
        } else {
            log_debug!(
                logger,
                "Could not find struct for an IVS key in IVS={:?}",
                new_balm_icv_ivs_key
            );
        }
        let final_int_rate = value.cust_cr_pref_pcnt.to_owned()
            + value.id_cr_pref_pnt.to_owned()
            + value.base_pcnt.to_owned()
            + value.nrml_int_pcnt.to_owned();
        //write to OP
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            key,
            value.del_flg,
            value.int_slab_dr_cr_flg,
            value.itc_lchg_time.format("%d-%m-%Y"),
            value.schm_type,
            value.int_tbl_code,
            value.int_version,
            value.int_tbl_ver_num,
            value.min_int_pcnt_cr,
            value.max_int_pcnt_cr,
            value.cust_cr_pref_pcnt,
            value.id_cr_pref_pnt,
            value.nrml_int_pcnt,
            value.id_dr_pref_pcnt,
            value.base_int_tbl_code,
            value.base_pcnt_dr,
            value.base_pcnt_cr,
            value.base_pcnt,
            value.acct_crncy_code,
            value.datachanged,
            final_int_rate
        )
        .expect("Unable to write account to output file.");
    }

    let end_update_write_reader = SystemTime::now();
    let total_update_write_duration = end_update_write_reader
        .duration_since(start_update_write_reader)
        .expect("Could not calculate total duration for update_write.");
    log_debug!(
        logger,
        "Reading and processing update_write File, Total Duration: {:?}.",
        total_update_write_duration
    );

    let health_report = HealthReport::new(0, 0, 0, tot_amt, tot_amt, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
