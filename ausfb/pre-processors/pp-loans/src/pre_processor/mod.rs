extern crate csv;
extern crate serde;
use self::derive::{calc_reset_date, naivedate_for_master, naivedate_for_ref2, remove_junk_char};
use calamine::{open_workbook_auto, Reader};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

mod derive;
pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let start_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let def_string = "NA".to_string();
    let mut output_writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let master_file = match new_buf_rdr(config_param.master_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found master_file: `{}`",
            config_param.master_file_path(),
        ),
    };
    let ref1_file_extension = Path::new(config_param.ref1_file_path())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("txt");
    let mut ref1_file_map: HashMap<String, Vec<String>> = HashMap::new();
    // Ref1 File reading started
    log_debug!(log, " Ref1 File reading started");
    if ref1_file_extension == "xlsx" || ref1_file_extension == "xls" {
        let mut ref1_excel = open_workbook_auto(&config_param.ref1_file_path())
            .expect("Unable to open  reference File.");
        if let Some(Ok(reader)) = ref1_excel.worksheet_range(&config_param.ref1_file_sheet_name) {
            for row in reader.rows().skip(1) {
                let mut ref_data_without_quotes = Vec::new();
                for s in row.iter() {
                    ref_data_without_quotes.push(remove_junk_char(&s.to_string()));
                }
                let gl = ref_data_without_quotes.remove(0);
                ref1_file_map.insert(gl, ref_data_without_quotes);
            }
        }
    } else {
        let ref1_file: std::io::BufReader<std::fs::File> =
            match new_buf_rdr(config_param.ref1_file_path()) {
                Ok(file) => file,
                Err(_error) => panic!(
                    "Could not found ref1_file: `{}`",
                    config_param.ref1_file_path(),
                ),
            };

        for (line_num, lines) in ref1_file.lines().enumerate().skip(1) {
            let ref1_line = match lines {
                Ok(ref1_line) => ref1_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.ref1_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let mut ref1_fields = remove_junk_char(&ref1_line)
                .split('|')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let gl = ref1_fields.remove(0);
            ref1_file_map.insert(gl, ref1_fields);
        }
    }
    log_debug!(log, "Ref1 File Reading Completed");
    let mut ref2_file_map: HashMap<String, Vec<String>> = HashMap::new();
    //Ref2 File reading started
    log_debug!(log, "ref2 File reading started");
    let ref2_file = match new_buf_rdr(config_param.ref2_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found ref2_file: `{}`",
            config_param.ref2_file_path(),
        ),
    };
    for (line_num, lines) in ref2_file.lines().enumerate().skip(1) {
        let ref2_line = match lines {
            Ok(ref2_line) => ref2_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref2_file_path(),
                line_num + 1,
                error
            ),
        };
        let ref2_fields: Vec<String> = remove_junk_char(&ref2_line)
            .split(',')
            .map(|s| s.to_string())
            .collect();
        let cod_acct_no = ref2_fields[0].to_string();
        let ref_2_data = vec![
            ref2_fields[6].to_string(),
            ref2_fields[7].to_string(),
            ref2_fields[10].to_string(),
            ref2_fields[11].to_string(),
        ];
        ref2_file_map.insert(cod_acct_no, ref_2_data);
    }
    log_debug!(log, "ref2 File Reading Completed");
    let mut ref3_file_map: HashMap<String, bool> = HashMap::new();
    log_debug!(log, "Benchmark File reading started");
    let ref3_file = match new_buf_rdr(config_param.ref3_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found ref3_file: `{}`",
            config_param.ref3_file_path(),
        ),
    };

    for (line_num, lines) in ref3_file.lines().enumerate().skip(1) {
        let benchmark_line = match lines {
            Ok(benchmark_line) => benchmark_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref3_file_path(),
                line_num + 1,
                error
            ),
        };

        ref3_file_map.insert(benchmark_line.trim().to_string(), true);
    }

    log_debug!(log, "Ref3 File Reading Completed");

    let ref4_file_extension = Path::new(config_param.ref4_file_path())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("txt");
    let mut ref4_file_map: HashMap<String, String> = HashMap::new();
    // Ref4 File reading started
    log_debug!(log, " Ref4 File reading started");
    if ref4_file_extension == "xlsx" || ref4_file_extension == "xls" {
        let mut ref4_excel = open_workbook_auto(&config_param.ref4_file_path())
            .expect("Unable to open  reference File 4.");
        if let Some(Ok(reader)) = ref4_excel.worksheet_range(&config_param.ref4_file_sheet_name) {
            for row in reader.rows().skip(1) {
                let mut ref_data_without_quotes = Vec::new();
                for s in row.iter() {
                    ref_data_without_quotes.push(remove_junk_char(&s.to_string()));
                }
                let account_no = ref_data_without_quotes[0].to_string();
                ref4_file_map.insert(account_no, ref_data_without_quotes[1].to_string());
            }
        }
    } else {
        let ref4_file: std::io::BufReader<std::fs::File> =
            match new_buf_rdr(config_param.ref4_file_path()) {
                Ok(file) => file,
                Err(_error) => panic!(
                    "Could not found ref1_file: `{}`",
                    config_param.ref4_file_path(),
                ),
            };

        for (line_num, lines) in ref4_file.lines().enumerate().skip(1) {
            let ref4_line = match lines {
                Ok(ref4_line) => ref4_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.ref4_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let ref4_fields = remove_junk_char(&ref4_line)
                .split('|')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let account_no = ref4_fields[0].to_string();
            ref4_file_map.insert(account_no, ref4_fields[1].to_string());
        }
    }
    log_debug!(log, "Ref4 File Reading Completed");
    //Ref4 file reading completed
    let mut ref5_file_map: HashMap<String, String> = HashMap::new();
    //Ref5 File reading started
    log_debug!(log, "ref5 File reading started");
    let ref5_file = match new_buf_rdr(config_param.ref5_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found ref5_file: `{}`",
            config_param.ref5_file_path(),
        ),
    };
    for (line_num, lines) in ref5_file.lines().enumerate().skip(1) {
        let ref5_line = match lines {
            Ok(ref5_line) => ref5_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref5_file_path(),
                line_num + 1,
                error
            ),
        };
        let ref5_fields: Vec<String> = remove_junk_char(&ref5_line)
            .split('|')
            .map(|s| s.to_string())
            .collect();
        let cod_acct_no = ref5_fields[3].to_string();
        ref5_file_map.insert(cod_acct_no, ref5_fields[14].to_string());
    }
    log_debug!(log, "ref5 File Reading Completed");

    let mut cust_entity_master_file_map: HashMap<String, String> = HashMap::new();
    log_debug!(log, "cust_entity_master File reading started");
    let cust_entity_master_file = match new_buf_rdr(config_param.cust_entity_master_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found cust_entity_master_file: `{}`",
            config_param.cust_entity_master_file_path(),
        ),
    };

    for (line_num, lines) in cust_entity_master_file.lines().enumerate().skip(1) {
        let cust_entity_master_line = match lines {
            Ok(cust_entity_master_line) => cust_entity_master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cust_entity_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let cust_entity_master_fields = remove_junk_char(&cust_entity_master_line)
            .split("~#~")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let flag_cust_typ = cust_entity_master_fields[0].to_string();

        if !cust_entity_master_file_map.contains_key(&flag_cust_typ.trim().to_string()) {
            cust_entity_master_file_map.insert(
                flag_cust_typ.trim().to_string(),
                cust_entity_master_fields[3].to_string(),
            );
        }
    }

    log_debug!(log, "Cust entity master File Reading Completed");

    let mut crm_master_cust_id_file_map: HashMap<String, String> = HashMap::new();
    log_debug!(log, "crm_master File reading started");
    let crm_master_file = match new_buf_rdr(config_param.crm_master_file_path()) {
        Ok(file) => file,
        Err(_error) => panic!(
            "Could not found crm_master_file: `{}`",
            config_param.crm_master_file_path(),
        ),
    };

    for (line_num, lines) in crm_master_file.lines().enumerate().skip(0) {
        let crm_master_line = match lines {
            Ok(crm_master_line) => crm_master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.crm_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let crm_master_fields = remove_junk_char(&crm_master_line)
            .split("~#~")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let cust_id = crm_master_fields[0].to_string();
        if !crm_master_cust_id_file_map.contains_key(&cust_id.trim().to_string()) {
            crm_master_cust_id_file_map
                .insert(cust_id.trim().to_string(), crm_master_fields[3].to_string());
        }
    }

    log_debug!(log, "Crm master File Reading Completed");
    //Master File reading started
    log_debug!(log, "Master File reading started");
    for (line_num, lines) in master_file.lines().enumerate().skip(1) {
        tot_rec += 1;
        let master_line = match lines {
            Ok(master_line) => master_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.master_file_path(),
                line_num + 1,
                error
            ),
        };

        let master_fields = remove_junk_char(&master_line)
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if naivedate_for_master(&master_fields[14], config_param.as_on_date())
            > *config_param.as_on_date()
        {
            log_debug!(
                log,
                "{} is skipped because first_disb_date:{} is greater than as_on_date: {}",
                master_fields[0],
                master_fields[14],
                config_param.as_on_date()
            );
        } else {
            succ_rec += 1;
            //derivation of fields
            let def_ref1_data = vec![
                "NA".to_string(),
                "NA".to_string(),
                "NA".to_string(),
                "NA".to_string(),
            ];
            let def_ref2_data = vec![
                "NA".to_string(),
                "NA".to_string(),
                "NA".to_string(),
                "NA".to_string(),
            ];
            let (foreclosure, foreclosure_rate_2) =
                if ref1_file_map.contains_key(&master_fields[10]) {
                    let ref1_data = ref1_file_map
                        .get(&master_fields[10])
                        .unwrap_or(&def_ref1_data);
                    let foreclosure = ref1_data[2].replace("%", "").parse::<f64>().unwrap_or(0.0);

                    (foreclosure, 1.0000 - foreclosure)
                } else {
                    (0.0000, 1.0000)
                };
            let foreclosure_rate_1 = if foreclosure == 0.0 {
                1.0000
            } else {
                1.0000 - foreclosure
            };
            let derived_arrear_date = if master_fields[28].is_empty()
                || naivedate_for_master(&master_fields[28], config_param.as_on_date())
                    == naivedate_for_master("01-JAN-1900", config_param.as_on_date())
            {
                naivedate_for_master(&master_fields[15], config_param.as_on_date())
            } else {
                naivedate_for_master(&master_fields[28], config_param.as_on_date())
            };

            let (derived_reset_date, rate_type) = calc_reset_date(
                master_fields[25].to_string(),
                master_fields[1].to_string(),
                &ref3_file_map,
                &ref2_file_map,
                *config_param.as_on_date(),
                naivedate_for_master(&master_fields[15], config_param.as_on_date()),
            );
            let ref2_data = ref2_file_map
                .get(&master_fields[1])
                .unwrap_or(&def_ref2_data);
            let default_ref5_val = "STANDARD".to_string();
            let npa_status = ref4_file_map.get(&master_fields[1]).unwrap_or(
                ref5_file_map
                    .get(&master_fields[1])
                    .unwrap_or(&default_ref5_val),
            );
            let lcr_catagory = cust_entity_master_file_map
                .get(&master_fields[9])
                .unwrap_or(&def_string);
            let cust_type = crm_master_cust_id_file_map
                .get(&master_fields[0])
                .unwrap_or(
                    cust_entity_master_file_map
                        .get(&master_fields[9])
                        .unwrap_or(&def_string),
                );
            let npa_final_status = if npa_status.to_uppercase().contains("DOUBTFUL") {
                "DOUBTFUL"
            } else {
                match npa_status.to_uppercase().as_str() {
                    "STANDARD" => "STANDARD",
                    "LOSS" | "LOS" => "LOSS",
                    "SUB-STANDARD" | "SUB STANDARD" => "SUB-STANDARD",
                    _ => "NA",
                }
            };
            let output_line = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.4}|{:.4}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|||",
                    master_fields[0],
                    master_fields[1],
                    master_fields[7],
                    master_fields[8],
                    master_fields[9],
                    master_fields[10],
                    master_fields[11],
                    master_fields[12],
                    naivedate_for_master(&master_fields[13], config_param.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string(),
                    naivedate_for_master(&master_fields[14], config_param.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string(),
                    naivedate_for_master(&master_fields[15], config_param.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string(),
                    master_fields[17],
                    master_fields[22],
                    master_fields[23],
                    master_fields[25],
                    rate_type,
                    npa_status,
                    npa_final_status,
                    master_fields[35],
                    foreclosure*100.00,
                    foreclosure_rate_1,
                    foreclosure_rate_2,
                    ref2_data[0],
                    if ref2_data[1]=="NA".to_string(){
                          "FIXED".to_string()
                    }
                    else{
                        ref2_data[1].to_string()
                    },
                    naivedate_for_ref2(&ref2_data[2], config_param.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string(),
                    ref2_data[3],
                    derived_reset_date.format("%d-%m-%Y").to_string(),
                    derived_arrear_date.format("%d-%m-%Y").to_string(),
                    master_fields[30],
                    lcr_catagory,
                    cust_type,
                    master_fields[17].parse().unwrap_or(0.0)+master_fields[30].parse().unwrap_or(0.0),
                );
            writeln!(output_writer, "{}", output_line)
                .expect("loans output line can not be written");
        }
    }
    log_debug!(log, "Master File Reading Completed");
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total process duration.");
    log_debug!(
        log,
        "Total Duration for preprocess the data: {:?}.",
        duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_param.output_file_path());
}
