extern crate serde;
extern crate serde_json;
use calamine::Reader;
use calamine::{open_workbook_auto, Sheets};
use chrono::Datelike;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::*;
use slog::Logger;
use std::collections::HashSet;

use crate::process::account_writer::AccountWithoutCashflows;
use crate::process::maturity::{AccountWithCashflows, Cashflow};
mod account_writer;
mod maturity;

pub fn process(config_params: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let mut tot_cfs = 0;
    let as_on_dt = *config_params.as_on_date();
    let mut writer = AccountWithoutCashflows::new(config_params.output_file_path(), log);

    //skip_rows:
    let mut skip_row_set: HashSet<usize> = HashSet::new();
    let mut max_limit = 100;
    for row in config_params.skip_rows().iter() {
        skip_row_set.insert(row.parse::<usize>().unwrap_or(0) - 1);
        if row.contains("GT") {
            max_limit = row.trim_start_matches("GT").parse::<usize>().unwrap_or(0) - 1;
        }
        if row.contains("-") {
            let up_low_limit:Vec<&str> = row.split("-").collect();
            let low_limit = up_low_limit[0].parse::<usize>().unwrap_or(0) - 1;
            let high_limit = up_low_limit[1].parse::<usize>().unwrap_or(0) - 1;
            for i in low_limit..high_limit {
                skip_row_set.insert(i);
            }
        }
    }

    //read input file:
    let mut input_excel = open_workbook_auto(config_params.input_file_path()).expect(&format!(
        "Unable to open Daily CF file, on path {}",
        config_params.input_file_path()
    ));
    let sheet_name = get_sheet_name(as_on_dt);
    check_sheet_name(
        config_params.input_file_path().to_owned(),
        &sheet_name,
        &input_excel,
    );
    if let Some(Ok(reader)) = input_excel.worksheet_range(&sheet_name) {
        for (row_num, row) in reader.rows().enumerate() {
            if reader.rows().len() >= max_limit && row_num > max_limit {
                break;
            }
            if skip_row_set.contains(&row_num) {
                continue;
            }
            tot_rec += 1;
            let days_in_month = get_days_from_month(*config_params.as_on_date()) as u64;
            let acc_name = row[0].to_string();
            for i in 1..=days_in_month {
                tot_cfs += 1;
                let inp_date = NaiveDate::from_ymd_opt(as_on_dt.year(), as_on_dt.month(), i as u32)
                    .unwrap_or(as_on_dt);
                let cf_date = incr_dt_by_days(as_on_dt, i as i64);
                let prin_amt = row[i as usize]
                    .to_string()
                    .trim_matches(',')
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    * config_params.denomination().parse::<f64>().unwrap_or(1.0);
                let mut acc = AccountWithCashflows::new();
                acc.acc_no = acc_name.clone();
                acc.currency = config_params.currency().to_owned();
                acc.input_date = timestamp(inp_date);
                acc.tenor = i.to_string();
                let cf = new_cashflow(0.0, prin_amt, timestamp(cf_date));
                acc.cashflows = protobuf::RepeatedField::from_vec(vec![cf]);
                writer.write(acc);
            }
            succ_rec += 1;
        }
    }
    writer.close();
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, tot_cfs);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file_path());
}

fn get_sheet_name(date: NaiveDate) -> String {
    date.format("%b%y").to_string()
}

fn new_cashflow(i_a: f64, p_a: f64, d: i64) -> Cashflow {
    let mut cf = Cashflow::new();
    cf.interest_amount = i_a;
    cf.principal_amount = p_a;
    cf.date = d;
    cf
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
