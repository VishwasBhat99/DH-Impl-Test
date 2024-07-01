use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader, Sheets};
use chrono::NaiveDate;
use core::str;
use hashbrown::HashMap;
use health_report::HealthReport;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;

    log_debug!(log, "WriteOff file reading started");
    let mut writeoff_map: HashMap<String, String> = HashMap::new();
    let writeoff_file = match new_buf_rdr(config_params.writeoff_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found writeoff_file: `{}` Error {}",
            config_params.writeoff_file_path(),
            error
        ),
    };

    for (line_num, lines) in writeoff_file.lines().enumerate().skip(0) {
        tot_rec += 1;
        let writeoff_line = match lines {
            Ok(writeoff_line) => writeoff_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.writeoff_file_path(),
                line_num + 1,
                error
            ),
        };
        let writeoff_fields = writeoff_line.split("*|~").collect::<Vec<&str>>();
        succ_rec += 1;
        if get_element_by_index(
            &writeoff_fields,
            0,
            line_num,
            config_params.writeoff_file_path(),
        )
        .to_uppercase()
        .replace('"', "")
            == "BANCS"
        {
            writeoff_map.insert(
                get_element_by_index(
                    &writeoff_fields,
                    2,
                    line_num,
                    config_params.writeoff_file_path(),
                )
                .trim()
                .replace('"', "")
                .to_string(),
                get_element_by_index(
                    &writeoff_fields,
                    1,
                    line_num,
                    config_params.writeoff_file_path(),
                )
                .trim()
                .replace('"', "")
                .to_string(),
            );
        }
    }
    log_debug!(log, "WriteOff file Reading Completed");

    log_debug!(log, "product_fixed_floating_file reading started");
    let mut product_fixed_floating_map: HashMap<String, String> = HashMap::new();
    let mut product_fixed_map: HashMap<String, (String, String)> = HashMap::new();
    let product_fixed_floating_file =
        match new_buf_rdr(config_params.product_fixed_floating_file_path()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found product_fixed_floating_file: `{}` Error {}",
                config_params.product_fixed_floating_file_path(),
                error
            ),
        };

    for (line_num, lines) in product_fixed_floating_file.lines().enumerate().skip(0) {
        tot_rec += 1;
        let product_fixed_floating_line = match lines {
            Ok(product_fixed_floating_line) => product_fixed_floating_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.product_fixed_floating_file_path(),
                line_num + 1,
                error
            ),
        };
        let product_fixed_floating_fields = product_fixed_floating_line
            .split("*|~")
            .collect::<Vec<&str>>();
        succ_rec += 1;
        product_fixed_floating_map.insert(
            get_element_by_index(
                &product_fixed_floating_fields,
                3,
                line_num,
                config_params.product_fixed_floating_file_path(),
            )
            .trim()
            .replace('"', "")
            .to_string(),
            get_element_by_index(
                &product_fixed_floating_fields,
                7,
                line_num,
                config_params.product_fixed_floating_file_path(),
            )
            .trim()
            .to_string(),
        );
        let product_id = get_element_by_index(
            &product_fixed_floating_fields,
            0,
            line_num,
            config_params.product_fixed_floating_file_path(),
        );
        let sub_product_name = get_element_by_index(
            &product_fixed_floating_fields,
            1,
            line_num,
            config_params.product_fixed_floating_file_path(),
        );
        let fixed_floating = get_element_by_index(
            &product_fixed_floating_fields,
            2,
            line_num,
            config_params.product_fixed_floating_file_path(),
        );
        product_fixed_map.insert(
            product_id.to_string(),
            (sub_product_name.to_string(), fixed_floating.to_string()),
        );
    }
    log_debug!(log, "product_fixed_floating_file Reading Completed");
    log_debug!(log, "Tcfsl File reading started");
    let mut tcfsl_map: HashMap<String, (String, String)> = HashMap::new();
    let mut tcfsl_excel: Sheets =
        open_workbook_auto(config_params.tcfsl_file_path()).expect(&format!(
            "Unable to open Tcfsl File. on path {}",
            config_params.tcfsl_file_path()
        ));
    check_sheet_name(
        config_params.tcfsl_file_path().to_string(),
        &config_params.tcfsl_sheet_name().to_string(),
        &tcfsl_excel,
    );

    if let Some(Ok(reader)) = tcfsl_excel.worksheet_range(config_params.tcfsl_sheet_name()) {
        for row in reader.rows().skip(1) {
            let schm_code = row
                .get(12)
                .expect(&format!(
                    "unable to get the index {} from file: {}",
                    12,
                    config_params.tcfsl_file_path(),
                ))
                .to_string();
            tcfsl_map.insert(
                row.get(0)
                    .expect(&format!(
                        "unable to get the index {} from file: {}",
                        0,
                        config_params.tcfsl_file_path(),
                    ))
                    .to_string(),
                (
                    row.get(50)
                        .expect(&format!(
                            "unable to get the index {} from file: {}",
                            50,
                            config_params.tcfsl_file_path(),
                        ))
                        .to_string(),
                    schm_code,
                ),
            );
        }
    }
    log_debug!(log, "Tcfsl File Reading Completed");

    let mut repayment_schedule_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    log_debug!(log, "Repayment Schedule File reading started");
    let repayment_schedule_file = match new_buf_rdr(config_params.repayment_schedule_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found repayment_schedule_file: `{}` Error {}",
            config_params.repayment_schedule_file_path(),
            error
        ),
    };

    for (line_num, lines) in repayment_schedule_file.lines().enumerate().skip(1) {
        tot_rec += 1;
        let repayment_schedule_line = match lines {
            Ok(repayment_schedule_line) => repayment_schedule_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.repayment_schedule_file_path(),
                line_num + 1,
                error
            ),
        };
        let repayment_schedule_fields = repayment_schedule_line.split("*|~").collect::<Vec<&str>>();
        succ_rec += 1;
        repayment_schedule_map
            .entry(
                get_element_by_index(
                    &repayment_schedule_fields,
                    0,
                    line_num,
                    config_params.repayment_schedule_file_path(),
                )
                .trim()
                .replace('"', "")
                .to_string(),
            )
            .and_modify(|data| {
                data.push(
                    repayment_schedule_fields[1..4]
                        .iter()
                        .map(|s| s.trim().replace('"', "").to_string())
                        .collect(),
                )
            })
            .or_insert(vec![repayment_schedule_fields[1..4]
                .iter()
                .map(|s| s.trim().replace('"', "").to_string())
                .collect()]);
    }
    log_debug!(log, "Repayment Schedule File reading Completed");

    let mut product_entity_mapping_map: HashMap<String, String> = HashMap::new();
    log_debug!(log, "Product Entity Mapping File reading started");
    let product_entity_mapping_file =
        match new_buf_rdr(config_params.product_entity_mapping_file_path()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found product_entity_mapping_file: `{}` Error {}",
                config_params.product_entity_mapping_file_path(),
                error
            ),
        };
    let company_codes = config_params.company_code();
    let mut company_code_vec: Vec<&str> = company_codes.split(',').collect();
    for (line_num, lines) in product_entity_mapping_file.lines().enumerate().skip(1) {
        tot_rec += 1;
        let product_entity_mapping_line = match lines {
            Ok(product_entity_mapping_line) => product_entity_mapping_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.product_entity_mapping_file_path(),
                line_num + 1,
                error
            ),
        };
        let product_entity_mapping_fields = product_entity_mapping_line
            .split(",")
            .collect::<Vec<&str>>();
        let company_code = get_element_by_index(
            &product_entity_mapping_fields,
            2,
            line_num,
            &config_params.product_entity_mapping_file_path,
        );
        if !company_code_vec.contains(&&company_code) {
            continue;
        }
        succ_rec += 1;
        product_entity_mapping_map.insert(
            get_element_by_index(
                &product_entity_mapping_fields,
                0,
                line_num,
                &config_params.product_entity_mapping_file_path,
            )
            .replace('"', "")
            .to_string(),
            get_element_by_index(
                &product_entity_mapping_fields,
                2,
                line_num,
                &config_params.product_entity_mapping_file_path,
            )
            .to_string()
            .replace('"', ""),
        );
    }
    log_debug!(log, "Product Entity Mapping File reading completed");

    let mut product_id_map: HashMap<String, bool> = HashMap::new();
    log_debug!(log, "Product Id File reading started");
    let product_id_file = match new_buf_rdr(config_params.product_id_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found product_id_file: `{}` Error {}",
            config_params.product_id_file_path(),
            error
        ),
    };

    for (line_num, lines) in product_id_file.lines().enumerate().skip(1) {
        let product_id_line = match lines {
            Ok(product_id_line) => product_id_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.product_id_file_path(),
                line_num + 1,
                error
            ),
        };
        product_id_map.insert(product_id_line, true);
    }
    log_debug!(log, "Product Id File reading completed");

    // Initialize the writer for output file
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

    // Read the Base Input file
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

        if !product_id_map.contains_key(
            product_entity_mapping_map
                .get(input_fields[1])
                .unwrap_or(&"NA".to_string()),
        ) {
            log_debug!(
                log,
                "account_number: {} is skipping because product_code: {} is not available in product_entity_mapping_file and product_id file",
                input_fields[15],
                input_fields[1]
            );
            continue;
        }
        succ_rec += 1;

        let loan_id =
            get_element_by_index(&input_fields, 0, line_num, config_params.input_file_path())
                .to_string()
                .trim()
                .replace('"', "")
                .to_string();
        let default_user_def_stats = "P".to_string();
        let def_usr_status = (
            default_user_def_stats.clone(),
            default_user_def_stats.clone(),
        );
        let trimed_loan_id: &str = loan_id
            .trim_start_matches('0')
            .trim()
            .trim_matches(|pat| pat == ' ' || pat == '"');

        let (user_def_stats, schm_code) = tcfsl_map.get(trimed_loan_id).unwrap_or(&def_usr_status);
        let mat_date: NaiveDate =
            NaiveDate::parse_from_str(&input_fields[6].replace("00:00:00", "").trim(), "%Y-%m-%d")
                .unwrap_or(*config_params.as_on_date());

        let default_repayment_schedule: Vec<Vec<String>> =
            vec![vec!["NA".to_string(), "NA".to_string(), "NA".to_string()]];

        let repayment_schedule_vec = repayment_schedule_map
            .get(&loan_id)
            .unwrap_or(&default_repayment_schedule);

        let mut cf_date_vec: Vec<NaiveDate> = Vec::new();
        for repayment_schedule_value in repayment_schedule_vec {
            let cf_date = NaiveDate::parse_from_str(&repayment_schedule_value[0], "%d-%m-%Y")
                .unwrap_or(*config_params.as_on_date());
            cf_date_vec.push(cf_date)
        }

        let mut last_reset_date = cf_date_vec
            .iter()
            .cloned()
            .min()
            .unwrap_or(*config_params.as_on_date());

        if last_reset_date <= *config_params.as_on_date() {
            last_reset_date = *config_params.as_on_date();
        };
        let default_compmis2 = "NULL".to_string();
        let compmis2 = writeoff_map.get(&loan_id).unwrap_or(&default_compmis2);
        let spread = product_fixed_floating_map
            .clone()
            .get(input_fields[17])
            .unwrap_or(&"NULL".to_string())
            .to_string();

        let npa_usr_typ_flag: bool =
            tcfsl_map.contains_key(trimed_loan_id) && schm_code.to_uppercase() == "BANCS";

        let mut npa_typ = "P".to_string();
        let mut final_user_def_status: String = "P".to_string();

        if npa_usr_typ_flag == true {
            npa_typ = user_def_stats.to_string();
            final_user_def_status = user_def_stats.to_string();
        }
        let mut accrued_int_wrt = false;
        for repayment_schedule_value in repayment_schedule_vec {
            let mut accrued_int = 0.0;
            if !accrued_int_wrt {
                accrued_int = get_element_by_index(
                    &input_fields,
                    35,
                    line_num,
                    config_params.input_file_path(),
                )
                .parse::<f64>()
                .unwrap_or(0.0);
                accrued_int_wrt = true;
            }
            let due_date = NaiveDate::parse_from_str(&repayment_schedule_value[0], "%d-%m-%Y")
                .unwrap_or(mat_date);
            let principal_amount = repayment_schedule_value[1].parse().unwrap_or(0.0);
            let interest_amount = repayment_schedule_value[2].parse().unwrap_or(0.0);

            let mut compmis3 = "NULL".to_string();
            let input_product_id =
                get_element_by_index(&input_fields, 1, line_num, config_params.input_file_path());
            let input_subproduct_id =
                get_element_by_index(&input_fields, 1, line_num, config_params.input_file_path());
            if product_fixed_map.contains_key(input_product_id.clone())
                && product_fixed_floating_map.contains_key(input_subproduct_id.clone())
            {
                let default_data = ("".to_string(), "NULL".to_string());
                let product_data = product_fixed_map
                    .get(input_product_id.clone())
                    .unwrap_or(&default_data);
                compmis3 = product_data.1.clone();
            }
            let division = compmis2;

            // Output line for Principal
            let output_line_prin = get_output_line(
                &input_fields,
                line_num,
                config_params,
                "PRINCIPAL".to_string(),
                principal_amount,
                due_date,
                final_user_def_status.to_string(),
                npa_typ.to_string(),
                compmis2,
                &spread,
                &last_reset_date,
                accrued_int,
                compmis3.clone(),
                division.to_string(),
            );
            // Output line for interest line
            let output_line_int = get_output_line(
                &input_fields,
                line_num,
                config_params,
                "MAIN_INT".to_string(),
                interest_amount,
                due_date,
                final_user_def_status.to_string(),
                npa_typ.to_string(),
                compmis2,
                &spread,
                &last_reset_date,
                0.0,
                compmis3.clone(),
                division.to_string(),
            );
            writeln!(output_writer, "{}", output_line_prin)
                .expect("Output Line can not be written");
            writeln!(output_writer, "{}", output_line_int).expect("Output Line can not be written");
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
fn get_output_line(
    input_fields: &Vec<&str>,
    line_num: usize,
    config_params: &ConfigurationParameters,
    account_type: String,
    amount: f64,
    due_date: NaiveDate,
    user_def_stats: String,
    npa_typ: String,
    compmis2: &String,
    spread: &String,
    last_reset_date: &NaiveDate,
    accrued_int: f64,
    compmis3: String,
    division: String,
) -> String {
    let mat_date: NaiveDate = NaiveDate::parse_from_str(
        get_element_by_index(&input_fields, 6, line_num, config_params.input_file_path())
            .replace("00:00:00", "")
            .trim(),
        "%Y-%m-%d"
    ).unwrap_or(*config_params.as_on_date());
    let output_line=format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||||{}|{}||{}||||||{}|{}|{}||{}|{}|||{}||||||",
        get_element_by_index(&input_fields, 14, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 0, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 10, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 23, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 5, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 4, line_num, config_params.input_file_path()),
        NaiveDate::parse_from_str(
            get_element_by_index(&input_fields, 8, line_num, config_params.input_file_path())
                .replace("00:00:00", "")
                .trim(),
            "%Y-%m-%d"
        ).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(
            get_element_by_index(&input_fields, 8, line_num, config_params.input_file_path())
                .replace("00:00:00", "")
                .trim(),
            "%Y-%m-%d"
        ).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
        NaiveDate::parse_from_str(
            get_element_by_index(&input_fields, 6, line_num, config_params.input_file_path())
                .replace("00:00:00", "")
                .trim(),
            "%Y-%m-%d"
        ).unwrap_or(*config_params.as_on_date()).format("%d-%m-%Y"),
        due_date.format("%d-%m-%Y"),
        user_def_stats,
        get_element_by_index(&input_fields, 1, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 19, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 3, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 7, line_num, config_params.input_file_path()),
        account_type,
        get_element_by_index(&input_fields, 7, line_num, config_params.input_file_path()),
        get_element_by_index(&input_fields, 12, line_num, config_params.input_file_path()),
        amount,
        spread,
        get_element_by_index(&input_fields, 1, line_num, config_params.input_file_path()),
        compmis2,
        compmis3,//23
        last_reset_date.format("%d-%m-%Y"),//28
        get_element_by_index(&input_fields, 38, line_num, config_params.input_file_path()),//29
        division,//31
        NaiveDate::parse_from_str(
            get_element_by_index(&input_fields, 39, line_num, config_params.input_file_path())
                .replace("00:00:00.0000000", "")
                .trim(),
            "%Y-%m-%d"
        ).unwrap_or(mat_date).format("%d-%m-%Y"),
        config_params.as_on_date().format("%d-%m-%Y"),
        config_params.int_basis(),
        get_element_by_index(&input_fields, 13, line_num, config_params.input_file_path()),
        npa_typ,
        accrued_int,
    ).to_string();
    output_line
}
pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
