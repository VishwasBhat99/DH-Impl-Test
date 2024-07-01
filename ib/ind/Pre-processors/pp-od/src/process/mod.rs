use crate::configuration_parameters::ConfigurationParameters;
use account::*;
use calamine::{open_workbook_auto, Reader};
use chrono::Datelike;
use derive::*;
use health_report::HealthReport;
use rbdate::{datevalue_to_naive_date, NaiveDate};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::{BufRead, Write};
use std::time::SystemTime;

mod account;
mod derive;

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
    let npa_file = match new_buf_rdr(config_params.npa_input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create read file: `{}` on location `{}` : {}",
            config_params.npa_input_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

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

    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut npa_input_map: HashMap<String, Vec<String>> = HashMap::new();
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
    for (line_num, lines) in npa_file.lines().enumerate() {
        let mut npa_data: Vec<String> = Vec::new();
        let npa_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.npa_input_file_path(),
                line_num + 1,
                error
            ),
        };
        let npa_fields = npa_line.split('|').collect::<Vec<&str>>();
        let npa_account_num = npa_fields[7].to_string();
        let npa_acc_key = &npa_account_num[0..(npa_account_num.len() - 1)];
        let npa_code = npa_fields[8].to_string();
        npa_code_map.insert(npa_acc_key.to_string(), npa_code.to_string());
        for val in npa_fields.iter() {
            npa_data.push(val.to_string());
        }
        npa_input_map.insert(npa_acc_key.to_string(), npa_data);
    }

    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let mut output_line = "".to_string();
        tot_rec += 1;
        if config_params
            .header_rows()
            .split('|')
            .any(|x| x == ((line_num + 1).to_string().as_str()))
        {
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
        let curr_bal = input_fields[9]
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
            .abs();
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
        let mut acct_open_date = *config_params.as_on_date();
        let mut last_rep_date = *config_params.as_on_date();
        let mut lst_fin_date = *config_params.as_on_date();
        let mut limit_expiry_date = config_params.as_on_date().to_string();
        //fields from KEY_1 to A10
        for index in 0..43 {
            if index == 33
                || index == 34
                || index == 40
                || index == 36
                || index == 37
                || index == 35
                || index == 39
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
                if index == 39 {
                    if input_fields[24].parse().unwrap_or(0) >= 4 {
                        output_line.push_str("0.0");
                    } else {
                        output_line.push_str(input_fields[index].trim());
                    }
                    output_line.push('|');
                    continue;
                }
                if index == 40 {
                    //let aol_flag = master_val[2].chars().nth(0).unwrap_or('A').to_string();
                    let month_start_date = NaiveDate::from_ymd_opt(
                        config_params.as_on_date().year(),
                        config_params.as_on_date().month(),
                        1,
                    )
                    .unwrap_or(*config_params.as_on_date());
                    let month_end_date = NaiveDate::from_ymd_opt(
                        config_params.as_on_date().year(),
                        config_params.as_on_date().month(),
                        rbdate::get_month_end_date(*config_params.as_on_date()).day(),
                    )
                    .unwrap_or(*config_params.as_on_date());
                    let closed_status = if ((acct_open_date >= month_start_date
                        && acct_open_date <= month_end_date)
                        || (last_rep_date >= month_start_date && last_rep_date <= month_end_date))
                        && (month_start_date <= lst_fin_date && lst_fin_date <= month_end_date)
                    {
                        "RMS"
                    } else if (input_fields[38].parse::<i32>().unwrap_or(0)
                        - input_fields[17].parse::<i32>().unwrap_or(0))
                        > 14
                    {
                        "PMT"
                    } else {
                        "NA"
                    }
                    .to_string();
                    output_line.push_str(&closed_status);
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
                    output_line.push_str(&limit_expiry_date.to_string());
                    output_line.push('|');
                }
                continue;
            }
            if index == 9 {
                output_line.push_str(&curr_bal.to_string());
                output_line.push('|');
                continue;
            }
            //Logic for OD-Tenor
            if index == 41 {
                let od_tenor = if input_fields[24].parse::<i64>().unwrap_or(0) >= 4 {
                    if npa_input_map.contains_key(&key_as_int.to_string()) {
                        let npa_date = npa_input_map
                            .get(&key_as_int.to_string())
                            .expect("Could Not find account in NPA Master File")[3]
                            .to_string();
                        get_od_tenor(config_params, npa_date, true)
                    } else {
                        get_od_tenor(config_params, "".to_string(), false)
                    }
                } else {
                    (input_fields[37].parse().unwrap_or(0) - input_fields[36].parse().unwrap_or(0))
                        .to_string()
                }
                .to_string();
                output_line.push_str(&od_tenor);
                output_line.push('|');
                continue;
            }
            if index == 12 {
                limit_expiry_date = get_limit_expiry_date(input_fields.to_owned(), config_params);
                output_line.push_str(&get_next_rep_date(
                    config_params,
                    input_fields[12].to_string(),
                    input_fields[23].to_string(),
                    input_fields[22].to_string(),
                    &mut rep_date_map,
                    &mut rep_day_month_map,
                    limit_expiry_date.clone(),
                ));
                output_line.push('|');
                continue;
            }

            if index == 15 {
                if [4, 5, 6, 7, 8]
                    .contains(&input_fields[24].to_string().parse::<i64>().unwrap_or(0))
                {
                    output_line.push('0');
                } else {
                    output_line.push_str(input_fields[index]);
                }
                output_line.push('|');
                continue;
            }

            if index == 42 {
                output_line.push_str(input_fields[index - 1]);
                output_line.push('|');
                continue;
            }

            if config_params
                .date_fields()
                .split('|')
                .any(|x| x == ((index + 1).to_string().as_str()))
            {
                let mut date = config_params.as_on_date().format("%d-%m-%Y");
                let modified_datevalue = input_fields[index].parse::<i64>().unwrap_or(0) + 1;
                if (0..=99999).contains(&modified_datevalue) {
                    date = datevalue_to_naive_date(&modified_datevalue.to_string())
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y");
                    output_line.push_str(&date.to_string());
                } else {
                    output_line.push_str(&date.to_string());
                }
                if index == 11 {
                    acct_open_date = NaiveDate::parse_from_str(&date.to_string(), "%d-%m-%Y")
                        .unwrap_or(*config_params.as_on_date());
                }
                if index == 13 {
                    last_rep_date = NaiveDate::parse_from_str(&date.to_string(), "%d-%m-%Y")
                        .unwrap_or(*config_params.as_on_date());
                }
                if index == 17 {
                    lst_fin_date = NaiveDate::parse_from_str(&date.to_string(), "%d-%m-%Y")
                        .unwrap_or(*config_params.as_on_date());
                }
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
            //product_code field derivation (A12)
            if index == 44 {
                let product_code = format!("{}{}", input_fields[3], input_fields[4]);
                output_line.push('|');
                output_line.push_str(&product_code);
                continue;
            }
            if index == 43 {
                let curr_bal = input_fields[9].parse::<f64>().unwrap_or(0.0).abs();
                let subsidy_bal = input_fields[48].parse::<f64>().unwrap_or(0.0);
                let subsidy_status = input_fields[45].parse().unwrap_or(0);
                let tot_npa_prov = if npa_input_map.contains_key(&key_as_int.to_string()) {
                    npa_input_map
                        .get(&key_as_int.to_string())
                        .expect("Could Not find account in NPA Master File")[49]
                        .to_string()
                        .parse()
                        .unwrap_or(0.0)
                } else {
                    0.0
                };
                let old_bad_debt_ind = input_fields[24].parse().unwrap_or(0);
                let mut a11 = curr_bal;
                if subsidy_status == 3 && old_bad_debt_ind <= 3 && subsidy_bal <= curr_bal {
                    a11 = curr_bal - subsidy_bal;
                } else if subsidy_status == 3
                    && old_bad_debt_ind >= 4
                    && curr_bal > subsidy_bal
                    && curr_bal > tot_npa_prov
                {
                    //a11 = curr_bal - subsidy_bal - tot_npa_prov;
                    a11 = curr_bal - subsidy_bal;
                } else if subsidy_status != 3 && old_bad_debt_ind <= 3 {
                    a11 = curr_bal;
                } else if subsidy_status != 3 && old_bad_debt_ind >= 4 && curr_bal > tot_npa_prov {
                    //a11 = curr_bal - tot_npa_prov;
                    a11 = curr_bal;
                }
                //Other cases are handled with default value i.e 0.0
                output_line.push('|');
                output_line.push_str(&a11.to_string());
                continue;
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

pub fn get_od_tenor(
    config_params: &ConfigurationParameters,
    npa_date: String,
    is_acct_npa: bool,
) -> String {
    //If Acct is found in NPA-File
    if is_acct_npa {
        rbdate::num_days_start_to_end(
            NaiveDate::parse_from_str(&npa_date, "%d-%m-%y").unwrap_or(*config_params.as_on_date()),
            *config_params.as_on_date(),
        )
    }
    //If Acct is not found in NPA-File
    else {
        rbdate::num_days_start_to_end(
            get_quarter_end_date(*config_params.as_on_date()),
            *config_params.as_on_date(),
        )
    }
    .to_string()
}

//Function which returns QuarterEnd-Date for corresponding As-On-Date
pub fn get_quarter_end_date(date: NaiveDate) -> NaiveDate {
    if vec![1, 2, 3].contains(&date.month()) {
        NaiveDate::from_ymd_opt(date.year() - 1, 12, 31).unwrap_or(date)
    } else if vec![4, 5, 6].contains(&date.month()) {
        NaiveDate::from_ymd_opt(date.year(), 3, 31).unwrap_or(date)
    } else if vec![7, 8, 9].contains(&date.month()) {
        NaiveDate::from_ymd_opt(date.year(), 6, 30).unwrap_or(date)
    } else {
        NaiveDate::from_ymd_opt(date.year(), 9, 30).unwrap_or(date)
    }
}
