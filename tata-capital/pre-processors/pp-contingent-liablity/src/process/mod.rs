use crate::configuration_parameters::ConfigurationParameters;
use crate::process::config::ConfigData;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputAccount};
use calamine::{open_workbook_auto, Reader};
use chrono::prelude::*;
use health_report::HealthReport;
use rbdate::get_days_from_month;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::{fs, io::Write};
mod input_account;
mod config;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;

    let mut config_file1_str = String::new();
    let mut config_file2_vec=Vec::new();

    //Reading config file

    let files_config = config::get_files(config_params.config_file_path());
    for config_field in files_config.files{
        config_file1_str = config_field.config_file_1;
        config_file2_vec = config_field.config_file_2;
    }
    
    let config_file1_vec_refs: Vec<String> = config_file1_str
        .split(',')
        .map(|s| s.to_uppercase().to_string())
        .collect();

    let config_file1_vec: Vec<&str> = config_file1_vec_refs.iter().map(|s| s.as_str()).collect();

    let mut config_file2_map : HashMap<String,ConfigData> = HashMap::new();
    for ele in config_file2_vec.clone() {
        let code = ele.code.clone();
        config_file2_map.insert(code.to_uppercase(), ele);
    }

    //Reading input master file 
    let mut input_master_file = open_workbook_auto(config_params.input_master_file_path())
        .expect("Unable to open the  input master file");
    info!(
        logger,
        "Sheet present in input master excel file: {:?}",
        input_master_file.sheet_names()
    );
    if !input_master_file
        .sheet_names()
        .contains(&config_params.input_master_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in input master File: `{}`",
            config_params.input_master_sheet_name(),
            config_params.input_master_file_path()
        );
    }
    info!(
        logger,
        "Reading Sheet: `{:?}` from input master excel",
        &config_params.input_master_sheet_name().to_string()
    );
    let mut op_writer = get_writer(config_params.output_file());

    if let Some(Ok(input_master_reader)) =
        input_master_file.worksheet_range(&config_params.input_master_sheet_name().to_string())
    {
        for (row_no, row) in input_master_reader.rows().enumerate() {
            acc_enc += 1;
           let code_id: String = get_str_from_xlsx(row, 0);
           let account_desc = get_str_from_xlsx(row, 1);
            if config_file1_vec.contains(&&(code_id.as_str())){
                acc_proc +=1;
                let balance = get_str_from_xlsx(row, 2).parse::<f64>().unwrap_or(0.0);
                let output_data : OutputAccount = OutputAccount{
                    code: code_id.clone(),
                    description: account_desc.clone(),
                    amount: balance,
                    currency: config_params.display_ccy().to_owned(),
                    cf_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
                    customer_code: "".to_string()
                };
                writeln!(op_writer, "{}", format_output(output_data))
                .expect("Error in Writing Output");
            }else {
                if config_file2_map.contains_key(&code_id.to_uppercase()){
                    for config_data in config_file2_vec.clone(){
                    let excel_path = &config_data.excel_path;
                    let amount_field = config_data.amount_field.clone()-1;
                    let excel_sheeet_name = &config_data.sheet_name;
                    let cf_date_field = config_data.cf_date_field.clone()-1;
                    let look_up_field = config_data.look_up_field.clone()-1;
                    let look_up_val = config_data.look_up_value.clone();
                    let ident_field= config_data.identifier_field.clone()-1;
                    let mut ref_excel_file = open_workbook_auto(excel_path)
                    .expect("Unable to open the  reference excel file from the path provided in config 3 parameter");
                if let Some(Ok(input_master_reader)) =
                    ref_excel_file.worksheet_range(&excel_sheeet_name)
                    {
                        for (row_no, row) in input_master_reader.rows().enumerate() {
                            let look_up_ref_val = get_str_from_xlsx(row, look_up_field as usize);
                            let identifier_val = get_str_from_xlsx(row, ident_field as usize);
                            if look_up_ref_val.to_uppercase() == look_up_val.to_uppercase() {
                                acc_proc +=1;
                                 let amount = get_str_from_xlsx(row,amount_field as usize ).parse::<f64>().unwrap_or(0.0);
                                 let cf_date = get_str_from_xlsx(row, cf_date_field as usize);
                                 let cf_date_from_dateval = get_date_from_string(cf_date.clone(),config_params);
                                 let output_data : OutputAccount = OutputAccount{
                                    code: code_id.clone(),
                                    description: look_up_ref_val.clone(),
                                    amount: amount.clone(),
                                    currency: config_params.display_ccy().to_owned(),
                                    cf_date : cf_date_from_dateval.format("%d-%m-%Y").to_string(),
                                    customer_code:  identifier_val
                                };
                                writeln!(op_writer, "{}", format_output(output_data))
                                .expect("Error in Writing Output");
                            }
                        }  
                    }

                }
            }
            }
        }
    }
    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
