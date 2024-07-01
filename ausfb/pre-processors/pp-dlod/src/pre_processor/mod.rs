extern crate csv;
extern crate serde;
use self::csv::ReaderBuilder;
use self::derive_fields::get_op_line;
use self::input_account::{InputAccount, Ref1};
use calamine::{open_workbook_auto, Reader};
use chrono::{Datelike, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use macros;
use pre_processor::derive_fields::check_cashflow;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::SystemTime;
mod derive_fields;
mod input_account;
use health_report::HealthReport;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_derive_timer = SystemTime::now();
    let mut ref1_map: HashMap<String, Ref1> = HashMap::new();
    let mut ref2_map: HashMap<String, String> = HashMap::new();
    let mut ref3_map: HashMap<String, String> = HashMap::new();
    let mut ta_config_map: HashMap<String, String> = HashMap::new();
    let mut dlod_cashflow_map: HashMap<String, bool> = HashMap::new();
    let mut dlod_cashflow_map2: HashMap<String, bool> = HashMap::new();
    let mut odfd_cashflow_map: HashMap<String, String> = HashMap::new();
    let mut rtl_cashflow_map: HashMap<String, bool> = HashMap::new();
    let mut cust_entity_map: HashMap<String, String> = HashMap::new();
    let mut crm_master_map: HashMap<String, String> = HashMap::new();
    let as_on_date = config_param.as_on_date();

    let dlod_date_format = get_date_format(config_param.dlod_date_format().to_lowercase());
    let dlod_2_date_format = get_date_format(config_param.dlod_2_date_format().to_lowercase());
    let rtl_date_format = get_date_format(config_param.rtl_date_format().to_lowercase());
    //Reading Reference 1 File
    let ref1_reader = match new_buf_rdr(config_param.reference_file_path_1()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.reference_file_path_1(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let asset_types: Vec<&str> = config_param.asset_types().split(',').collect();
    for (line_num, lines) in ref1_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.reference_file_path_1(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.reference_1_delim()).collect();
        //Check for asset_types:
        if asset_types.contains(&fields[0].trim()) {
            //Drop the leading two letters
            let cod_acc_no = &fields[1]
                .replace("'", "")
                .replace('\'', "")
                .replace("OD", "");
            ref1_map
                .entry(cod_acc_no.to_string())
                .and_modify(|data| data.get_max(fields.to_owned()))
                .or_insert(Ref1 {
                    asset_type: fields[0].to_string().replace("'", "").replace("\"", ""),
                    cod_acc_no: fields[1].to_string().replace("'", "").replace("\"", ""),
                    cod_limit_no: fields[4].parse().unwrap_or(0),
                    loan_limit_amount: fields[5].parse().unwrap_or(0.0),
                    index_code: fields[10].to_string().replace("'", "").replace("\"", ""),
                    index_name: fields[11].to_string().replace("'", "").replace("\"", ""),
                    index_rate: fields[12].parse().unwrap_or(0.0),
                    effective_roi: fields[14].parse().unwrap_or(0.0),
                    reset_frequency: fields[15].to_string().replace("'", "").replace("\"", ""),
                    next_reset_date: fields[16].to_string().replace("'", "").replace("\"", ""),
                    tenure: fields[17].parse().unwrap_or(0.0),
                });
        }
    }
    //Reading Reference 2 File
    //File format can be xlsx or txt
    let file_name: Vec<&str> = config_param.reference_file_path_2().split('.').collect();
    let last_element = file_name.last().unwrap_or(&"na").to_string();
    match last_element.to_lowercase().as_str() {
        "xlsx" => {
            let mut input_reader = open_workbook_auto(config_param.reference_file_path_2())
                .expect("Unable to open Referece file 2 in xlsx format.");
            if !input_reader
                .sheet_names()
                .contains(&config_param.ref2_sheet_name().to_owned())
            {
                panic!(
                    "Sheet name: {} not found in file:{}",
                    &config_param.ref2_sheet_name(),
                    config_param.reference_file_path_2()
                );
            } else if let Some(Ok(reader)) =
                input_reader.worksheet_range(config_param.ref2_sheet_name())
            {
                for row in reader.rows() {
                    ref2_map.insert(
                        row[0].to_string().replace("'", "").replace("\"", ""),
                        row[1].to_string().replace("'", "").replace("\"", ""),
                    );
                }
            }
        }
        "txt" | "csv" => {
            let ref2_reader = match new_buf_rdr(config_param.reference_file_path_2()) {
                Ok(file) => file,
                Err(error) => panic!(
                    "Could not found file `{}` on location `{}` : {}.",
                    config_param.reference_file_path_2(),
                    current_dir()
                        .expect("Error while getting current directory path.")
                        .display(),
                    error
                ),
            };
            for (line_num, lines) in ref2_reader.lines().enumerate() {
                let line = match lines {
                    Ok(line) => line,
                    Err(error) => panic!(
                        "Unable to read file `{}` at line number: `{}` : {}",
                        config_param.reference_file_path_2(),
                        line_num + 1,
                        error
                    ),
                };
                let fields: Vec<&str> = line.split(config_param.reference_2_delim()).collect();
                ref2_map.insert(
                    fields[0].to_string().replace("'", "").replace("\"", ""),
                    fields[1].to_string().replace("'", "").replace("\"", ""),
                );
            }
        }
        _ => {
            let msg = format!("File format: {} of ref file 2 is not valid.", last_element);
            panic!("{}", msg);
        }
    }

    //Reading Reference 3 File
    let ref3_reader = match new_buf_rdr(config_param.reference_file_path_3()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.reference_file_path_3(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ref3_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.reference_file_path_2(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.reference_3_delim()).collect();
        ref3_map.insert(
            fields[3].to_string().replace("'", "").replace("\"", ""),
            fields[14].to_string().replace("'", "").replace("\"", ""),
        );
    }

    let ta_config_reader = match new_buf_rdr(config_param.ta_config_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ta_config_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ta_config_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ta_config_file(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.ta_config_delim()).collect();
        ta_config_map.insert(
            fields[0].to_string().replace("'", "").replace("\"", ""),
            "".to_string(),
        );
    }
    let dlod_cashflow_reader = match new_buf_rdr(config_param.dlod_cashflow_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.dlod_cashflow_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in dlod_cashflow_reader.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ta_config_file(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.dlod_delim()).collect();
        let dlod_date_field = if dlod_date_format.1 == true {
            update_year(fields[11], as_on_date, &dlod_date_format.0.as_str())
        } else {
            NaiveDate::parse_from_str(fields[11], &dlod_date_format.0.as_str())
                .expect("Could not format the date from DLOD file.")
        };

        dlod_cashflow_map
            .entry(
                fields[1]
                    .to_string()
                    .replace("'", "")
                    .replace("\"", "")
                    .replace("OD", ""),
            )
            .and_modify(|data| *data = check_cashflow(&dlod_date_field, &data, &as_on_date))
            .or_insert(check_cashflow(&dlod_date_field, &true, &as_on_date));
    }

    let dlod_cashflow_reader2 = match new_buf_rdr(config_param.dlod_cashflow_file_2()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.dlod_cashflow_file_2(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in dlod_cashflow_reader2.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.dlod_cashflow_file_2(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.dlod_2_delim()).collect();
        let dlod_2_date_field = if dlod_2_date_format.1 == true {
            update_year(fields[5], as_on_date, dlod_2_date_format.0.as_str())
        } else {
            NaiveDate::parse_from_str(fields[5], dlod_2_date_format.0.as_str())
                .expect("Could not format the date from DLOD Cashflow 2 file.")
        };

        dlod_cashflow_map2
            .entry(
                fields[3]
                    .to_string()
                    .replace("'", "")
                    .replace("\"", "")
                    .replace("OD", "")
                    .replace("A", ""),
            )
            .and_modify(|data| *data = check_cashflow(&dlod_2_date_field, &data, &as_on_date))
            .or_insert(check_cashflow(&dlod_2_date_field, &true, &as_on_date));
    }

    let odfd_cashflow_reader = match new_buf_rdr(config_param.odfd_cashflow_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.odfd_cashflow_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in odfd_cashflow_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ta_config_file(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.odfd_delim()).collect();
        odfd_cashflow_map.insert(
            fields[0].to_string().replace("'", "").replace("\"", ""),
            "".to_string(),
        );
    }

    let rtl_cashflow_reader = match new_buf_rdr(config_param.rtl_cashflow_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.rtl_cashflow_file(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in rtl_cashflow_reader.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ta_config_file(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.rtl_delim()).collect();
        let rtl_date_field = if rtl_date_format.1 == true {
            update_year(fields[3], as_on_date, rtl_date_format.0.as_str())
        } else {
            NaiveDate::parse_from_str(fields[3], &rtl_date_format.0.as_str())
                .expect("Could not format the date from RTL file.")
        };

        rtl_cashflow_map
            .entry(
                fields[0]
                    .to_string()
                    .replace("'", "")
                    .replace("\"", "")
                    .replace("OD", ""),
            )
            .and_modify(|data| *data = check_cashflow(&rtl_date_field, &data, &as_on_date))
            .or_insert(check_cashflow(&rtl_date_field, &true, &as_on_date));
    }

    let cust_entity_reader = match new_buf_rdr(config_param.cust_entity_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.cust_entity_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in cust_entity_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cust_entity_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.cust_entity_delim()).collect();
        cust_entity_map.insert(
            fields[0].to_string().replace("'", "").replace("\"", ""),
            fields[3].to_string().replace("'", "").replace("\"", ""),
        );
    }

    let crm_master_reader = match new_buf_rdr(config_param.crm_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.crm_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in crm_master_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.crm_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(config_param.crm_file_delim()).collect();
        crm_master_map.insert(
            fields[0].to_string().replace("'", "").replace("\"", ""),
            fields[2].to_string().replace("'", "").replace("\"", ""),
        );
    }

    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let mut header = "Customer_ID|Account_ID|Product_Type|Scheme_Type|Product_Code|Currency|Customer_Type|GL_Account_Principal|Open_date|Value_Date|Maturity_Date|Limit_Amount|Current_Balance_Amount|FLG_Fixed_Floating|Interest Paid|Interest Received|FLG_Performing_NPA|ASSET_TYPE|COD_ACCT_NO|COD_LIMIT_NO|LOAN_LIMIT_AMOUNT|Index_Code|Index_Name|Index_Rate|Effective_ROI|Reset_Frequency|Next_RESET_DATE|Tenure|Classification|Derived_reset_date|Final_reset_date|NPA_STATUS|NPA_FINAL_STATUS|add_pt_1|add_pt_2|add_pt_3|add_pt_4|add_pt_5|add_pt_6|add_pt_7|add_pt_8".to_string();
    header.push('\n');
    let mut tot_acc_encntrd: i64 = 0;
    let mut total_amt = 0.0;
    let mut writer = BufWriter::new(output_file);
    match writer.write_all(header.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing header data: {:?}", error);
        }
    }
    //Reading Input File
    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(config_param.input_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut acc_processed = 0;
    for (line_num, lines) in reader.deserialize().enumerate().skip(1) {
        let input_account: InputAccount = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        tot_acc_encntrd += 1;
        total_amt += input_account
            .current_bal_amount
            .parse::<f64>()
            .unwrap_or(0.0);
        let mut temp_string = get_op_line(
            &input_account,
            config_param.as_on_date,
            &ref1_map,
            &ref2_map,
            &ref3_map,
            &ta_config_map,
            &dlod_cashflow_map,
            &dlod_cashflow_map2,
            &odfd_cashflow_map,
            &rtl_cashflow_map,
            &cust_entity_map,
            &crm_master_map,
        );
        temp_string.push('\n');
        acc_processed += 1;
        match writer.write_all(temp_string.as_bytes()) {
            Ok(val) => val,
            Err(error) => {
                panic!("Error writing processed data: {:?}", error);
            }
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_processed,
        tot_acc_encntrd - acc_processed,
        total_amt,
        total_amt,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_param.output_file_path());
}

fn get_date_format(manual_format: String) -> (String, bool) {
    match manual_format.as_str() {
        "dd-mm-yyyy" => ("%d-%m-%Y".to_string(), false),
        "dd-mm-yy" => ("%d-%m-%y".to_string(), true),
        "dd-mmm-yyyy" => ("%d-%b-%Y".to_string(), false),
        "dd-mmm-yy" => ("%d-%b-%y".to_string(), true),
        "dd mmm yy" => ("%d %b %y".to_string(), true),
        "dd mmm yyyy" => ("%d %b %Y".to_string(), false),
        _ => ("dd-mm-yyyy".to_string(), false),
    }
}

fn update_year(date_string: &str, as_on_date: &NaiveDate, fmt: &str) -> NaiveDate {
    let date = NaiveDate::parse_from_str(date_string, &fmt)
        .expect("Cannot parse date format to update the year.");
    let current_year = date.year() % 100;
    let current_century = (as_on_date.year() / 100) * 100;
    let final_year = current_century + current_year;
    let updated_date = NaiveDate::from_ymd(final_year, date.month(), date.day());
    updated_date
}
