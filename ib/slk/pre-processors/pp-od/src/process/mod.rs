use crate::configuration_parameters::ConfigurationParameters;
use calamine::{open_workbook_auto, DataType, Reader};
use chrono::Datelike;
use health_report::HealthReport;
use rbdate::*;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_params.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut map_npa_excel = open_workbook_auto(config_params.npa_input_file_path())
        .expect("Unable to open Mapping NPA File.");

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
    let output_path_split = config_params
        .output_file_path()
        .split('.')
        .collect::<Vec<&str>>();
    let npa_output_directory = format!("{}_npa.txt", output_path_split[0]);
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
    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut rep_gl_map: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut npa_input_map: HashMap<String, Vec<DataType>> = HashMap::new();
    let mut npa_code_map: HashMap<String, String> = HashMap::new();
    let mut master_excel = open_workbook_auto(config_params.master_file_path())
        .expect("Unable to open Mapping Master File.");

    if let Some(Ok(reader)) = master_excel.worksheet_range(config_params.master_sheet_name()) {
        for row in reader.rows().skip(1) {
            let gl_acc_no = row[0].to_string();
            let class = row[2].to_string();
            let group = row[3].to_string();
            let clsfn = row[4].to_string();
            master_map.insert(gl_acc_no, [group, clsfn, class].to_vec());
        }
    }
    //reading reprising.xlsx:
    let mut reprising_excel = open_workbook_auto(config_params.repricing_file_path())
        .expect("Unable to open Repricing File.");
    if let Some(Ok(reader)) = reprising_excel.worksheet_range(config_params.repricing_sheet_name())
    {
        for row in reader.rows().skip(1) {
            let curr_gl_code = row[0].to_string();
            let ccy = row[2].to_string();
            let status = row[3].to_string();
            let repricing_dt = row[4].to_string();
            let repricing_bucket = row[5].to_string();

            rep_gl_map.insert(
                (curr_gl_code, ccy.clone()),
                [ccy, status, repricing_dt, repricing_bucket].to_vec(),
            );
        }
    }
    if let Some(Ok(reader)) = map_npa_excel.worksheet_range(config_params.npa_sheet_name()) {
        for row in reader.rows().skip(0) {
            let npa_account_num = row[7].to_string();
            let npa_acc_key = &npa_account_num[0..(npa_account_num.len() - 1)];
            let npa_code = row[8].to_string();
            npa_code_map.insert(npa_acc_key.to_string(), npa_code.to_string());
            npa_input_map.insert(npa_acc_key.to_string(), row.to_vec());
        }
    }

    let mut limit_exp_dt = config_params.as_on_date().to_string();
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let mut output_line = "".to_string();
        tot_rec += 1;
        if header_rows.contains(&((line_num + 1).to_string().as_str())) {
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
        //Get Gl Code from GL CLASS CODE
        let gl_class_code = input_fields[20].trim();
        let mut gl_code = "NA";
        if gl_class_code.len() < 18 {
            debug!(
                log,
                "(GL_class_code len is less then 18) while getting GL Code for Account - {}",
                input_fields[0]
            );
        } else {
            gl_code = &gl_class_code[8..18];
        }
        let curr_bal = input_fields[9];
        // Get key to extract npa code from npa file
        let key_string = input_fields[0].trim();
        let key = &key_string[3..key_string.len()];
        if key.parse::<i64>().unwrap_or(0) == 0 {
            continue;
        }
        let key_as_int = key.parse::<i64>().unwrap_or(0);
        let ccy = input_fields[6];
        let npa_class = input_fields[24];
        input_map.insert(
            key_as_int.to_string(),
            [
                gl_code.to_string(),
                curr_bal.to_string(),
                ccy.to_string(),
                npa_class.to_string(),
                gl_class_code.to_string(),
            ]
            .to_vec(),
        );
        //fields from KEY_1 to A10
        for index in 0..43 {
            if index == 33
                || index == 34
                || index == 40
                || index == 36
                || index == 37
                || index == 35
                || index == 38
            {
                let def_master_val: Vec<String> =
                    ["NONE".to_string(), "NONE".to_string(), "ASSET".to_string()].to_vec();
                let master_val = master_map.get(gl_code).unwrap_or(&def_master_val);
                let input_npa_code = npa_code_map
                    .get(&key_as_int.to_string())
                    .unwrap_or(&"NONE".to_string())
                    .to_string();
                if index == 33 {
                    output_line.push_str(&master_val[0]);
                    output_line.push('|');
                }
                if index == 34 {
                    output_line.push_str(&master_val[1]);
                    output_line.push('|');
                }
                if index == 35 {
                    output_line.push_str(&input_npa_code);
                    output_line.push('|');
                }
                if index == 40 {
                    let aol_flag = master_val[2].chars().nth(0).unwrap_or('A').to_string();
                    output_line.push_str(&aol_flag);
                    output_line.push('|');
                }
                if index == 36 {
                    let mut limit_set_datevalue = input_fields[36].parse::<i64>().unwrap_or(0) + 1;
                    if limit_set_datevalue <= 0 || limit_set_datevalue >= 99999 {
                        limit_set_datevalue = input_fields[11].parse::<i64>().unwrap_or(0) + 1;
                    }
                    let limit_set_date = datevalue_to_naive_date(&limit_set_datevalue.to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y");
                    output_line.push_str(&limit_set_date.to_string());
                    output_line.push('|');
                }
                if index == 37 {
                    let limit_expiry_datevalue = input_fields[37].parse::<i64>().unwrap_or(0) + 1;
                    if limit_expiry_datevalue <= 0 || limit_expiry_datevalue >= 99999 {
                        let limit_expiry_date = config_params
                            .as_on_date()
                            .succ_opt()
                            .unwrap()
                            .format("%d-%m-%Y");
                        limit_exp_dt = limit_expiry_datevalue.to_string();
                        output_line.push_str(&limit_expiry_date.to_string());
                        output_line.push('|');
                    } else {
                        let limit_expiry_date =
                            datevalue_to_naive_date(&limit_expiry_datevalue.to_string())
                                .unwrap_or(*config_params.as_on_date())
                                .format("%d-%m-%Y");
                        limit_exp_dt = limit_expiry_date.to_string();
                        output_line.push_str(&limit_expiry_date.to_string());
                        output_line.push('|');
                    }
                }
                if index == 38 {
                    let def_input_val: Vec<String> = [
                        "NONE".to_string(),
                        "NONE".to_string(),
                        "NONE".to_string(),
                        "NONE".to_string(),
                    ]
                    .to_vec();

                    let key = (gl_code.to_string(), input_fields[6].to_string());

                    if rep_gl_map.contains_key(&key) {
                        let gl_values = rep_gl_map.get(&key).unwrap_or(&def_input_val);
                        let curr = gl_values[0].clone();
                        let rep_dt = gl_values[2].parse::<u32>().unwrap_or(0);
                        let rep_bucket = gl_values[3].clone();

                        let input_concat = format!("{}{}", gl_code, input_fields[6],);
                        let rep_concat = format! {
                            "{}{}",
                            gl_code,curr,
                        };
                        if input_concat == rep_concat {
                            if rep_bucket == "M" {
                                let new_date = get_next_month_date(config_params.as_on_date,rep_dt);
                                let push_date = new_date.format("%d-%m-%Y").to_string();
                                output_line.push_str(&push_date);
                            } else if rep_bucket == "Q" {
                                let new_date = get_quarter_end_date(config_params.as_on_date,rep_dt);
                                let push_date = new_date.format("%d-%m-%Y").to_string();
                                output_line.push_str(&push_date);
                            } else {
                                output_line.push_str(&limit_exp_dt.to_string());
                            }
                        } else {
                            output_line.push_str(&limit_exp_dt.to_string());
                        }
                    } else {
                        output_line.push_str(&limit_exp_dt.to_string());
                    }
                    output_line.push('|');
                }
                continue;
            }
            if index == 39 {
                let product_code = format!("{}{}", input_fields[3], input_fields[4]);
                output_line.push_str(&product_code);
                output_line.push('|');
                continue;
            }
            if index == 41 {
                let od_tenor = (input_fields[37].parse().unwrap_or(0)
                    - input_fields[36].parse().unwrap_or(0))
                .to_string();
                output_line.push_str(&od_tenor);
                output_line.push('|');
                continue;
            }

            if date_fields.contains(&((index + 1).to_string().as_str())) {
                let modified_date = input_fields[index].parse::<i64>().unwrap_or(0) + 1;
                let date = datevalue_to_naive_date(&modified_date.to_string())
                    .unwrap_or(*config_params.as_on_date())
                    .format("%d-%m-%Y");
                output_line.push_str(&date.to_string());
            } else {
                output_line.push_str(input_fields[index]);
            }
            //checking for last fields
            output_line.push('|');
        }
        //For GLCODE
        output_line.push_str(gl_code);
        //For INT_RATE
        output_line.push('|');
        if !input_fields[18].is_empty() {
            output_line.push_str(input_fields[18]);
        } else if !input_fields[15].is_empty() {
            output_line.push_str(input_fields[15]);
        } else {
            output_line.push_str(input_fields[18]);
        }
        //For CURR_BAL_LCY
        output_line.push('|');
        output_line.push_str("");
        //For As_on_Date
        output_line.push('|');
        output_line.push_str(&(config_params.as_on_date().format("%d-%m-%Y").to_string()));
        //input fields from A11 to A16
        for index in 43..input_fields.len() {
            //Account status
            if index == 43 {
                output_line.push('|');
                if input_fields[2] == "07" {
                    output_line.push('C');
                } else {
                    output_line.push_str("");
                }
            } else {
                output_line.push('|');
                output_line.push_str(input_fields[index]);
            }
        }
        writeln!(writer, "{}", output_line).expect("Output Line can not be written");
    }
    let mut npa_output_line = "".to_string();
    for (key, val) in npa_input_map.iter() {
        let npa_fields = val;
        let mut npa_compare = 0.0;
        let mut total_prov = 0.0;
        let mut new_gnpa = 0.0;
        let npa_account_num = &npa_fields[7].to_string();
        let npa_acc_key = &npa_account_num[0..(npa_account_num.len() - 1)];
        let def_input_val: Vec<String> = [
            "NONE".to_string(),
            "NONE".to_string(),
            "NONE".to_string(),
            "NONE".to_string(),
            "NONE".to_string(),
        ]
        .to_vec();
        if input_map.contains_key(npa_acc_key) {
            let input_val = input_map.get(npa_acc_key).unwrap_or(&def_input_val);
            npa_output_line.push_str(key);
            npa_output_line.push('|');
            npa_output_line.push_str(&input_val[2]);
            npa_output_line.push('|');
            npa_output_line.push_str(&input_val[1]);
            npa_output_line.push('|');
            npa_output_line.push_str(&input_val[4]);
            npa_output_line.push('|');
            npa_output_line.push_str(&input_val[3]);
            npa_output_line.push('|');

            let def_master_val: Vec<String> =
                ["NONE".to_string(), "NONE".to_string(), "ASSET".to_string()].to_vec();
            let gl_code = &input_val[0];
            let master_val = master_map.get(gl_code).unwrap_or(&def_master_val);
            npa_output_line.push_str(&master_val[0]);
            npa_output_line.push('|');
            npa_output_line.push_str(&master_val[1]);
            npa_output_line.push('|');
            let npa_date =
                NaiveDate::parse_from_str(npa_fields[3].to_string().as_str(), "%d-%m-%y")
                    .unwrap_or(config_params.as_on_date);
            let data_line = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
                npa_date.format("%d-%m-%Y"),
                npa_fields[6],
                npa_fields[7],
                npa_fields[8],
                npa_fields[10],
                npa_fields[30],
                npa_fields[35],
                npa_fields[36],
                npa_fields[37],
                npa_fields[41],
                npa_fields[43],
                npa_fields[47],
                npa_fields[48],
                npa_fields[49],
                npa_fields[50],
                npa_fields[52],
                npa_fields[73]
            );
            npa_output_line.push_str(&data_line);
            let curr_bal = &input_val[1].parse::<f64>().unwrap_or(0.0);
            let npa_cbs_bal = &npa_fields[10].to_string().parse::<f64>().unwrap_or(0.0);

            if curr_bal.abs() <= npa_cbs_bal.abs() {
                npa_output_line.push_str(&npa_cbs_bal.abs().to_string());
                npa_output_line.push('|');
                npa_compare = *npa_cbs_bal;
            } else {
                npa_output_line.push_str(&curr_bal.abs().to_string());
                npa_output_line.push('|');
                npa_compare = *curr_bal;
            }

            new_gnpa = npa_compare.abs()
                - npa_fields[30].to_string().parse::<f64>().unwrap_or(0.0)
                - npa_fields[52].to_string().parse::<f64>().unwrap_or(0.0);
            npa_output_line.push_str(&new_gnpa.to_string());
            npa_output_line.push('|');

            total_prov = npa_fields[49].to_string().parse::<f64>().unwrap_or(0.0)
                - npa_fields[52].to_string().parse::<f64>().unwrap_or(0.0)
                + npa_fields[41].to_string().parse::<f64>().unwrap_or(0.0);
            npa_output_line.push_str(&total_prov.to_string());
            npa_output_line.push('|');

            let net_npa = npa_fields[73].to_string().parse::<f64>().unwrap_or(0.0) - total_prov;
            npa_output_line.push_str(&net_npa.to_string());
            npa_output_line.push('|');

            if new_gnpa == 0.0 {
                npa_output_line.push_str("0.0");
                npa_output_line.push('\n');
            } else {
                let new_net_npa = new_gnpa - total_prov;
                npa_output_line.push_str(&new_net_npa.to_string());
                npa_output_line.push('\n');
            }

            input_map.remove(npa_acc_key);
            continue;
        }
    }
    for (key, val) in &input_map {
        let as_on_date = config_params.as_on_date().format("%d-%m-%Y");
        let def_master_val: Vec<String> =
            ["NONE".to_string(), "NONE".to_string(), "ASSET".to_string()].to_vec();
        let gl_code = &val[0];
        let npa_compare = val[1].parse::<f64>().unwrap_or(0.0).abs();
        let master_val = master_map.get(gl_code).unwrap_or(&def_master_val);
        let other_od_account_line = format!("{}|{}|{}|{}|{}|{}|{}|{}|0|NA|0|0.0|0.0|0.0|0.0|0.0|0.0|0.0|0.0|0.0|0.0|NA|0.0|0.0|{}|{}|0.0|0.0|{}",
        key,
        val[2],
        val[1],
        val[4],
        val[3],
        master_val[0],
        master_val[1],
        as_on_date,
        npa_compare,
        val[1],
        val[1]
        );
        npa_output_line.push_str(other_od_account_line.as_str());
        npa_output_line.push('\n');
    }
    write!(npa_writer, "{}", npa_output_line).expect("Output Line can not be written");
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

