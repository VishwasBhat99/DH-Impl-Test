use self::cf_writer::account::Account;
use self::cf_writer::CFwrite;
use self::input_account::*;
use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use crate::process::account_appender::create_cf_acc;
use crate::process::output_writer::{get_moc_writer, get_output_writer};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::time::SystemTime;

mod account_appender;
mod cf_writer;
mod input_account;
mod io;
mod output_writer;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut tot_acc_skp = 0;
    let mut tot_cf_acc = 0;
    let mut tot_amt_in_ip = 0.0;
    let mut tot_amt_in_op = 0.0;
    let output_path = format!("{}.txt", config_params.output_file_path());
    let mut op_writer = get_writer(&output_path);
    let mut cf_writer = CFwrite::new(config_params.output_file_path(), log);

    //Reading ALM Master File
    let mut crdr_map: HashMap<String, String> = HashMap::new();
    let mut net_map: HashMap<String, String> = HashMap::new();
    let mut al_map: HashMap<String, String> = HashMap::new();
    let mut alm_coa_map: HashMap<String, Vec<i64>> = HashMap::new();
    let mut alm_master = open_workbook_auto(config_params.alm_master_file())
        .expect("Error while opening `ALM Master file`.");
    if let Some(Ok(reader)) = alm_master.worksheet_range(config_params.alm_master_sheet_name()) {
        for row in reader.rows().skip(1) {
            let method = row[3].to_string();
            let gl = row[0].to_string().parse::<i64>().unwrap_or(0);
            if method == "NET" {
                net_map.insert(gl.to_string(), row[2].to_string());
            } else if method == "AL" {
                al_map.insert(gl.to_string(), row[2].to_string());
                alm_coa_map
                    .entry(row[2].to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            } else {
                crdr_map.insert(gl.to_string(), row[2].to_string());
            }
        }
    }
    //Reading Input Master File
    let input = File::open(&config_params.input_file()).expect("Could Not Read Input File");
    let input_reader = BufReader::new(input);
    let mut input_map: HashMap<String, InputAccount> = HashMap::new();
    for (_index, line) in input_reader.lines().enumerate() {
        tot_acc_encntrd += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        let acc = InputAccount::new(
            input_fields[0].to_string(),
            input_fields[1].to_string(),
            input_fields[2].to_string(),
            input_fields[3].to_string(),
            input_fields[4].to_string(),
            input_fields[5].to_string(),
            input_fields[6].to_string(),
        );
        input_map
            .entry(input_fields[0].to_string())
            .and_modify(|val| val.append_data(acc.clone()))
            .or_insert(acc);
    }
    //HashMap for aggr_net_bal of alm_coa for method AL
    let mut net_bal_map: HashMap<i64, f64> = HashMap::new();
    for (_k, _v) in alm_coa_map {
        let mut net_bal = 0.0;
        for val in _v.iter() {
            if input_map.contains_key(&val.to_string()) {
                let input_acc = input_map.get(&val.to_string()).unwrap();
                net_bal += input_acc.out_bal;
            }
        }
        for val in _v.iter() {
            net_bal_map.insert(*val, net_bal);
        }
    }

    //Reading GL-Exclude File
    let gl_ex = File::open(&config_params.gl_ex_master()).expect("Could Not Read GL-Ex File");
    let gl_ex_reader = BufReader::new(gl_ex);
    let mut gl_ex_map: HashMap<String, String> = HashMap::new();
    for (_index, line) in gl_ex_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        gl_ex_map.insert(input_fields[0].to_string(), input_fields[0].to_string());
    }

    //Reading Financial-Mapping File
    let mut fin_map: HashMap<String, FinMap> = HashMap::new();
    let mut fin_map_file = open_workbook_auto(config_params.fin_code_map_file())
        .expect("Error while opening `Fin Map file`.");
    if let Some(Ok(reader)) = fin_map_file.worksheet_range(config_params.fin_code_sheet_name()) {
        for row in reader.rows().skip(1) {
            if row.len() != 5 {
                log_error!(log, "Incorrect data in Fin_Map file for GL: `{}`", row[0]);
                continue;
            }
            let fin_acc = FinMap::new(
                row[0].to_string(),
                row[1].to_string(),
                row[2].to_string(),
                row[3].to_string(),
                row[4].to_string(),
            );
            fin_map.insert(row[0].to_string(), fin_acc);
        }
    }

    //Read the GL-Moc File
    let mut moc_map: HashMap<String, MocAccount> = HashMap::new();
    let mut moc_file = open_workbook_auto(config_params.gl_moc_entry_file())
        .expect("Error while opening `Fin Map file`.");
    if let Some(Ok(reader)) = moc_file.worksheet_range(config_params.gl_moc_sheet_name()) {
        for row in reader.rows().skip(1) {
            if row.len() != 7 {
                log_error!(log, "Incorrect data in Fin_Map file for GL: `{}`", row[0]);
                continue;
            }
            let moc_acc = MocAccount::new(
                row[0].to_string(),
                row[1].to_string(),
                row[2].to_string(),
                row[3].to_string(),
                row[4].to_string(),
                row[5].to_string(),
                row[6].to_string(),
            );
            moc_map.insert(row[0].to_string(), moc_acc);
        }
    }
    //Writing Output
    let mut output_line = String::new();
    for line in
        BufReader::new(File::open(&config_params.input_file()).expect("Could Not Read Input File"))
            .lines()
            .skip(1)
    {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("error while reading input file : {:?}", error);
            }
        };
        let input_fields: Vec<&str> = record.split("|").collect();
        if input_fields.len() != 7 {
            log_error!(log, "Incorrect Account in Master-Input: `{}`", record);
            tot_acc_skp += 1;
            continue;
        }
        tot_amt_in_ip += input_fields[1].to_string().parse::<f64>().unwrap_or(0.0);
        let mut is_acc_gl = "N".to_string();
        if gl_ex_map.contains_key(&input_fields[0].to_string()) {
            is_acc_gl = "Y".to_string();
        }
        if crdr_map.contains_key(&input_fields[0].to_string()) {
            get_output_writer(
                &config_params,
                &input_fields,
                &"C".to_string(),
                &is_acc_gl,
                &crdr_map.get(&input_fields[0].to_string()).unwrap(),
                &fin_map,
                &mut output_line,
            );
            get_output_writer(
                &config_params,
                &input_fields,
                &"D".to_string(),
                &is_acc_gl,
                &crdr_map.get(&input_fields[0].to_string()).unwrap(),
                &fin_map,
                &mut output_line,
            );
        }
        if net_map.contains_key(&input_fields[0].to_string()) {
            get_output_writer(
                &config_params,
                &input_fields,
                &"N".to_string(),
                &is_acc_gl,
                &net_map.get(&input_fields[0].to_string()).unwrap(),
                &fin_map,
                &mut output_line,
            );
        }
        if al_map.contains_key(&input_fields[0].to_string()) {
            let net_bal = net_bal_map
                .get(&input_fields[0].to_string().parse::<i64>().unwrap_or(0))
                .expect("");
            if net_bal >= &0.0 {
                get_output_writer(
                    &config_params,
                    &input_fields,
                    &"A".to_string(),
                    &is_acc_gl,
                    &al_map.get(&input_fields[0].to_string()).unwrap(),
                    &fin_map,
                    &mut output_line,
                );
            } else {
                get_output_writer(
                    &config_params,
                    &input_fields,
                    &"L".to_string(),
                    &is_acc_gl,
                    &al_map.get(&input_fields[0].to_string()).unwrap(),
                    &fin_map,
                    &mut output_line,
                );
            }
        }
    }
    //Writing Moc Output
    let mut moc_output_line = String::new();
    for (_key, val) in moc_map.drain() {
            get_moc_writer(
                &config_params,
                &val,
                &"C".to_string(),
                &"N".to_string(),
                &mut moc_output_line,
            );
            get_moc_writer(
                &config_params,
                &val,
                &"D".to_string(),
                &"N".to_string(),
                &mut moc_output_line,
            );
    }
    let mut out_accounts: Vec<Account> = Vec::new();
    write!(op_writer, "{}", &output_line).expect("Unable to write output file.");
    let output_lines: Vec<&str> = output_line.split('\n').collect();
    for line in output_lines.iter() {
        if *line == "" {
            continue;
        }
        let output_fields: Vec<&str> = line.split('|').collect();
        if output_fields.len() != 14 {
            log_error!(
                log,
                "Could not write data for GL `{}` in CF-Output from Input-Master",
                output_fields[0],
            );
            continue;
        }
        tot_cf_acc += 1;
        tot_amt_in_op += output_fields[4].to_string().parse::<f64>().unwrap_or(0.0);
        out_accounts.push(create_cf_acc(output_fields, &log));
    }
    write!(op_writer, "{}", &moc_output_line)
        .expect("Unable to write moc accounts in output file.");
    let moc_output_lines: Vec<&str> = moc_output_line.split('\n').collect();
    for line in moc_output_lines.iter() {
        if *line == "" {
            continue;
        }
        let moc_output_fields: Vec<&str> = line.split('|').collect();
        if moc_output_fields.len() != 14 {
            log_error!(
                log,
                "Could not write data for GL `{}` in CF-Output from Moc-File",
                moc_output_fields[0]
            );
            continue;
        }
        tot_cf_acc += 1;
        tot_amt_in_op += moc_output_fields[4]
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        out_accounts.push(create_cf_acc(moc_output_fields, &log));
    }
    for acc in out_accounts.iter() {
        cf_writer.write(acc.clone());
    }

    let end_process_timer = SystemTime::now();
    let total_duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration.");
    println!("Total time take: {:?}", total_duration);
    log_info!(log, "Total time take: {:?}", total_duration);
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skp,
        tot_acc_skp,
        tot_amt_in_ip,
        tot_amt_in_op,
        tot_cf_acc,
    );
    println!("{}", health_report.display());
    log_info!(log, "{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}
