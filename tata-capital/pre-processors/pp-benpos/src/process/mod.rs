use crate::configuration_parameters::ConfigurationParameters;
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
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;

    //Getting company code
    let company_code_vec: Vec<&str> = config_params.product_ids().split(',').collect();
    //Reading master input file
    let mut input_master_map: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let as_on_date = config_params.as_on_date();
    let first_day_of_curr_month = NaiveDate::from_ymd(
        config_params.as_on_date().year(),
        config_params.as_on_date().month(),
        1,
    );
    let formated_first_day_of_curr_month = first_day_of_curr_month.format("%d%m%Y").to_string();
    let days_in_month = get_days_from_month(*as_on_date);
    let month_end_day_of_curr_month: NaiveDate =
        first_day_of_curr_month + chrono::Duration::days(days_in_month - 1);
    let formated_end_day_of_curr_month = month_end_day_of_curr_month.format("%d%m%Y").to_string();
    let input_file_string = format!(
        "{}_{}_{}.txt",
        config_params.input_master_file_path(),
        formated_first_day_of_curr_month,
        formated_end_day_of_curr_month
    );
    let benpos_file_string = format!(
        "{}_{}_{}.txt",
        config_params.benpos_cashflow_file_path(),
        formated_first_day_of_curr_month,
        formated_end_day_of_curr_month
    );

    let input_file_reader =
        fs::read_to_string(input_file_string).expect("could not read input file");
    for (line_no, line) in input_file_reader.lines().enumerate().skip(1) {
        acc_enc += 1;
        let input_vec: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let prod_id = get_str(
            config_params.input_master_file_path(),
            &input_vec,
            5,
            line_no + 1,
        );
        let portfolio = get_str(
            config_params.input_master_file_path(),
            &input_vec,
            8,
            line_no + 1,
        );
        let trans_id = get_str(
            config_params.input_master_file_path(),
            &input_vec,
            1,
            line_no + 1,
        );
        let company_code = get_str(
            config_params.input_master_file_path(),
            &input_vec,
            0,
            line_no + 1,
        );
        let company_code_slice: &str = &company_code;
        if !company_code_vec.contains(&company_code_slice) {
            continue;
        }
        input_master_map
            .entry(prod_id)
            .and_modify(|prev_data| prev_data.push((trans_id.clone(), portfolio.clone())))
            .or_insert(vec![(trans_id, portfolio)]);
    }

    //Reading Cashflow file
    let mut tid_isin_map: HashMap<String, String> = HashMap::new();
    let cashflow_file_reader =
        fs::read_to_string(benpos_file_string).expect("Error getting benpos cashflow file");
    for (line_no, line) in cashflow_file_reader.lines().enumerate().skip(1) {
        let cashflow_vec: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let trans_id = get_str(
            config_params.benpos_cashflow_file_path(),
            &cashflow_vec,
            1,
            line_no + 1,
        );
        let is_in = get_str(
            config_params.benpos_cashflow_file_path(),
            &cashflow_vec,
            9,
            line_no + 1,
        );
        let company_code = get_str(
            config_params.benpos_cashflow_file_path(),
            &cashflow_vec,
            0,
            line_no + 1,
        );
        let company_code_slice: &str = &company_code;

        if !company_code_vec.contains(&company_code_slice) || is_in == "" {
            continue;
        }
        tid_isin_map.insert(trans_id, is_in);
    }

    //Reading ncd benpos file
    let mut ncd_cp_map: HashMap<String, Vec<NcdCpData>> = HashMap::new();

    let mut ncd_benpos_file = open_workbook_auto(config_params.ncd_benpos_file_path())
        .expect("Unable to open the  ncd benpos file.");
    info!(
        logger,
        "Sheet present in ncd excel file: {:?}",
        ncd_benpos_file.sheet_names()
    );
    if !ncd_benpos_file
        .sheet_names()
        .contains(&config_params.ncd_benpos_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in ncd benpos File: `{}`",
            config_params.ncd_benpos_sheet_name(),
            config_params.ncd_benpos_file_path()
        );
    }
    info!(
        logger,
        "Reading Sheet: `{:?}` from Product-File",
        &config_params.ncd_benpos_sheet_name().to_string()
    );
    if let Some(Ok(ncd_benpos_file_reader)) =
        ncd_benpos_file.worksheet_range(&config_params.ncd_benpos_sheet_name().to_string())
    {
        for (row_no, row) in ncd_benpos_file_reader.rows().enumerate().skip(1) {
            let ncd_data: NcdCpData = NcdCpData::new_from_excel(config_params, row);
            let isin_id = &ncd_data.isin_id;
            ncd_cp_map
                .entry(isin_id.to_string())
                .and_modify(|prev_data| prev_data.push(ncd_data.clone()))
                .or_insert(vec![ncd_data.clone()]);
        }
    }

    //Reading Cp benpos file
    let mut cp_benpos_file = open_workbook_auto(config_params.cp_benpos_file_path())
        .expect("Unable to open the  cp benpos file.");
    info!(
        logger,
        "Sheet present in cp excel file: {:?}",
        cp_benpos_file.sheet_names()
    );
    if !cp_benpos_file
        .sheet_names()
        .contains(&config_params.cp_benpos_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in cp benpos File: `{}`",
            config_params.cp_benpos_sheet_name(),
            config_params.cp_benpos_file_path()
        );
    }

    info!(
        logger,
        "Reading Sheet: `{:?}` from cp benpos file",
        &config_params.cp_benpos_sheet_name().to_string()
    );
    if let Some(Ok(cp_benpos_file_reader)) =
        cp_benpos_file.worksheet_range(&config_params.cp_benpos_sheet_name().to_string())
    {
        for (row_no, row) in cp_benpos_file_reader.rows().enumerate().skip(1) {
            let ncd_data: NcdCpData = NcdCpData::new_from_excel(config_params, row);
            let isin_id = &ncd_data.isin_id;
            ncd_cp_map
                .entry(isin_id.to_string())
                .and_modify(|prev_data| prev_data.push(ncd_data.clone()))
                .or_insert(vec![ncd_data]);
        }
    }

    //Reading Product excel file
    let mut op_writer = get_writer(config_params.output_file());
    let mut isin_flag_set: HashSet<String> = HashSet::new();
    let mut product_file_path = open_workbook_auto(config_params.product_file_path())
        .expect("Unable to open the product xlsx file.");
    info!(
        logger,
        "Sheet present in product excel file: {:?}",
        product_file_path.sheet_names()
    );
    if !product_file_path
        .sheet_names()
        .contains(&config_params.product_sheet_name().to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in TCFSL NPA File: `{}`",
            config_params.product_sheet_name(),
            config_params.product_file_path()
        );
    }
    info!(
        logger,
        "Reading Sheet: `{:?}` from Product-File",
        &config_params.product_sheet_name().to_string()
    );
    let mut isin_hashset: HashSet<String> = HashSet::new();
    if let Some(Ok(product_file_reader)) =
        product_file_path.worksheet_range(&config_params.product_sheet_name().to_string())
    {
        for (row_no, row) in product_file_reader.rows().enumerate().skip(1) {
            let product_id = get_str_from_xlsx(row, 0);
            let default_val: Vec<(String, String)> = Vec::new();
            let trans_id_vec = input_master_map.get(&product_id).unwrap_or(&default_val);
            for (tid, portfolio) in trans_id_vec {
                let default_isin = "".to_string();
                let isin: &String = tid_isin_map.get(&tid.to_string()).unwrap_or(&default_isin);
                if isin.is_empty() {
                    continue;
                }
                let ncd_cp_default: Vec<NcdCpData> = Vec::new();
                let ncdcp_data = ncd_cp_map.get(isin).unwrap_or(&ncd_cp_default);
                if !isin_hashset.contains(isin) {
                    for ele in ncdcp_data {
                        acc_proc += 1;
                        let output_data: OutputAccount = OutputAccount {
                            product_type: product_id.clone(),
                            isin: isin.to_string(),
                            first_holder_pan: ele.first_holder_pan.to_string(),
                            first_holder_name: ele.first_holder_name.to_string(),
                            category: ele.category.to_string(),
                            amount: ele.amount.to_string(),
                            principal_os: "".to_string(),
                            mat_date: ele.mat_date.format("%d-%m-%Y").to_string(),
                            portfolio: portfolio.to_string(),
                            ccy: config_params.display_ccy().to_string(),
                        };
                        writeln!(op_writer, "{}", format_output(output_data))
                            .expect("Error in Writing Output");
                    }
                    isin_hashset.insert(isin.to_string());
                }
            }
        }
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
