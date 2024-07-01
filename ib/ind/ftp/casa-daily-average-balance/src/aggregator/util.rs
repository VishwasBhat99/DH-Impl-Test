use aggregator::input_account::DailyData;
use chrono::{Datelike, NaiveDate};
use rbdate::date_from_timestamp;
use sdb_io::buf_file_wrtr;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs;
use std::{
    env::current_dir,
    fs::{File, OpenOptions},
};

pub fn is_first_day_of_month(date: NaiveDate) -> bool {
    return date.day() == 01;
}

pub fn first_occurence_of_account(input_account: DailyData, date: NaiveDate, op_writer:&mut BufWriter<File> ) {
    let cls_dt_timestamp = input_account.acc_cls_dt.parse::<i64>().unwrap_or(0);
    let cls_dt = date_from_timestamp(cls_dt_timestamp)
        .format("%d-%m-%Y")
        .to_string()
        .to_string();
    let sum_prod = (input_account.out_bal.parse::<f64>().unwrap_or(0.0).abs() * input_account.int_rt.parse::<f64>().unwrap_or(0.0).abs()).to_string();

    let op_str = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",input_account.acc_num,input_account.out_bal,input_account.int_rt,input_account.int_posted,input_account.int_posted,sum_prod,input_account.curr_status,input_account.class,cls_dt,input_account.gl_cd);
    writeln!(op_writer, "{}", op_str).expect("Unable to generate summary file.");
}

pub fn previous_occurence_of_account(
    input_account: DailyData,
    previous_data: Vec<&str>,
    date: NaiveDate,
    op_writer:&mut BufWriter<File> 
) {
    let cls_dt_timestamp = input_account.acc_cls_dt.parse::<i64>().unwrap_or(0);
    let cls_dt = date_from_timestamp(cls_dt_timestamp)
        .format("%d-%m-%Y")
        .to_string()
        .to_string();
        let sum_out = (previous_data[1].parse::<f64>().unwrap_or(0.0).abs() + input_account.out_bal.parse::<f64>().unwrap_or(0.0).abs()).to_string();
        let sum_int_rt = (previous_data[2].parse::<f64>().unwrap_or(0.0).abs() + input_account.int_rt.parse::<f64>().unwrap_or(0.0).abs()).to_string();
        let sum_int_posted = (previous_data[3].parse::<f64>().unwrap_or(0.0).abs() + input_account.int_posted.parse::<f64>().unwrap_or(0.0).abs()).to_string();
        let sum_prod = (previous_data[5].parse::<f64>().unwrap_or(0.0).abs() + (input_account.out_bal.parse::<f64>().unwrap_or(0.0).abs() * input_account.int_rt.parse::<f64>().unwrap_or(0.0).abs())).to_string();

        let op_str = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",input_account.acc_num,sum_out,sum_int_rt,sum_int_posted,input_account.int_posted,sum_prod,input_account.curr_status,input_account.class,cls_dt,input_account.gl_cd);
        writeln!(op_writer, "{}", op_str).expect("Unable to generate summary file.");
}
