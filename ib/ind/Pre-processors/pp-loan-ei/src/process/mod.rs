use crate::configuration_parameters::ConfigurationParameters;
use crate::process::derive::get_mat_date;
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use rbdate::{datevalue_to_naive_date, num_days_start_to_end, DateParser};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

mod derive;
mod npa_structs;

use self::derive::get_next_rep_date;
use self::npa_structs::{NPAData, RepDateData};
use crate::macros;

pub fn process(config_params: &ConfigurationParameters, _log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Error while getting input directory path.")
                .display(),
            error
        ),
    };

    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut master_excel = open_workbook_auto(config_params.master_file_path())
        .expect("Unable to open Mapping Master File.");
    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(0) {
            let gl_acc_no = row[0].to_string();
            let clsfn = row[2].to_string();
            let group = row[3].to_string();
            let llg = row[4].to_string();
            master_map.insert(gl_acc_no, [clsfn, group, llg].to_vec());
        }
    }

    let mut rep_date_map: HashMap<String, Vec<RepDateData>> = HashMap::new();
    let mut rep_day_month_map: HashMap<String, u32> = HashMap::new();
    let mut rep_date_excel = open_workbook_auto(config_params.next_rep_file())
        .expect("Unable to open Rep Date Excel File.");

    if let Some(Ok(reader)) = rep_date_excel.worksheet_range(config_params.next_rep_sheet_name()) {
        for row in reader.rows().skip(0) {
            let bm = row[0].to_string();
            let rep_date_data = RepDateData::new(row);
            let rep_month_day = format!("{}{}", bm, row[5]);
            if !bm.is_empty() {
                rep_date_map
                    .entry(bm)
                    .and_modify(|data| data.push(rep_date_data.to_owned()))
                    .or_insert_with(|| vec![rep_date_data.to_owned()]);
                rep_day_month_map.insert(
                    rep_month_day,
                    row[4].to_string().parse::<u32>().unwrap_or(0),
                );
            }
        }
    }

    //npa file reading
    let npa_file_extension = Path::new(config_params.npa_master_file_path())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("xlsx");
    let mut npa_master_map: HashMap<String, NPAData> = HashMap::new();
    if npa_file_extension == "txt" || npa_file_extension == "csv" {
        let npa_master_file = match new_buf_rdr(config_params.npa_master_file_path()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found input file: `{}` on location `{}` : {}.",
                config_params.npa_master_file_path(),
                current_dir()
                    .expect("Error while getting input directory path.")
                    .display(),
                error
            ),
        };
        for (line_num, lines) in npa_master_file.lines().enumerate().skip(1) {
            let npa_input_line = match lines {
                Ok(npa_input_line) => npa_input_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_params.input_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let npa_input_fields = npa_input_line
                .split('|')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            let mut npa_key = npa_input_fields[7].to_string();
            npa_key.pop();
            npa_master_map.insert(
                npa_key, //removing the last digit from key
                NPAData {
                    npa_code: npa_input_fields[8].to_string(),
                    npa_cbs_bal: npa_input_fields[10].to_string(),
                    npa_int_unc_prev: npa_input_fields[30].to_string(),
                    npa_claim_recd: npa_input_fields[35].to_string(),
                    npa_sd_held: npa_input_fields[36].to_string(),
                    npa_int_real: npa_input_fields[37].to_string(),
                    npa_tot_cf: npa_input_fields[41].to_string(),
                    npa_exc_prov: npa_input_fields[43].to_string(),
                    npa_prov_sec: npa_input_fields[47].to_string(),
                    npa_prov_unsec: npa_input_fields[48].to_string(),
                    npa_tot_prov: npa_input_fields[49].to_string(),
                    npa_auc: npa_input_fields[50].to_string(),
                    npa_auc_cur: npa_input_fields[52].to_string(),
                    npa_status: npa_input_fields[54].to_string(),
                    gnpa: npa_input_fields[73].to_string(),
                    npa_date: npa_input_fields[3].to_string(),
                    npa_cif: npa_input_fields[6].to_string(),
                    npa_acct_no: npa_input_fields[8].to_string(),
                },
            );
        }
    } else {
        let mut npa_master_excel = open_workbook_auto(config_params.npa_master_file_path())
            .expect("Unable to open NPA Mapping Master File.");
        if let Some(Ok(reader)) = npa_master_excel.worksheet_range(config_params.npa_sheet_name()) {
            for npa_input_fields in reader.rows().skip(1) {
                let mut npa_key = npa_input_fields[7].to_string();
                npa_key.pop();
                npa_master_map.insert(
                    npa_key, //removing the last digit from key
                    NPAData {
                        npa_code: npa_input_fields[8].to_string(),
                        npa_cbs_bal: npa_input_fields[10].to_string(),
                        npa_int_unc_prev: npa_input_fields[30].to_string(),
                        npa_claim_recd: npa_input_fields[35].to_string(),
                        npa_sd_held: npa_input_fields[36].to_string(),
                        npa_int_real: npa_input_fields[37].to_string(),
                        npa_tot_cf: npa_input_fields[41].to_string(),
                        npa_exc_prov: npa_input_fields[43].to_string(),
                        npa_prov_sec: npa_input_fields[47].to_string(),
                        npa_prov_unsec: npa_input_fields[48].to_string(),
                        npa_tot_prov: npa_input_fields[49].to_string(),
                        npa_auc: npa_input_fields[50].to_string(),
                        npa_auc_cur: npa_input_fields[52].to_string(),
                        npa_status: npa_input_fields[54].to_string(),
                        gnpa: npa_input_fields[73].to_string(),
                        npa_date: npa_input_fields[3].to_string(),
                        npa_cif: npa_input_fields[6].to_string(),
                        npa_acct_no: npa_input_fields[8].to_string(),
                    },
                );
            }
        }
    }
    let mut writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    let npa_output_path_split = config_params
        .output_file_path()
        .split('.')
        .collect::<Vec<&str>>();
    let npa_output_directory = format!("{}_npa.txt", npa_output_path_split[0]);
    let mut npa_writer = match buf_file_wrtr(&npa_output_directory, None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create npa output file: `{}` on location `{}` : {}",
            npa_output_directory,
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    for (line_num, lines) in input_file.lines().enumerate() {
        let mut output_line = "".to_string();
        tot_rec += 1;
        if config_params
            .header_rows()
            .split('|')
            .any(|x| x == ((line_num + 1).to_string().as_str()))
        {
            debug!(diag_log, "skipped record for header KEY_1:{}", line_num + 1);
            continue;
        }
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line.split('|').collect::<Vec<&str>>();
        succ_rec += 1;
        let master_val: Vec<String> = master_map
            .get(&input_fields[29].to_string())
            .unwrap_or(&["ASSETS".to_string(), "NONE".to_string(), "NONE".to_string()].to_vec())
            .to_vec();
        //default npa_map_value initialize with NA
        let default_npa_vec = NPAData {
            npa_code: "NA".to_string(),
            npa_cbs_bal: "NA".to_string(),
            npa_int_unc_prev: "NA".to_string(),
            npa_claim_recd: "NA".to_string(),
            npa_sd_held: "NA".to_string(),
            npa_int_real: "NA".to_string(),
            npa_tot_cf: "NA".to_string(),
            npa_exc_prov: "NA".to_string(),
            npa_prov_sec: "NA".to_string(),
            npa_prov_unsec: "NA".to_string(),
            npa_tot_prov: "NA".to_string(),
            npa_auc: "NA".to_string(),
            npa_auc_cur: "NA".to_string(),
            npa_status: "NA".to_string(),
            gnpa: "NA".to_string(),
            npa_date: "NA".to_string(),
            npa_cif: "NA".to_string(),
            npa_acct_no: "NA".to_string(),
        };

        let npa_key = input_fields[0][3..]
            .to_string()
            .trim_start_matches(|c: char| c == '0')
            .to_string();
        let npa_master_val = match npa_master_map.get(&npa_key) {
            Some(val) => val,
            None => {
                log_warn!(
                    _log,
                    "Values Defaulted as data not found in NPA file for key: {}",
                    npa_key
                );
                &default_npa_vec
            }
        };
        let mut maturity_date: String = config_params.as_on_date().to_string();

        //Not to write last 4 fields from Input
        for index in 0..input_fields.len() - 4 {
            //Last repriced and next repriced date calculation
            if index == 37 {
                let appr_date = datevalue_to_naive_date(
                    &(input_fields[14].parse().unwrap_or(0) + 1).to_string(),
                )
                .unwrap_or(*config_params.as_on_date());
                let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
                let cal_date = date_parser
                    .parse_opt(input_fields[37])
                    .unwrap_or(*config_params.as_on_date());
                let last_repr_date = if cal_date < appr_date {
                    appr_date
                } else {
                    cal_date
                }
                .format("%d-%m-%Y");
                output_line.push_str(&last_repr_date.to_string());
                output_line.push('|');
                continue;
            }
            if index == 38 {
                maturity_date = get_mat_date(input_fields.to_owned(), config_params);
                output_line.push_str(&get_next_rep_date(
                    config_params,
                    input_fields[38].to_string(),
                    input_fields[43].to_string(),
                    &mut rep_date_map,
                    &mut rep_day_month_map,
                    maturity_date.clone(),
                ));
                output_line.push('|');
                continue;
            }
            //A3 Calculation (Field used for FTP Impl)
            if index == 54 {
                let int_amt = input_fields[23].parse::<f64>().unwrap_or(0.0).abs()
                    + input_fields[24].parse::<f64>().unwrap_or(0.0).abs()
                    + input_fields[54].parse::<f64>().unwrap_or(0.0).abs();
                if input_fields[33].parse().unwrap_or(0) >= 4 {
                    output_line.push_str("0.0");
                } else {
                    output_line.push_str(&int_amt.to_string());
                }
                output_line.push('|');
                continue;
            }
            //A4 calculation (Field used for FTP Impl)
            if index == 55 {
                let a4;
                let unpd_prin_bal = input_fields[62].parse::<f64>().unwrap_or(0.0);
                let comp_amt = input_fields[64].parse::<f64>().unwrap_or(0.0);
                let mut subsidy_bal = input_fields[35].parse::<f64>().unwrap_or(0.0);
                let comp_freq = input_fields[65];
                if ["0", "00"].contains(&comp_freq) {
                    a4 = if subsidy_bal >= (unpd_prin_bal + comp_amt) {
                        0.0
                    } else {
                        (unpd_prin_bal + comp_amt) - subsidy_bal
                    };
                } else {
                    let loan_bal = input_fields[9].parse::<f64>().unwrap_or(0.0).abs();
                    let mut tot_npa_prov =
                        npa_master_val.npa_tot_prov.parse::<f64>().unwrap_or(0.0);
                    let old_bad_debt_ind = input_fields[33].parse().unwrap_or(0);
                    if subsidy_bal < 0.0 {
                        subsidy_bal = 0.0
                    };
                    if tot_npa_prov < 0.0 {
                        tot_npa_prov = 0.0
                    };
                    a4 = if old_bad_debt_ind <= 3 || !npa_master_map.contains_key(&npa_key) {
                        if loan_bal >= subsidy_bal {
                            loan_bal - subsidy_bal
                        } else {
                            0.0
                        }
                    } else {
                        if (subsidy_bal <= loan_bal) && (tot_npa_prov <= loan_bal) {
                            //loan_bal - tot_npa_prov - subsidy_bal
                            loan_bal - 0.0 - subsidy_bal
                        } else if (subsidy_bal <= loan_bal) && (tot_npa_prov > loan_bal) {
                            loan_bal - subsidy_bal
                        } else if (subsidy_bal > loan_bal) && (tot_npa_prov <= loan_bal) {
                            //loan_bal - tot_npa_prov
                            loan_bal - 0.0
                        } else {
                            0.0
                        }
                    };
                }
                output_line.push_str(&a4.to_string());
                output_line.push('|');
                continue;
            }
            //A5 calculation
            if index == 56 {
                let npa_date = datevalue_to_naive_date(input_fields[34])
                    .unwrap_or(*config_params.as_on_date());
                let npa_tenor = num_days_start_to_end(npa_date, *config_params.as_on_date());
                output_line.push_str(&npa_tenor.to_string());
                output_line.push('|');
                continue;
            }
            //A6 fields logic
            if index == 57 {
                output_line.push_str(&npa_master_val.npa_code);
                output_line.push('|');
                continue;
            }
            if index == 52 {
                let concate = format!("{}{}", input_fields[2], input_fields[19]);
                output_line.push_str(&concate);
                output_line.push('|');
                continue;
            }
            if index == 58 {
                output_line.push_str(&master_val[1]);
                output_line.push('|');
                continue;
            }
            if index == 59 {
                output_line.push_str(&master_val[2]);
                output_line.push('|');
                continue;
            }
            if index == 60 {
                let aorl_flag = master_val[0].chars().next().unwrap_or('A').to_string();
                output_line.push_str(&aorl_flag);
                output_line.push('|');
                continue;
            }
            if config_params
                .date_fields()
                .split('|')
                .any(|x| x == ((index + 1).to_string().as_str()))
            {
                let date = datevalue_to_naive_date(
                    &(input_fields[index].parse().unwrap_or(0) + 1).to_string(),
                )
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y");
                output_line.push_str(&date.to_string());
            } else {
                output_line.push_str(input_fields[index].trim());
            }
            if index + 1 == input_fields.len() {
                continue;
            } else {
                output_line.push('|');
            }
        }
        output_line.pop();
        //For ASONDATE
        output_line.push('|');
        output_line.push_str(&(config_params.as_on_date().format("%d-%m-%Y").to_string()));
        //For Maturity Date
        output_line.push('|');

        output_line.push_str(&(maturity_date));
        writeln!(writer, "{}", output_line).expect("Output Line can not be written");
        //logic for npa output file
        let npa_group = master_val[1].to_string();
        let npa_llg = master_val[2].to_string();
        let npa_compare = if input_fields[9] <= &npa_master_val.npa_cbs_bal.to_string() {
            &npa_master_val.npa_cbs_bal
        } else {
            input_fields[9]
        };
        let new_gnpa = npa_compare.parse().unwrap_or(0.0)
            - npa_master_val.npa_int_unc_prev.parse().unwrap_or(0.0)
            - npa_master_val.npa_auc_cur.parse().unwrap_or(0.0);
        //O2.21-O2.23+O2.17
        let total_provision = npa_master_val.npa_tot_prov.parse().unwrap_or(0.0)
            - npa_master_val.npa_auc.parse().unwrap_or(0.0)
            + npa_master_val.npa_tot_cf.parse().unwrap_or(0.0);
        //O2.24-O2.27

        let net_npa = npa_master_val.gnpa.parse().unwrap_or(0.0) - total_provision;
        //If O2.26 = 0 then 0 else O2.26-O2.27
        let new_net_npa = if new_gnpa == 0.0 {
            0.0
        } else {
            new_gnpa - total_provision
        };
        let npa_output_line=format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
       input_fields[0],
       input_fields[26],
       input_fields[9],
       input_fields[29],
       input_fields[33],
       npa_group,
       npa_llg,
       npa_master_val.npa_date,
       npa_master_val.npa_cif,
       npa_master_val.npa_acct_no,
       npa_master_val.npa_code,
       npa_master_val.npa_cbs_bal,
       npa_master_val.npa_int_unc_prev,
       npa_master_val.npa_claim_recd,
       npa_master_val.npa_sd_held,
       npa_master_val.npa_int_real,
       npa_master_val.npa_tot_cf,
       npa_master_val.npa_exc_prov,
       npa_master_val.npa_prov_sec,
       npa_master_val.npa_prov_unsec,
       npa_master_val.npa_tot_prov,
       npa_master_val.npa_auc,
       npa_master_val.npa_auc_cur,
       npa_master_val.gnpa,
       npa_compare,
       new_gnpa,
       total_provision,
       net_npa,
       new_net_npa);
        writeln!(npa_writer, "{}", npa_output_line).expect("NPA Output Line can not be written");
    }
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.", duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.output_file_path());
}
