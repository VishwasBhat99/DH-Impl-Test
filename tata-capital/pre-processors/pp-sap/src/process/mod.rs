use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::input_account::*;
use crate::process::output::{write_output, Output};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use rbdate::NaiveDate;
use serde_json::to_string;
use slog::Logger;
use std::collections::HashMap;
use std::{fs, io::Write};

mod config;
mod input_account;
mod output;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    //Three Outputs(B and Borrowings-Amortized)
    let op_path_borr = format!(
        "{}-borr.txt",
        &config_params.output_file().replace(".txt", "")
    );
    let op_path_inv: String = format!(
        "{}-inv.txt",
        &config_params.output_file().replace(".txt", "")
    );
    let op_path_fx: String = format!(
        "{}-fx.txt",
        &config_params.output_file().replace(".txt", "")
    );
    let mut op_writer_borr = get_writer(&op_path_borr);
    let mut op_writer_inv = get_writer(&op_path_inv);
    let mut op_writer_fx = get_writer(&op_path_fx);

    let op_header = "DEAL_ID|BRANCH|INST_NAME|LEND_BORR_TYP|TYPOLOGY|USAGE|SUB_TYP_BORR_LEND|CNTRPRTY|CRTN_DT|VAL_DATE|DEAL_DATE|CCY|CRNT_DEAL_AMT|CRNT_CONV_RT_LCY|CRNT_DEAL_AMT_LCY|ROI|TENOR_DAYS|MAT_DT|PRIN_AMT|INT_AMT|CF_TYP|FLOW_TYP|MAT_AMT|DEALER_NAME|NDS_REF_NO|NXT_FIX_DT|RESIDUAL_TENOR|NXT_PUT_DT|NXT_CALL_DT|NXT_INT_PAY_DT|INT_PAY_TENOR|AIP_AIR|DOWNGRADE_CLAUSE|AVG_MONTHLY_BAL|GLCODE|CNTRPRTY_CTGRY_1|CNTRPRTY_CTGRY_2|CNTRPRTY_CTGRY_3|CNTRPRTY_CTGRY_4|INT_PAY_REC|BCKT_DAYS|CNTRPRTY_CTGRY_5|IND_OUTSIDE_IND|SYSTEM_GL|PROD_CONCAT|ALM_CONCAT|DIV|ALM_LINE|IA_LINE".to_string();
    writeln!(op_writer_borr, "{}", op_header).expect("Error in Writing Header to Borr Output");
    writeln!(op_writer_inv, "{}", op_header).expect("Error in Writing Header to Inv Output");
    writeln!(op_writer_fx, "{}", op_header).expect("Error in Writing Header to FX Output");

    let files_config = config::get_files(config_params.config_file_path());

    //Reading Input File 2
    let mut input_map_2: HashMap<(String, String), Vec<Input2>> = HashMap::new();
    let input_reader_2 =
        fs::read_to_string(&files_config.input_file_2).expect("Could Not Read Input File 2");

    let mut isin = "NA";
    for (line_no, line) in input_reader_2.lines().enumerate() {
        let inp_vec_2: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
            isin = inp_vec_2[9];
        let input_data_2 = Input2::new(
            config_params,
            &files_config.input_file_2,
            &inp_vec_2,
            line_no + 1,
            isin.to_string(),
        );
        //get the first record's isin from cf-file
        input_map_2
            .entry((
                input_data_2.txn_id.to_string(),
                input_data_2.co_code.to_string(),
            ))
            .and_modify(|data| data.push(input_data_2.clone()))
            .or_insert(vec![input_data_2.clone()]);
    }
    //Reading Input File 3
    let mut input_map_3: HashMap<String, Input3> = HashMap::new();
    let input_reader_3 =
        fs::read_to_string(&files_config.input_file_3).expect("Could Not Read Input File 3");
    for (line_no, line) in input_reader_3.lines().enumerate() {
        let inp_vec_3: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let input_data_3 = Input3::new(
            config_params,
            &files_config.input_file_3,
            &inp_vec_3,
            line_no + 1,
        );
        input_map_3.insert(input_data_3.base_rate_id.to_string(), input_data_3);
    }

    //Reading Input File 4
    let mut input_map_4: HashMap<String, Input4> = HashMap::new();
    let input_reader_4 =
        fs::read_to_string(&files_config.input_file_4).expect("Could Not Read Input File 4");
    for (line_no, line) in input_reader_4.lines().enumerate() {
        let inp_vec_4: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let input_data_4 = Input4::new(
            config_params,
            &files_config.input_file_4,
            &inp_vec_4,
            line_no + 1,
        );
        input_map_4.insert(
            input_data_4
                .bp_id
                .to_string()
                .trim_start_matches('0')
                .to_string(),
            input_data_4,
        );
    }

    //Reading Input File 5
    let mut input_map_5: HashMap<String, Input5> = HashMap::new();
    let input_reader_5 =
        fs::read_to_string(&files_config.input_file_5).expect("Could Not Read Input File 5");
    for (line_no, line) in input_reader_5.lines().enumerate() {
        let inp_vec_5: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let input_data_5 = Input5::new(
            config_params,
            &files_config.input_file_5,
            &inp_vec_5,
            line_no + 1,
        );
        input_map_5.insert(input_data_5.co_code.to_string(), input_data_5);
    }

    //Reading Input File 6
    let mut input_map_6: HashMap<String, Input6> = HashMap::new();
    let input_reader_6 =
        fs::read_to_string(&files_config.input_file_6).expect("Could Not Read Input File 6");
    let mut cal_date_diff = i64::MAX;
    for (line_no, line) in input_reader_6.lines().enumerate() {
        let inp_vec_6: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let input_data_6 = Input6::new(
            config_params,
            &files_config.input_file_6,
            &inp_vec_6,
            line_no + 1,
        );
        if input_map_6.contains_key(&input_data_6.txn_id) {
            //insert the lastest date with respect to as_on_date
            let mut curr_cal_date_diff = input_data_6
                .calc_end_date
                .signed_duration_since(*config_params.as_on_date())
                .num_days();
            if curr_cal_date_diff < 0 {
                curr_cal_date_diff *= -1;
            }
            if curr_cal_date_diff < cal_date_diff {
                input_map_6.insert(input_data_6.txn_id.to_string(), input_data_6);
                cal_date_diff = curr_cal_date_diff;
            }
        } else {
            input_map_6.insert(input_data_6.txn_id.to_string(), input_data_6);
        }
    }

    //Read Input File 7
    let mut input_file_7 =
        open_workbook_auto(&files_config.input_file_7).expect("Unable to open Input File 7.");
    println!(
        "Sheets present in Input File 7: `{:?}`",
        input_file_7.sheet_names()
    );
    if !input_file_7
        .sheet_names()
        .contains(&files_config.file7_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Input File 7: `{}`",
            files_config.file7_sheet_name, files_config.input_file_7,
        );
    }
    println!(
        "Reading Sheet: `{}` from Input File 7",
        files_config.file7_sheet_name,
    );
    let mut input_map_7: HashMap<String, Input7> = HashMap::new();
    if let Some(Ok(reader)) = input_file_7.worksheet_range(&files_config.file7_sheet_name) {
        for (line, row) in reader.rows().enumerate() {
            let input_acc_7 = Input7::new(&files_config.input_file_7, row, line + 1);
            input_map_7.insert(input_acc_7.prod_type.to_string(), input_acc_7);
        }
    }

    //Read Input File 8
    let mut input_file_8 =
        open_workbook_auto(&files_config.input_file_8).expect("Unable to open Input File 8.");
    println!(
        "Sheets present in Input File 8: `{:?}`",
        input_file_8.sheet_names()
    );
    if !input_file_8
        .sheet_names()
        .contains(&files_config.file8_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Input File 8: `{}`",
            files_config.file8_sheet_name, files_config.input_file_8,
        );
    }
    println!(
        "Reading Sheet: `{}` from Input File 8",
        files_config.file8_sheet_name,
    );
    let mut input_map_8: HashMap<String, Input8> = HashMap::new();
    if let Some(Ok(reader)) = input_file_8.worksheet_range(&files_config.file8_sheet_name) {
        for (line, row) in reader.rows().enumerate() {
            let input_acc_8 = Input8::new(&files_config.input_file_8, row, line + 1);
            input_map_8.insert(input_acc_8.update_type.to_string(), input_acc_8);
        }
    }

    //Read Input File 9
    let mut input_file_9 =
        open_workbook_auto(&files_config.input_file_9).expect("Unable to open Input File 9.");
    println!(
        "Sheets present in Input File 9: `{:?}`",
        input_file_9.sheet_names()
    );
    if !input_file_9
        .sheet_names()
        .contains(&files_config.file9_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Input File 9: `{}`",
            files_config.file9_sheet_name, files_config.input_file_9,
        );
    }
    println!(
        "Reading Sheet: `{}` from Input File 9",
        files_config.file9_sheet_name,
    );
    let mut input_map_9: HashMap<String, Input9> = HashMap::new();
    if let Some(Ok(reader)) = input_file_9.worksheet_range(&files_config.file8_sheet_name) {
        for (line, row) in reader.rows().enumerate() {
            let input_acc_9 = Input9::new(config_params, &files_config.input_file_9, row, line + 1);
            input_map_9.insert(input_acc_9.isin_no.to_string(), input_acc_9);
        }
    }

    //Read Input File 10
    let mut input_file_10 =
        open_workbook_auto(&files_config.input_file_10).expect("Unable to open Input File 10.");
    println!(
        "Sheets present in Input File 10: `{:?}`",
        input_file_10.sheet_names()
    );
    if !input_file_10
        .sheet_names()
        .contains(&files_config.file10_sheet_name.to_string())
    {
        panic!(
            "Sheet passed: `{}` not present in Input File 10: `{}`",
            files_config.file10_sheet_name, files_config.input_file_10,
        );
    }
    println!(
        "Reading Sheet: `{}` from Input File 10",
        files_config.file10_sheet_name,
    );
    let mut input_map_10: HashMap<String, Input10> = HashMap::new();
    if let Some(Ok(reader)) = input_file_10.worksheet_range(&files_config.file8_sheet_name) {
        for (line, row) in reader.rows().enumerate() {
            let input_acc_10 = Input10::new(&files_config.input_file_10, row, line + 1);
            input_map_10.insert(input_acc_10.portfolio.to_string(), input_acc_10);
        }
    }

    //Reading Input File 11
    let mut input_map_11: HashMap<String, Input11> = HashMap::new();
    let input_reader_11 =
        fs::read_to_string(&files_config.input_file_11).expect("Could Not Read Input File 11");
    for (line_no, line) in input_reader_11.lines().enumerate() {
        let inp_vec_11: Vec<&str> = line.split("|").collect::<Vec<&str>>();
        let input_data_11 = Input11::new(
            config_params,
            &files_config.input_file_11,
            &inp_vec_11,
            line_no + 1,
        );
        input_map_11.insert(input_data_11.transactionclassid.to_string(), input_data_11);
    }
    //Reading Input File 1
    let input_reader =
        fs::read_to_string(&files_config.input_file_1).expect("Could Not Read Input File 1");
    for (line_no, line) in input_reader.lines().enumerate() {
        acc_enc += 1;
        let inp_vec: Vec<&str> = line.split("*|~").collect::<Vec<&str>>();
        let input_data = Input1::new(
            config_params,
            &files_config.input_file_1,
            &inp_vec,
            line_no + 1,
        );
        if input_data.end_date <= *config_params.as_on_date() || input_data.principal_ost <= 0.0 {
            log_error!(
                _logger,
                "Skipping Account: {} as End-Date: {} Lesser that As-On-Date Found",
                input_data.txn_id,
                input_data.end_date
            );
            continue;
        }
        if input_data.co_code != config_params.company_code() {
            log_warn!(
                _logger,
                "Skipping Account: {} as Company Code: {} Not Matching with Company Code: {} passed in config",
                input_data.txn_id,
                input_data.co_code,
                config_params.company_code()
            );
            continue;
        }
        ip_amt += input_data.principal_amt;
        let mut output_data = Output::new(input_data.clone(), config_params);

        if input_map_4.contains_key(&input_data.bp_id.trim_start_matches('0').to_string()) {
            output_data.cntrprty = input_map_4
                .get(&input_data.bp_id.trim_start_matches('0').to_string())
                .expect("Could Not get data from Input File 4")
                .bp_name
                .to_string();
        }

        output_data.tenor_days =
            rbdate::num_days_start_to_end(input_data.start_date, input_data.end_date).to_string();

        output_data.residual_tenor =
            rbdate::num_days_start_to_end(*config_params.as_on_date(), input_data.end_date)
                .to_string();

        (output_data.nxt_put_dt, output_data.nxt_call_dt) =
            if input_map_5.contains_key(&input_data.txn_id) {
                let opt_type = input_map_5
                    .get(&input_data.txn_id.to_string())
                    .expect("Could Not get data from Input File 5")
                    .opt_type
                    .to_string();
                if opt_type.to_uppercase() == "CALL" {
                    (input_data.update_date, input_data.end_date)
                } else if opt_type.to_uppercase() == "PUT" {
                    (input_data.end_date, input_data.update_date)
                } else {
                    (input_data.end_date, input_data.end_date)
                }
            } else {
                (input_data.end_date, input_data.end_date)
            };

        if input_map_6.contains_key(&input_data.txn_id) {
            let input_data_6 = &input_map_6
                .get(&input_data.txn_id.to_string())
                .expect("Could Not get data from Input File 6");
            let base_rate_id = input_data_6.base_rate_id.to_string();
            output_data.roi = input_data_6.int_rate;
            if input_map_3.contains_key(&base_rate_id) {
                output_data.nxt_int_pay_dt = input_map_3
                    .get(&base_rate_id)
                    .expect("Could Not get data from Input File 3")
                    .reset_date;
            }
        }

        if input_map_7.contains_key(&input_data.prd_typ.clone()) {
            let borr_inv_fx_data = input_map_7
                .get(&input_data.prd_typ.to_string())
                .expect("Could Not get data from Input File 7");
            output_data.lend_borr_typ = borr_inv_fx_data.borr_inv_fx.to_string();
            output_data.sub_type_borr_lend = borr_inv_fx_data.desc.to_string();
        } else {
            log_error!(
                _logger,
                "Product: `{}` Not found for Account: {},{} in Input-File-7",
                input_data.prd_typ,
                input_data.co_code,
                input_data.txn_id
            );
        }
        let default_input_data_11 = Input11::default();
        let input_data_11 = input_map_11
            .get(&input_data.txn_id)
            .unwrap_or(&default_input_data_11);

        let default_cashflow_vec = vec![Input2::default()];
        output_data.nds_ref_no = match input_map_2.get(&(
            input_data.txn_id.to_string(),
            input_data.co_code.to_string(),
        )) {
            Some(val) => val,
            None => &default_cashflow_vec,
        }[0]
        .isin_no
        .to_owned();
        output_data.val_date = input_data.end_date;

        output_data.aip_air = input_map_9
            .get(&output_data.nds_ref_no)
            .unwrap_or(&Input9::default())
            .category
            .to_owned();
        output_data.flow_typ = input_map_10
            .get(&input_data.portfolio)
            .unwrap_or(&Input10::default())
            .protfolio_name
            .to_owned();

        //write the output for inv
        if output_data.lend_borr_typ.to_uppercase().as_str() == "INVESTMENTS" {
            output_data.prin_amt = input_data.book_val;
            write_output(
                output_data.lend_borr_typ.to_uppercase(),
                &mut output_data.clone(),
                &mut op_writer_inv,
                _logger,
            );
        }
        //write the output for fx
        if output_data.lend_borr_typ.to_uppercase().as_str() == "FX" {
            if ["10L", "10M", "45A", "45B", "45C"].contains(&input_data.prd_typ.as_str()) {
                output_data.prin_amt = input_data.principal_ost;
            }
            write_output(
                output_data.lend_borr_typ.to_uppercase(),
                &mut output_data.clone(),
                &mut op_writer_fx,
                _logger,
            );
        }

        match input_map_2.get(&(
            input_data.txn_id.to_string(),
            input_data.co_code.to_string(),
        )) {
            Some(account) => {
                let mut cf_date_vec = Vec::new();
                let mut isin="";
                for cashflow in account.iter() {
                    cf_date_vec.push(cashflow.payment_date);
                    if isin=="NA" || isin.trim().is_empty(){
                        isin=&cashflow.isin_no;
                    }
                   
                }
                output_data.nxt_fix_dt = cf_date_vec
                    .iter()
                    .cloned()
                    .min()
                    .unwrap_or(*config_params.as_on_date());
                if output_data.nxt_fix_dt <= *config_params.as_on_date() {
                    output_data.nxt_fix_dt = *config_params.as_on_date();
                };
                let mut accrued_int_wrt = false;
                for cashflow in account.iter() {
                    //logic to write the accrued interest for first cf only
                    output_data.int_pay_rec = "0.0".to_string();
                    if !accrued_int_wrt {
                        if input_data_11.category_type == output_data.aip_air {
                            output_data.int_pay_rec =
                                input_data_11.category_accruedinterest.to_string()
                        }
                        output_data.nds_ref_no = isin.to_string();
                        accrued_int_wrt = true;
                    }

                    if cashflow.payment_date <= *config_params.as_on_date() {
                        log_error!(
                            _logger,
                            "Skipping Account: {} as CF-Date: {} Lesser that As-On-Date Found",
                            cashflow.payment_date,
                            input_data.end_date
                        );
                        continue;
                    }
                    if !input_map_8.contains_key(&cashflow.update_type.to_string()) {
                        output_data.cf_typ = "Interest".to_string();
                        output_data.int_amt = cashflow.amount;
                        output_data.prin_amt = 0.0;
                    } else {
                        output_data.int_amt = 0.0;
                        output_data.cf_typ = "Principal".to_string();
                        output_data.prin_amt = cashflow.amount;
                    }
                    output_data.aip_air = input_map_9
                        .get(&output_data.nds_ref_no)
                        .unwrap_or(&Input9::default())
                        .category
                        .to_owned();
                    output_data.val_date = cashflow.payment_date;
                    if input_data_11.category_type==output_data.aip_air {
                      output_data.nxt_int_pay_dt=input_data_11.nextintdate;
                    }
                    if cashflow.update_type.to_uppercase().to_string() == "MM1510" {
                        output_data.prin_amt= output_data.prin_amt*(-1.0);
                    }
                    if output_data.lend_borr_typ.to_uppercase().as_str() == "BORROWINGS" {
                        write_output(
                            output_data.lend_borr_typ.to_uppercase(),
                            &mut output_data.clone(),
                            &mut op_writer_borr,
                            _logger,
                        );
                    }
                }
            }
            None => {
                log_warn!(
                    _logger,
                    "Cashflows not found for account: {}",
                    input_data.txn_id
                );
                if output_data.lend_borr_typ.to_uppercase().as_str() == "BORROWINGS" {
                    write_output(
                        output_data.lend_borr_typ.to_uppercase(),
                        &mut output_data.clone(),
                        &mut op_writer_borr,
                        _logger,
                    )
                }
            }
        }

        acc_proc += 1;
        op_amt += output_data.prin_amt;
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
