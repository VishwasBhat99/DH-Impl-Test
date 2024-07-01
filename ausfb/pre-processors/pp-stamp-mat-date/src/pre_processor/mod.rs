extern crate csv;
extern crate serde;
use calamine::{open_workbook_auto, Reader};
use chrono::{Datelike, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::increment_date_by_months;
use rbdate::{datevalue_to_naive_date, get_month_end_date, incr_dt_by_mon_presrv_eom};
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{} Cannot read output file path", error),
    };
    let start_timer = SystemTime::now();

    let mut tot_rec = 0;
    let mut succ_rec = 0;
    let month_long = vec![
        "JANRUARY".to_string(),
        "FEBRUARY".to_string(),
        "MARCH".to_string(),
        "APRIL".to_string(),
        "MAY".to_string(),
        "JUNE".to_string(),
        "JULY".to_string(),
        "AUGUST".to_string(),
        "SEPTEMBER".to_string(),
        "OCTOBER".to_string(),
        "NOVEMBER".to_string(),
        "DECEMBER".to_string(),
    ];
    let month_short = vec![
        "JAN".to_string(),
        "FEB".to_string(),
        "MAR".to_string(),
        "APR".to_string(),
        "MAY".to_string(),
        "JUN".to_string(),
        "JUL".to_string(),
        "AUG".to_string(),
        "SEP".to_string(),
        "OCT".to_string(),
        "NOV".to_string(),
        "DEC".to_string(),
    ];
    let mut output_writer = BufWriter::new(output_file);
    let master_file_extension = Path::new(config_param.master_file_path())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("txt");
    let mut master_map: HashMap<String, Vec<String>> = HashMap::new();
    //Mapping master File reading started
    log_debug!(log, "Mapping master File reading started");
    if master_file_extension == "xlsx" || master_file_extension == "xls" {
        let mut master_excel = open_workbook_auto(config_param.master_file_path())
            .expect("Unable to open Mapping Master File.");

        if let Some(Ok(reader)) = master_excel.worksheet_range(config_param.master_sheet_name()) {
            for row in reader.rows().skip(0) {
                let mut master_vec: Vec<String> = Vec::new();
                for data in row {
                    master_vec.push(data.to_string().trim().to_string());
                }
                let gl_code = row[0].to_string();
                master_map.insert(gl_code, master_vec);
            }
        }
    } else {
        let master_file = match new_buf_rdr(config_param.master_file_path()) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found master_file: `{}`",
                config_param.master_file_path(),
            ),
        };

        for (line_num, lines) in master_file.lines().enumerate().skip(1) {
            let master_line = match lines {
                Ok(master_line) => master_line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.master_file_path(),
                    line_num + 1,
                    error
                ),
            };
            let master_fields = master_line
                .split('|')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            let gl_code = master_fields[0].to_string();
            master_map.insert(gl_code, master_fields);
        }
    }
    log_debug!(log, "Master File Reading Completed");

    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input_file: `{}`",
            config_param.input_file_path(),
        ),
    };

    //input file reading started
    for (line_num, lines) in input_file.lines().enumerate().skip(1) {
        let input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        tot_rec += 1;
        let mut input_fields: Vec<String> = input_line
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();
        if !master_map.contains_key(&input_fields[0]) {
            log_debug!(
                log,
                "Acccount skipped `{}` becasue MaterFile does not contains this value",
                input_fields[0]
            );
            continue;
        }
        succ_rec += 1;
        let master_data = master_map.get(&input_fields[0]).unwrap();
        let mut output_line = "".to_string();
        //For empty currency field map it with cnfig parameter currnecy valu
        if input_fields[6].is_empty() {
            input_fields[6] = config_param.currency().to_string();
        }
        let input_line = input_fields.join("|");
        output_line.push_str(&input_line);
        output_line.push_str("|");
        //Derivation of maturity date on the basis of 4th field (bucket) of master file
        let bucket = master_data[3].as_str();
        let mat_date = mat_dt_cal(&month_long, &month_short, config_param.as_on_date(), bucket)
            .format("%d-%m-%Y")
            .to_string();
        output_line.push_str(&mat_date);
        output_line.push_str("|");
        let master_line = master_data.join("|");
        //Add the master fields in output file
        output_line.push_str(&master_line);
        writeln!(output_writer, "{}", output_line).expect("output_line can not be written");
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total process duration.");
    log_debug!(
        log,
        "Total Duration for preprocess the data: {:?}.",
        duration
    );
    info!(
        diag_log,
        "Total Duration for preprocess the data: {:?}.", duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(&config_param.output_file_path());
}

fn mat_dt_cal(
    month_long: &Vec<String>,
    month_short: &Vec<String>,
    as_on_date: &NaiveDate,
    bucket: &str,
) -> NaiveDate {
    let as_on_month = as_on_date.month();
    let as_on_year = as_on_date.year();
    let as_on_day = as_on_date.day();
    let bucket_upper_case = bucket.trim().to_uppercase();
    //When bucket is like as 10 oct (yearly)
    if bucket_upper_case.contains("(YEARLY)") {
        let month_date_vec: Vec<String> = bucket_upper_case
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let yearly_day = month_date_vec[0].parse().unwrap_or(0);
        let yearly_month = match month_long.iter().position(|x| x == &month_date_vec[1]) {
            Some(x) => x + 1,
            None => match month_short.iter().position(|x| x == &month_date_vec[1]) {
                Some(x) => x + 1,
                None => panic!(
                    "for bucket {},the given month is not in correct formate",
                    bucket
                ),
            },
        };
        let current_date =
            match NaiveDate::from_ymd_opt(as_on_year, yearly_month as u32, yearly_day) {
                Some(date) => date,
                None => get_month_end_date(
                    NaiveDate::from_ymd_opt(as_on_year, yearly_month as u32, 1)
                        .unwrap_or(*as_on_date),
                ),
            };
        let mat_date = if *as_on_date <= current_date {
            current_date
        } else {
            match NaiveDate::from_ymd_opt(as_on_year + 1, yearly_month as u32, yearly_day) {
                Some(date) => date,
                //date get out of range in case of february
                None => get_month_end_date(
                    NaiveDate::from_ymd_opt(as_on_year + 1, yearly_month as u32, 1)
                        .unwrap_or(*as_on_date),
                ),
            }
        };
        return mat_date;
    }
    let mat_date = match NaiveDate::parse_from_str(bucket, "%d-%m-%y") {
        //Given bucket value as a date in dd-mm-yy format
        Ok(x) => x,
        Err(err) => match NaiveDate::parse_from_str(bucket, "%d-%m-%Y") {
            //Given  bucket value as a date on dd-mm-yyyy format
            Ok(x) => x,
            //Given  bucket value as a date on dd-mom-yyyy format
            Err(err) => match NaiveDate::parse_from_str(bucket, "%d-%b-%Y") {
                Ok(x) => x,
                Err(err) => {
                    if bucket.parse::<u64>().unwrap_or(0) >= 1
                        && bucket.parse::<u64>().unwrap_or(0) <= 31
                    {
                        effective_date_cal_from_day(as_on_date, bucket.parse().unwrap_or(0))
                    } else {
                        //if date value is given in the bucket
                        match datevalue_to_naive_date(bucket) {
                            Ok(x) => x,
                            Err(err) => match bucket_upper_case.as_str() {
                                "JANUARY" | "JAN" | "JA" => {
                                    effective_date_cal_from_month(as_on_date, 1)
                                }
                                "FEBRUARY" | "FEB" | "FE" => {
                                    effective_date_cal_from_month(as_on_date, 2)
                                }
                                "MARCH" | "MAR" | "MR" => {
                                    effective_date_cal_from_month(as_on_date, 3)
                                }
                                "APRIL" | "APR" | "AP" => {
                                    effective_date_cal_from_month(as_on_date, 4)
                                }
                                "MAY" | "MY" => effective_date_cal_from_month(as_on_date, 5),
                                "JUNE" | "JUN" | "JN" => {
                                    effective_date_cal_from_month(as_on_date, 6)
                                }
                                "JULY" | "JUL" | "JL" => {
                                    effective_date_cal_from_month(as_on_date, 7)
                                }
                                "AUGUST" | "AUG" | "AU" => {
                                    effective_date_cal_from_month(as_on_date, 8)
                                }
                                "SEPTEMBER" | "SEP" | "SE" => {
                                    effective_date_cal_from_month(as_on_date, 9)
                                }
                                "OCTOBER" | "OCT" | "OC" => {
                                    effective_date_cal_from_month(as_on_date, 10)
                                }
                                "NOVEMBER" | "NOV" | "NV" => {
                                    effective_date_cal_from_month(as_on_date, 11)
                                }
                                "DECEMBER" | "DEC" | "DE" => {
                                    effective_date_cal_from_month(as_on_date, 12)
                                }
                                "QUARTERLY" => {
                                    if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 1, 1)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 6, 30)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 6, 30)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 6, 30)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 12, 31)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 12, 31)
                                            .unwrap_or(*as_on_date)
                                    } else {
                                        NaiveDate::from_ymd_opt(as_on_year + 1, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    }
                                }
                                "ADVANCE TAX QUARTERLY" | "ADVANCE-TAX-QUARTERLY" => {
                                    if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year - 1, 12, 16)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 3, 15)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 3, 15)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 3, 15)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 6, 15)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 6, 15)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 6, 15)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 9, 15)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 9, 15)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 9, 15)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 12, 15)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 12, 15)
                                            .unwrap_or(*as_on_date)
                                    }
                                    //last remaining 15 days in the current as_on_year from 15-dec to 31-dec
                                    else {
                                        NaiveDate::from_ymd_opt(as_on_year + 1, 3, 15)
                                            .unwrap_or(*as_on_date)
                                    }
                                }
                                "HALF YEARLY" | "HALF-YEARLY" => {
                                    if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 1, 1)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 6, 30)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 6, 30)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 6, 30)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 12, 31)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 12, 31)
                                            .unwrap_or(*as_on_date)
                                    } else {
                                        NaiveDate::from_ymd_opt(as_on_year + 1, 6, 30)
                                            .unwrap_or(*as_on_date)
                                    }
                                }
                                "YEARLY" => match NaiveDate::from_ymd_opt(
                                    as_on_year + 1,
                                    as_on_month,
                                    as_on_day,
                                ) {
                                    Some(date) => date,
                                    //in case of february date will get out of range
                                    None => get_month_end_date(
                                        NaiveDate::from_ymd_opt(as_on_year + 1, as_on_month, 1)
                                            .unwrap_or(*as_on_date),
                                    ),
                                },
                                "YEAR END" | "YEAR-END" => {
                                    NaiveDate::from_ymd_opt(as_on_year, 12, 31)
                                        .unwrap_or(*as_on_date)
                                }
                                "FINANCIAL YEAR END" | "FINANCIAL-YEAR-END" => {
                                    if *as_on_date
                                        < NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    } else {
                                        NaiveDate::from_ymd_opt(as_on_year + 1, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    }
                                }
                                "TDS" => {
                                    if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                            .unwrap_or(*as_on_date)
                                    } else if *as_on_date
                                        >= NaiveDate::from_ymd_opt(as_on_year, 9, 30)
                                            .unwrap_or(*as_on_date)
                                        && *as_on_date
                                            < NaiveDate::from_ymd_opt(as_on_year + 1, 3, 31)
                                                .unwrap_or(*as_on_date)
                                    {
                                        NaiveDate::from_ymd_opt(as_on_year + 1, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    }
                                    //this case will handle the date from 01-01-yyyy to 30-03-yyyy for current as_on_date year
                                    else {
                                        NaiveDate::from_ymd_opt(as_on_year, 3, 31)
                                            .unwrap_or(*as_on_date)
                                    }
                                }
                                _ => *as_on_date,
                            },
                        }
                    }
                }
            },
        },
    };

    mat_date
}
fn effective_date_cal_from_day(as_on_date: &NaiveDate, day: u32) -> NaiveDate {
    let cal_date = match NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month(), day) {
        Some(date) => date,
        //if the date goes out of index in case of february
        None => {
            //increment the date by one month
            match NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month() + 1, day) {
                Some(date) => date,
                None => get_month_end_date(*as_on_date),
            }
        }
    };
    if *as_on_date < cal_date {
        cal_date
    } else {
        //next month date
        if as_on_date.month() == 12 {
            increment_date_by_months(cal_date, 1)
        } else {
            match NaiveDate::from_ymd_opt(cal_date.year(), cal_date.month() + 1, day) {
                Some(date) => date,
                //if the date goes out of index in case of february
                None => get_month_end_date(increment_date_by_months(cal_date, 1)),
            }
        }
    }
}
fn effective_date_cal_from_month(as_on_date: &NaiveDate, month: u32) -> NaiveDate {
    let first_day_cal_date =
        NaiveDate::from_ymd_opt(as_on_date.year(), month, 1).unwrap_or(*as_on_date);
    let month_end_cal_date = get_month_end_date(first_day_cal_date);
    if *as_on_date < month_end_cal_date {
        month_end_cal_date
    } else {
        //next year month end date
        get_month_end_date(
            NaiveDate::from_ymd_opt(as_on_date.year() + 1, month, 1).unwrap_or(*as_on_date),
        )
    }
}
