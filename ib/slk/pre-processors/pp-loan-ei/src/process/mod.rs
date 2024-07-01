use self::npa_structs::{NPAData, RepData};
use crate::configuration_parameters::ConfigurationParameters;
use calamine::{open_workbook_auto, Reader};
use chrono::{Datelike, NaiveDate};
use health_report::HealthReport;
use rbdate::{
    datevalue_to_naive_date, incr_dt_by_mon_presrv_eom_checked, num_days_start_to_end, DateParser,
};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

mod npa_structs;
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

    let mut repricing_map: HashMap<String, RepData> = HashMap::new();
    let mut repricing_master_file = open_workbook_auto(config_params.repricing_master_file_path())
        .expect("Unable to open the repricing master xlsx file.");
    if let Some(Ok(repricing_file_reader)) =
        repricing_master_file.worksheet_range(&config_params.repricing_sheet_name())
    {
        for (row_no, row) in repricing_file_reader.rows().enumerate().skip(1) {
            let rep_data = RepData::new_from_xlsx(row);
            let concat = format!(
                "{}{}{}",
                rep_data.gl_code, rep_data.currency, rep_data.benchmark,
            );
            repricing_map.insert(concat, rep_data);
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
    let date_fields = config_params
        .date_fields()
        .split('|')
        .collect::<Vec<&str>>();
    let header_rows = config_params
        .header_rows()
        .split('|')
        .collect::<Vec<&str>>();
    for (line_num, lines) in input_file.lines().enumerate() {
        let mut output_line = "".to_string();
        tot_rec += 1;
        if header_rows.contains(&((line_num + 1).to_string().as_str())) {
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
        for index in 0..input_fields.len() {
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
                let default_rep = RepData {
                    gl_code: "".to_string(),
                    gl_desc: "".to_string(),
                    currency: "".to_string(),
                    benchmark: "".to_string(),
                    fix_float: "".to_string(),
                    repricing_date: "".to_string(),
                    repricing_bucket: "".to_string(),
                };
                maturity_date = get_mat_date(input_fields.to_owned(), config_params);
                let concat = format!(
                    "{}{}{}",
                    input_fields[29], input_fields[26], input_fields[43],
                );
                if repricing_map.contains_key(&concat) {
                    let rep_data = repricing_map.get(&concat).unwrap_or(&default_rep);
                    if !rep_data.repricing_date.is_empty() || !rep_data.repricing_bucket.is_empty()
                    {
                        let mut curr_repricing_date = *config_params.as_on_date();
                        if !rep_data.repricing_date.is_empty() {
                            let rep_day = rep_data.repricing_date.parse::<u32>().unwrap_or(0);
                            curr_repricing_date = NaiveDate::from_ymd(
                                curr_repricing_date.year(),
                                curr_repricing_date.month(),
                                rep_day,
                            );
                        }
                        if &rep_data.repricing_bucket == "M" {
                            curr_repricing_date =
                                rbdate::incr_dt_by_mon_presrv_eom(curr_repricing_date, 1)
                                    .expect("Cannot derive `next repricing date`.")
                        } else if &rep_data.repricing_bucket == "Q" {
                            let mut quater_frequency = NaiveDate::from_ymd(
                                curr_repricing_date.year(),
                                1,
                                curr_repricing_date.day(),
                            );
                            while curr_repricing_date >= quater_frequency {
                                quater_frequency =
                                    rbdate::incr_dt_by_mon_presrv_eom(quater_frequency, 3)
                                        .expect("Cannot derive `next repricing date`.")
                            }
                            curr_repricing_date = quater_frequency;
                        }
                        let next_repricing_date =
                            NaiveDate::parse_from_str(&curr_repricing_date.to_string(), "%Y-%m-%d")
                                .unwrap_or(curr_repricing_date)
                                .format("%d-%m-%Y")
                                .to_string();
                        output_line.push_str(&next_repricing_date);
                    } else {
                        output_line.push_str(&maturity_date);
                    }
                } else {
                    output_line.push_str(&maturity_date);
                }
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
            //A4 calculation (Field used for FTP Impl) //NOT REQUIRED FOR SLK
            if index == 55 {
                let a4;
                let loan_bal = input_fields[9].parse::<f64>().unwrap_or(0.0).abs();
                let mut subsidy_bal = input_fields[35].parse::<f64>().unwrap_or(0.0);
                let mut tot_npa_prov = npa_master_val.npa_tot_prov.parse::<f64>().unwrap_or(0.0);
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
                //}
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
            if date_fields.contains(&((index + 1).to_string().as_str())) {
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
pub fn get_mat_date(input_data: Vec<&str>, config_params: &ConfigurationParameters) -> String {
    let apprv_date_value = input_data[14].parse::<i64>().unwrap_or(0) + 1;
    let apprv_date = datevalue_to_naive_date(&apprv_date_value.to_string())
        .unwrap_or(*config_params.as_on_date());
    let loan_trm = input_data[20]
        .to_string()
        .parse::<usize>()
        .expect("unable to parse LOAN_TRM");
    let mut mat_date = incr_dt_by_mon_presrv_eom_checked(apprv_date, loan_trm)
        .unwrap_or(*config_params.as_on_date());

    //if loan term is in days values
    if input_data[61] == "D" {
        mat_date = datevalue_to_naive_date(&(apprv_date_value + loan_trm as i64).to_string())
            .unwrap_or(*config_params.as_on_date());
    }
    mat_date.format("%d-%m-%Y").to_string()
}
