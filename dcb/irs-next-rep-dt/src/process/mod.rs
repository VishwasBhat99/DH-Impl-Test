use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use rbdate::{NaiveDate, incr_dt_by_mon_presrv_eom_checked, increment_date_by_months};
use slog::Logger;
use std::collections::HashMap;
use std::default;
use std::{fs, io::Write};
use chrono::prelude::*;
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    //Reading Mapping master File
    let mut rep_master_map: HashMap<String, NaiveDate> = HashMap::new();
    let rep_master_file_path=config_params.rep_master_file_path();
    if rep_master_file_path.ends_with(".xlsx") {
        let mut master_file_path = open_workbook_auto(config_params.rep_master_file_path()).expect("Unable to open the repricing master xlsx file.");
        println!(
            "Sheets present in Master-File: `{:?}`",
            master_file_path.sheet_names()
        );
        if !master_file_path
            .sheet_names()
            .contains(&config_params.rep_sheet_name().to_string())
        {
            panic!(
                "Sheet passed: `{}` not present in Master-File: `{}`",
                config_params.rep_sheet_name(),
                config_params.rep_master_file_path()
            );
        }
        println!(
            "Reading Sheet: `{}` from Master-File",
            config_params.rep_sheet_name(),
        );
        if let Some(Ok(master_file_reader)) =
        master_file_path.worksheet_range(&config_params.rep_sheet_name())
    {
        for (row_no, row) in master_file_reader.rows().enumerate().skip(1) {
            let rep_data = RepMaster::new_from_excel(row);
            let benchmark_code=rep_data.bench_mark_code;
            let vec_f1:Vec<&str>=rep_data.f1.split('-').collect();
            let as_on_dt=config_params.as_on_date();
            let incr_bucket=rep_data.incr_bucket.parse::<u16>().unwrap_or(0);
            let mut date_f1=get_date_from_string(vec_f1[0].parse::<u32>().unwrap_or(0),vec_f1[1].parse::<u32>().unwrap_or(0),as_on_dt.year());
            while &date_f1 <= as_on_dt{
               date_f1=increment_date_by_months(date_f1, incr_bucket);
               
            }
            rep_master_map.insert(benchmark_code, date_f1);
        }
    }

    }else if rep_master_file_path.ends_with(".txt"){
        let rep_master_reader=fs::read_to_string(rep_master_file_path).expect("Could not read repricing master file");
        for (line_no, line) in rep_master_reader.lines().enumerate().skip(1) {
            let rep_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
            let rep_data = RepMaster::new(config_params.rep_master_file_path(),&rep_vec,line_no+1);
            let benchmark_code=rep_data.bench_mark_code;
            let vec_f1:Vec<&str>=rep_data.f1.split('-').collect();
            let as_on_dt=config_params.as_on_date();
            let incr_bucket=rep_data.incr_bucket.parse::<u16>().unwrap_or(0);
            let mut date_f1: NaiveDate=get_date_from_string(vec_f1[0].parse::<u32>().unwrap_or(0),vec_f1[1].parse::<u32>().unwrap_or(0),as_on_dt.year());
            while &date_f1 <= as_on_dt{
               date_f1=increment_date_by_months(date_f1, incr_bucket);
               
            }
            rep_master_map.insert(benchmark_code, date_f1);
        }
    }
    let mut op_writer = get_writer(config_params.output_file());

    //Reading Input file
    let recon_file_reader =
        fs::read_to_string(config_params.input_file_path()).expect("Could not read Input file");
    for (line_no, line) in recon_file_reader.lines().enumerate().skip(0) {
        acc_enc +=1;
        acc_proc +=1;
        let input_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let mut input_data: InputData = InputData::new(
            config_params,
            config_params.input_file_path(),
            &input_vec,
            line_no + 1,
        );
        let flag=&input_data.int_type;
        let bench_mark=&input_data.float_rate_bench_mark;
        let mut rep_date=&input_data.next_rep_date;
        if flag.to_uppercase() == "N"{
           rep_date=rep_master_map.get(bench_mark).unwrap_or(rep_date);
           input_data.next_rep_date=rep_date.clone();
        }
        let output_data:InputData=input_data.clone();
        writeln!(op_writer, "{}", format_output(output_data)).expect("Error in Writing Output");
       
    }


    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
pub fn get_date_from_string(day:u32,month:u32,year:i32)->NaiveDate{
    let required_date=NaiveDate::from_ymd(year, month, day);
    return required_date;
}

