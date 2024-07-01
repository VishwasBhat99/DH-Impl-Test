use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader, Sheets};
use chrono::NaiveDate;
use core::str;
use hashbrown::HashMap;
use health_report::HealthReport;
use rbdate::datevalue_to_naive_date;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::io::prelude::*;
use std::time::SystemTime;
pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let date_parser = rbdate::DateParser::new(config_params.input_date_format().to_string(), false);
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
    //SC Dump File reading is started
    log_debug!(log, "SC Dump File reading started");
    let sc_dump_file = match new_buf_rdr(config_params.sc_dump_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found sc_dump_file: `{}` Error: `{}`.",
            config_params.sc_dump_file_path(),
            error
        ),
    };
    let mut sc_dump_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (line_num, lines) in sc_dump_file.lines().enumerate().skip(1) {
        let sc_dump_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.sc_dump_file_path(),
                line_num + 1,
                error
            ),
        };
        let sc_dump_fields: Vec<&str> = sc_dump_line.split("*|~").collect::<Vec<&str>>();
        sc_dump_map
            .entry(
                get_element_by_index(
                    &sc_dump_fields,
                    1,
                    config_params.sc_dump_file_path(),
                    &line_num,
                )
                .to_string(),
            )
            .and_modify(|data| {
                data.push(sc_dump_fields[4..6].iter().map(|s| s.to_string()).collect())
            })
            .or_insert(vec![sc_dump_fields[4..6]
                .iter()
                .map(|s| s.to_string())
                .collect()]);
    }

    log_debug!(log, "SC Dump File Reading is Completed");
    //Brnet File reading is started
    log_debug!(log, "Brnet File reading started");
    let brnet_file = match new_buf_rdr(config_params.brnet_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found brnet_file: `{}` Error: `{}`.",
            config_params.brnet_file_path(),
            error
        ),
    };
    let mut brnet_map: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in brnet_file.lines().enumerate().skip(1) {
        let brnet_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.brnet_file_path(),
                line_num + 1,
                error
            ),
        };
        let brnet_fields: Vec<&str> = brnet_line.split("*|~").collect::<Vec<&str>>();

        brnet_map.insert(
            get_element_by_index(&brnet_fields, 1, config_params.brnet_file_path(), &line_num)
                .trim()
                .to_string(),
            get_element_by_index(&brnet_fields, 2, config_params.brnet_file_path(), &line_num)
                .to_string()
                .trim()
                .parse::<f64>()
                .unwrap_or(0.0),
        );
    }
    log_debug!(log, "Brnet File Reading is Completed");
    // writeoff merged file reading started
    log_debug!(log, "writeoff merged File reading started");
    let writeoff_merged_file = match new_buf_rdr(config_params.writeoff_merged_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found brnet_file: `{}` Error: `{}`.",
            config_params.writeoff_merged_file_path(),
            error
        ),
    };
    let mut writeoff_merged_map: HashMap<String, (String, String)> = HashMap::new();
    for (line_num, lines) in writeoff_merged_file.lines().enumerate().skip(1) {
        let writeoff_merged_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.writeoff_merged_file_path(),
                line_num + 1,
                error
            ),
        };
        let writeoff_merged_field: Vec<&str> =
            writeoff_merged_line.split("*|~").collect::<Vec<&str>>();
        writeoff_merged_map.insert(
            get_element_by_index(
                &writeoff_merged_field,
                2,
                config_params.writeoff_merged_file_path(),
                &line_num,
            )
            .to_string()
            .trim()
            .to_string(),
            (
                get_element_by_index(
                    &writeoff_merged_field,
                    0,
                    config_params.writeoff_merged_file_path(),
                    &line_num,
                )
                .trim()
                .to_string(),
                get_element_by_index(
                    &writeoff_merged_field,
                    1,
                    config_params.writeoff_merged_file_path(),
                    &line_num,
                )
                .trim()
                .to_string(),
            ),
        );
    }
    log_debug!(log, "Writeoff merged File Reading is Completed");
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
        let input_fields = input_line.split("*|~").collect::<Vec<&str>>();
        let alm_dest_end_date = NaiveDate::parse_from_str(
            *get_element_by_index(&input_fields, 6, config_params.input_file_path(), &line_num),
            "%Y-%m-%d",
        )
        .expect(&format!(
            "{}: date can not be converted",
            get_element_by_index(&input_fields, 6, config_params.input_file_path(), &line_num)
        ));

        let alm_dest_princ_outstand =
            get_element_by_index(&input_fields, 9, config_params.input_file_path(), &line_num)
                .parse()
                .unwrap_or(0.0);
        if alm_dest_princ_outstand > 0.0 {
            succ_rec += 1;
            let default_asset_classification = "P".to_string();
            let asset_classification = tcfsl_map
                .get(
                    &get_element_by_index(
                        &input_fields,
                        1,
                        config_params.input_file_path(),
                        &line_num,
                    )
                    .to_string(),
                )
                .unwrap_or(&default_asset_classification);

            let default_sc_dump = vec![vec!["NA".to_string(), "NA".to_string()]];
            let sc_dump_vec = sc_dump_map
                .get(
                    &get_element_by_index(
                        &input_fields,
                        1,
                        config_params.input_file_path(),
                        &line_num,
                    )
                    .to_string()
                    .trim()
                    .to_string(),
                )
                .unwrap_or(&default_sc_dump);
            let accured_interest_from_inp = brnet_map
                .get(
                    &get_element_by_index(
                        &input_fields,
                        1,
                        config_params.input_file_path(),
                        &line_num,
                    )
                    .to_string()
                    .trim()
                    .to_string(),
                )
                .unwrap_or(&0.0);
            let def_division_tuple = &("Active".to_string(), "Active".to_string());
            let division = writeoff_merged_map
                .get(
                    &get_element_by_index(
                        &input_fields,
                        1,
                        config_params.input_file_path(),
                        &line_num,
                    )
                    .to_string()
                    .to_string(),
                )
                .unwrap_or(&def_division_tuple);

            let division_val;
            if division.0 == "BRNET" || division.0 == "BR NET" {
                division_val = division.1.clone();
            } else {
                division_val = "Active".to_string();
            }

            let mut cf_date_vec: Vec<NaiveDate> = Vec::new();
            for sc_dump_value in sc_dump_vec {
                let cf_date = NaiveDate::parse_from_str(&sc_dump_value[0], "%Y-%m-%d")
                    .unwrap_or(*config_params.as_on_date());
                cf_date_vec.push(cf_date);
            }
            cf_date_vec.sort();

            let mut last_reset_dt = *config_params.as_on_date();
            for date in cf_date_vec {
                if date > *config_params.as_on_date() {
                    last_reset_dt = date;
                    break;
                }
            }
            let mut accrued_int_wrt = false;
            for sc_dump_value in sc_dump_vec {
                let mut accrued_int = 0.0;
                if !accrued_int_wrt {
                    accrued_int = *accured_interest_from_inp;
                    accrued_int_wrt = true;
                }
                //For write in output file
                let output_line = format!("{}|{}|||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||||||{}|{}|{}||{}||||{}|{}||{}||{}|{}|{}|{}||{}|||||",
                get_element_by_index(&input_fields,69,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,1,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,17,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,45,config_params.input_file_path(),&line_num),
                date_parser.parse_opt(&get_element_by_index(&input_fields,0,config_params.input_file_path(),&line_num)).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse_opt(&get_element_by_index(&input_fields,0,config_params.input_file_path(),&line_num)).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse_opt(&get_element_by_index(&input_fields,6,config_params.input_file_path(),&line_num)).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                date_parser.parse_opt(&sc_dump_value[0]).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                asset_classification,
                get_element_by_index(&input_fields,64,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields, 68,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,11,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,9,config_params.input_file_path(),&line_num),
                "PRINCIPAL",
                get_element_by_index(&input_fields,9,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,8,config_params.input_file_path(),&line_num).parse::<f64>().unwrap_or(0.0)-get_element_by_index(&input_fields,9,config_params.input_file_path(),&line_num).parse::<f64>().unwrap_or(0.0),
                sc_dump_value[1],
                date_parser.parse_opt(&get_element_by_index(&input_fields,6,config_params.input_file_path(),&line_num)).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                last_reset_dt.format("%d-%m-%Y"),
                "Fixed",
                division_val,
                get_element_by_index(&input_fields,28,config_params.input_file_path(),&line_num),
                date_parser.parse_opt(&get_element_by_index(&input_fields,6,config_params.input_file_path(),&line_num)).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
                config_params.as_on_date().format("%d-%m-%Y"),
                get_element_by_index(&input_fields,26,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,42,config_params.input_file_path(),&line_num),
                get_element_by_index(&input_fields,71,config_params.input_file_path(),&line_num),
                "Individual",
                accrued_int,
            );
                writeln!(output_writer, "{}", output_line).expect("Output Line can not be written");
            }
        } else {
            log_debug!(
                log,
                "account_no: {} skipped because alm_dest_princ_outstand is less or equals to 0",
                get_element_by_index(&input_fields, 1, config_params.input_file_path(), &line_num)
            );
            continue;
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
    file_name: &str,
    row_no: &usize,
) -> &'a T {
    let ele = data.get(index).expect(&format!(
        "index out of bounds: the len is {} but the index is {} in file: {} at record_no: {}",
        data.len(),
        index,
        file_name,
        row_no
    ));
    ele
}
