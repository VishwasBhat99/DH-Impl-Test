use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader, Sheets};
use chrono::NaiveDate;
use core::str;
use hashbrown::HashMap;
use health_report::HealthReport;
use rbdate::{datevalue_to_naive_date, num_days_start_to_end};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::io::prelude::*;
use std::time::SystemTime;
pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    //Input File reading started
    log_debug!(log, "Tcfsl File reading started");
    let mut tcfsl_map: HashMap<String, String> = HashMap::new();
    let mut tcfsl_excel = open_workbook_auto(config_params.tcfsl_file_path()).expect(&format!(
        "Unable to open Tcfsl File. on path {}",
        config_params.tcfsl_file_path()
    ));
    check_sheet_name(
        config_params.tcfsl_file_path().to_string(),
        &config_params.tcfsl_sheet_name().to_string(),
        &tcfsl_excel,
    );
    if let Some(Ok(reader)) = tcfsl_excel.worksheet_range(config_params.tcfsl_sheet_name()) {
        for row in reader.rows().skip(0) {
            tcfsl_map.insert(
                row.get(0)
                    .expect(&format!(
                        "index out of bounds: the len is {} but the index is {} in file : {}",
                        row.len(),
                        0,
                        config_params.tcfsl_file_path(),
                    ))
                    .to_string(),
                row.get(50)
                    .expect(&format!(
                        "index out of bounds: the len is {} but the index is {} in file : {}",
                        row.len(),
                        50,
                        config_params.tcfsl_file_path(),
                    ))
                    .to_string(),
            );
        }
    }
    log_debug!(log, "Tcfsl File Reading Completed");
    //tcfsl File reading is Completed
    //Investment future File reading is started
    log_debug!(log, "Investment future File reading started");
    let mut investment_future_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut investment_future_excel =
        open_workbook_auto(config_params.investment_future_file_path()).expect(&format!(
            "Unable to open investment_future File. on path {}",
            config_params.investment_future_file_path()
        ));
    check_sheet_name(
        config_params.investment_future_file_path().to_string(),
        &config_params.investment_future_sheet_name().to_string(),
        &investment_future_excel,
    );
    if let Some(Ok(reader)) =
        investment_future_excel.worksheet_range(config_params.investment_future_sheet_name())
    {
        for row in reader.rows().skip(1) {
            investment_future_map
                .entry(
                    row.get(1)
                        .expect(&format!(
                            "index out of bounds: the len is {} but the index is {} in file : {}",
                            row.len(),
                            1,
                            config_params.investment_future_file_path()
                        ))
                        .to_string(),
                )
                .and_modify(|data| data.push(row[2..6].iter().map(|s| s.to_string()).collect()))
                .or_insert(vec![row[2..6].iter().map(|s| s.to_string()).collect()]);
        }
    }
    log_debug!(log, "Investment future File reading completed");
    //cred_gl_mapping_master File reading is started
    log_debug!(log, "cred_gl_mapping_master File reading started");
    let mut cred_gl_mapping_master_map: HashMap<String, String> = HashMap::new();
    let mut cred_gl_mapping_master_excel =
        open_workbook_auto(config_params.cred_gl_mapping_master_file_path()).expect(&format!(
            "Unable to open cred_gl_mapping_master File. on path {}",
            config_params.cred_gl_mapping_master_file_path()
        ));
    check_sheet_name(
        config_params.cred_gl_mapping_master_file_path().to_string(),
        &config_params
            .cred_gl_mapping_master_sheet_name()
            .to_string(),
        &cred_gl_mapping_master_excel,
    );
    if let Some(Ok(reader)) = cred_gl_mapping_master_excel
        .worksheet_range(config_params.cred_gl_mapping_master_sheet_name())
    {
        for row in reader.rows().skip(1) {
            cred_gl_mapping_master_map.insert(
                row.get(0)
                    .expect(&format!(
                        "index out of bounds: the len is {} but the index is {} in file : {}",
                        row.len(),
                        0,
                        config_params.cred_gl_mapping_master_file_path()
                    ))
                    .to_string()
                    .to_uppercase()
                    .trim()
                    .to_string(),
                row.get(1)
                    .expect(&format!(
                        "index out of bounds: the len is {} but the index is {} in file : {}",
                        row.len(),
                        1,
                        config_params.cred_gl_mapping_master_file_path()
                    ))
                    .to_string()
                    .trim()
                    .to_string(),
            );
        }
    }
    log_debug!(log, "cred_gl_mapping_master File reading completed");
    let mut alm_cred_manual_map: HashMap<String, String> = HashMap::new();
    let mut alm_cred_manual = open_workbook_auto(config_params.alm_credence_manual_file_path())
        .expect(&format!(
            "Unable to open alm_cred_manual File. on path {}",
            config_params.alm_credence_manual_file_path()
        ));
    check_sheet_name(
        config_params.alm_credence_manual_file_path().to_string(),
        &config_params.alm_credence_manual_sheet_name().to_string(),
        &alm_cred_manual,
    );
    if let Some(Ok(reader)) =
        alm_cred_manual.worksheet_range(config_params.alm_credence_manual_sheet_name())
    {
        for row in reader.rows().skip(1) {
            alm_cred_manual_map.insert(
                row.get(0)
                    .expect(&format!(
                        "index out of bounds: the len is {} but the index is {} in file : {}",
                        row.len(),
                        0,
                        config_params.alm_credence_manual_file_path(),
                    ))
                    .to_string(),
                "".to_string(),
            );
        }
    }
    log_debug!(log, "alm_cred_manual File Reading Completed");
    //Initialize the writer for TD
    let mut output_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(td_output_file) => td_output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` Error: `{}` ",
            config_params.output_file_path(),
            error,
        ),
    };
    let input_file = match new_buf_rdr(config_params.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input_file: `{}` Error {}",
            config_params.input_file_path(),
            error
        ),
    };
    //Read the Base Input file
    log_debug!(log, "Base Input File reading started");
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        tot_rec += 1;
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let input_fields = input_line
            .split("*|~")
            .map(|s| s.trim_matches('"'))
            .collect::<Vec<&str>>();
        if let Some(_) = alm_cred_manual_map.get(&input_fields[9].to_string()) {
            //Skip the record.
            log_debug!(
                log,
                "Record skipped:{}, line no:{}",
                input_fields[0],
                line_num + 1
            );
            continue;
        }
        succ_rec += 1;
        let default_prod_concat = "P".to_string();
        let default_gl_code = cred_gl_mapping_master_map.get("DEFAULT").expect(&format!(
            "DEFAULT condition is not present in {} file",
            config_params.cred_gl_mapping_master_file_path()
        ));
        let gl_code = cred_gl_mapping_master_map
            .get(
                &get_element_by_index(&input_fields, 13, line_num, config_params.input_file_path())
                    .to_uppercase()
                    .to_string(),
            )
            .unwrap_or(default_gl_code);
        let prod_concat = tcfsl_map
            .get(
                &get_element_by_index(&input_fields, 9, line_num, config_params.input_file_path())
                    .to_string(),
            )
            .unwrap_or(&default_prod_concat);

        let default_investment_future: Vec<Vec<String>> =
            vec![vec!["NA".to_string(), "NA".to_string(), "NA".to_string()]];
        let investment_future_vec = investment_future_map
            .get(
                &get_element_by_index(&input_fields, 9, line_num, config_params.input_file_path())
                    .to_string()
                    .trim()
                    .to_string(),
            )
            .unwrap_or(&default_investment_future);
        log_debug!(log, "Base Input File reading started");
        let mut nxt_int_pay_dt = NaiveDate::from_ymd(9999, 01, 01);
        let mut cf_date_vec: Vec<NaiveDate> = Vec::new();
        for investment_future_value in investment_future_vec {
            let cf_date = datevalue_to_naive_date(&investment_future_value[1])
                .unwrap_or(*config_params.as_on_date());
            if investment_future_value[0].trim().to_uppercase() == "INTEREST".to_string()
                && cf_date > *config_params.as_on_date()
                && cf_date < nxt_int_pay_dt
            {
                nxt_int_pay_dt = cf_date
            }

            cf_date_vec.push(cf_date);
        }
        if nxt_int_pay_dt == NaiveDate::from_ymd(9999, 01, 01) {
            nxt_int_pay_dt = *config_params.as_on_date()
        }
        let mut accrued_int_wrt = false;
        for investment_future_value in investment_future_vec {
            let mut accrued_int = 0.0;
            if !accrued_int_wrt {
                accrued_int = get_element_by_index(
                    &input_fields,
                    42,
                    line_num,
                    config_params.input_file_path(),
                )
                .parse::<f64>()
                .unwrap_or(0.0);
                accrued_int_wrt = true;
            }
            let principal_amount = if investment_future_value[0].trim().to_uppercase()
                == "REDEMPTION - PARTIAL".to_string()
                || investment_future_value[0].trim().to_uppercase() == "REDEMPTION"
            {
                investment_future_value[2].parse().unwrap_or(0.0)
            } else {
                0.0
            };
            let interest_amount =
                if investment_future_value[0].trim().to_uppercase() == "INTEREST".to_string() {
                    investment_future_value[2].parse().unwrap_or(0.0)
                } else {
                    0.0
                };
            //For write in output file
            let output_line_prin = &format!("{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|||||{}|||||{}|||||{}||||",
                get_element_by_index(&input_fields,9,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,8,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,13,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,19,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,12,line_num,config_params.input_file_path()),
                prod_concat,
                get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),
                datevalue_to_naive_date(&investment_future_value[1]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),
                "INR",
                get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,17,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                num_days_start_to_end(NaiveDate::parse_from_str(&get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date()),
                 NaiveDate::parse_from_str(&get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date())), 
                get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),
                principal_amount,
                interest_amount,
                "PRINCIPAL",
                "PRINCIPAL",
                get_element_by_index(&input_fields,21,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,11,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,9,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,20,line_num,config_params.input_file_path()),
                num_days_start_to_end(*config_params.as_on_date(),
                NaiveDate::parse_from_str(&get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date())),
                nxt_int_pay_dt.format("%d-%m-%Y"),
                gl_code,
                accrued_int,
                prod_concat,
            );

            let output_line_int = format!("{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|||||{}|||||{}|||||{}||||",
                get_element_by_index(&input_fields,9,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,8,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,13,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,19,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,12,line_num,config_params.input_file_path()),
                prod_concat,
                get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),
                datevalue_to_naive_date(&investment_future_value[1]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),
                "INR",
                get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,17,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                num_days_start_to_end(NaiveDate::parse_from_str(&get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date()),
                 NaiveDate::parse_from_str(&get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date())), 
                get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),
                principal_amount,
                interest_amount,
                "INTEREST",
                "INTEREST",
                get_element_by_index(&input_fields,21,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                get_element_by_index(&input_fields,11,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,9,line_num,config_params.input_file_path()),
                get_element_by_index(&input_fields,20,line_num,config_params.input_file_path()),
                num_days_start_to_end(*config_params.as_on_date(),
                NaiveDate::parse_from_str(&get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),"%%d-%m-%Y").unwrap_or(*config_params.as_on_date())),
                nxt_int_pay_dt.format("%d-%m-%Y"),
                gl_code,
                accrued_int,
                prod_concat,
            );
            if investment_future_value[0].trim().to_uppercase()
                == "REDEMPTION - PARTIAL".to_string()
                || investment_future_value[0].trim().to_uppercase() == "REDEMPTION"
            {
                writeln!(output_writer, "{}", output_line_prin)
                    .expect("Output Line can not be written");
            } else if investment_future_value[0] == "INTEREST".to_string() {
                writeln!(output_writer, "{}", output_line_int)
                    .expect("Output Line can not be written");
            } else {
                let output_line= format!("{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|||||{}|||||{}|||||{}||||",
                    get_element_by_index(&input_fields,9,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,8,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,13,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,19,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,12,line_num,config_params.input_file_path()),
                    prod_concat,
                    get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),
                    "INR",
                    get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                    get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                    get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                    get_element_by_index(&input_fields,17,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                    num_days_start_to_end(NaiveDate::parse_from_str(&get_element_by_index(&input_fields,15,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date()),
                     NaiveDate::parse_from_str(&get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),"%d-%m-%Y").unwrap_or(*config_params.as_on_date())), 
                    get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,22,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                    0.0,
                    "PRINCIPAL",
                    "PRINCIPAL",
                    get_element_by_index(&input_fields,21,line_num,config_params.input_file_path()).parse().unwrap_or(0.0),
                    get_element_by_index(&input_fields,11,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,9,line_num,config_params.input_file_path()),
                    get_element_by_index(&input_fields,20,line_num,config_params.input_file_path()),
                    num_days_start_to_end(*config_params.as_on_date(),
                    NaiveDate::parse_from_str(&get_element_by_index(&input_fields,16,line_num,config_params.input_file_path()),"%%d-%m-%Y").unwrap_or(*config_params.as_on_date())),
                    nxt_int_pay_dt.format("%d-%m-%Y"),
                    gl_code,
                    accrued_int,
                    prod_concat,
                );
                writeln!(output_writer, "{}", output_line).expect("Output Line can not be written");
            }
        }
    }
    log_debug!(log, "Base Input File Reading Completed");
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    log_debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.",
        duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
fn check_sheet_name(file_name: String, sheet_name: &String, excel_sheets: &Sheets) {
    if !excel_sheets.sheet_names().contains(&sheet_name.to_string()) {
        panic!(
            "sheet name {} is not present in {} : Available sheet names :{:?}",
            sheet_name,
            file_name,
            excel_sheets.sheet_names()
        )
    }
}
fn get_element_by_index<'a, T>(
    data: &'a Vec<T>,
    index: usize,
    row_no: usize,
    file_name: &str,
) -> &'a T {
    let ele = data.get(index).expect(&format!(
        "index out of bounds: the len is {} but the index is {} in file: {} on row_no:{}",
        data.len(),
        index,
        file_name,
        row_no,
    ));
    ele
}