//Function which returns QuarterEnd-Date for corresponding As-On-Date
pub fn get_quarter_end_date(date: NaiveDate,mut rep_dt:u32) -> NaiveDate {
    if rep_dt != 1 {
        rep_dt = date.day();
    }

    let mut next_date = date;
    if vec![1, 2, 3].contains(&date.month()) {
        next_date = NaiveDate::from_ymd_opt(date.year(), 4, 1).unwrap_or(date)
    } else if vec![4, 5, 6].contains(&date.month()) {
        next_date = NaiveDate::from_ymd_opt(date.year(), 7, 1).unwrap_or(date)
    } else if vec![7, 8, 9].contains(&date.month()) {
        next_date = NaiveDate::from_ymd_opt(date.year(), 10, 1).unwrap_or(date)
    } else if vec![10, 11, 12].contains(&date.month()) {
        next_date = NaiveDate::from_ymd_opt(date.year()+1, 1, 1).unwrap_or(date)
    } else {
        next_date = NaiveDate::from_ymd_opt(date.year(), 9, 1).unwrap_or(date)
    }
    
    let no_of_days = get_days_from_month(next_date) as u32;
    if rep_dt <= no_of_days {
        NaiveDate::from_ymd_opt(next_date.year(), next_date.month(), rep_dt).unwrap_or(date)
    }
    else{
        get_month_end_date(next_date)
    }
}

pub fn get_next_month_date(date:NaiveDate,mut rep_dt:u32) -> NaiveDate {
    let month = date.month();
    if rep_dt != 1 {
        rep_dt = date.day();
    }

    let mut next_date = date;
    if vec![12].contains(&date.month()) {
        next_date=NaiveDate::from_ymd_opt(date.year()+1, 1, rep_dt).unwrap_or(date);
    }
    else{
        next_date=NaiveDate::from_ymd_opt(date.year(), month+1, rep_dt).unwrap_or(date);
    }

    if next_date == date {
        get_month_end_date(incr_dt_by_mon_presrv_eom_checked(next_date, 1).unwrap_or(date))
    }
    else{
        next_date
    }
}
