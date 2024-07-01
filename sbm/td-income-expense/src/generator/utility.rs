use crate::macros;
use chrono::{Duration, NaiveDate};
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::{collections::HashMap, env::current_dir, fs, io::BufRead};

pub fn get_pnl_bacid_map(
    balm_fc_gsp_file: String,
    pnl_bacid_pos: usize,
    log: &Logger,
) -> HashMap<String, String> {
    let balm_fc_gsp = match new_buf_rdr(&balm_fc_gsp_file) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found BALM FC GSP file: `{}` on location `{}` : {}.",
            balm_fc_gsp_file,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };
    let mut pnl_bacid_map: HashMap<String, String> = HashMap::new();

    for (line_num, lines) in balm_fc_gsp.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                balm_fc_gsp_file,
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();

        if fields.len() < 7 {
            log_debug!(
                log,
                "Insufficient fields detected for Scheme Code {}: Expected 7 fields, found {} at line num {}.",
                fields[0],
                fields.len(),
                line_num+1
            );
            continue;
        }

        pnl_bacid_map.insert(fields[0].to_string(), fields[pnl_bacid_pos - 1].to_string());
    }

    pnl_bacid_map
}

pub fn get_income_master_map(
    income_master_file: String,
    log: &Logger,
) -> HashMap<String, (f64, f64)> {
    let income_master = match new_buf_rdr(&income_master_file) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found INCOME MASTER file: `{}` on location `{}` : {}.",
            income_master_file,
            current_dir()
                .unwrap_or_else(|error| {
                    panic!("Error while getting current directory path: {}", error);
                })
                .display(),
            error
        ),
    };
    let mut income_master_map: HashMap<String, (f64, f64)> = HashMap::new();

    for (line_num, lines) in income_master.lines().enumerate().skip(1) {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                income_master_file,
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();

        if fields.len() < 26 {
            log_debug!(
                log,
                "Insufficient fields detected for Entity ID {}: Expected 26 fields, found {} on line num {}.",
                fields[1],
                fields.len(),
                line_num+1
            );
            continue;
        }

        income_master_map.insert(
            fields[1].to_string(),
            (
                fields[12].to_string().parse().unwrap_or(0.0),
                fields[14].to_string().parse().unwrap_or(0.0),
            ),
        );
    }

    income_master_map
}

pub fn get_exchange_rate_map(cons_curr: String, ccy_path: String) -> HashMap<String, f64> {
    let ccy_file_contents = fs::read_to_string(ccy_path).expect("cannot read currency file");
    let mut currency_map: HashMap<String, f64> = HashMap::new();
    for line in ccy_file_contents.lines() {
        let each_line: Vec<&str> = line.split('|').collect();
        if each_line[1] == cons_curr {
            currency_map.insert(
                each_line[0].to_string(),
                each_line[2]
                    .parse::<f64>()
                    .expect("unable to parse exchange rate"),
            );
        }
    }
    currency_map
}

pub fn parse_date(date_field: &str, line_number: usize, account_id: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(date_field, "%d-%m-%Y") {
        Ok(date) => date,
        Err(_) => match NaiveDate::parse_from_str(date_field, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => date,
            Err(_) => {
                panic!(
                        "Error parsing date on line {}: Account ID '{}'. Expected Date Format YYYY-MM-DD HH-MM-SS",
                        line_number, account_id
                    );
            }
        },
    }
}

pub fn parse_date_unwraped(date_field: &str, default_date: NaiveDate) -> NaiveDate {
    match NaiveDate::parse_from_str(date_field, "%d-%m-%Y") {
        Ok(date) => date,
        Err(_) => match NaiveDate::parse_from_str(date_field, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => date,
            Err(_) => default_date,
        },
    }
}

pub fn is_date_between(start_date: NaiveDate, end_date: NaiveDate, check_date: NaiveDate) -> bool {
    check_date >= start_date && check_date <= end_date
}

pub fn get_td_daily_basis_map(
    td_daily_files: String,
    first_day_of_month: NaiveDate,
    last_day_of_month: NaiveDate,
    log: &Logger,
) -> HashMap<NaiveDate, HashMap<String, (f64, f64)>> {
    let mut td_daily_basis_input_map: HashMap<NaiveDate, HashMap<String, (f64, f64)>> =
        HashMap::new();

    let mut current_day = first_day_of_month - Duration::days(2);

    while current_day <= last_day_of_month {
        let mut td_input_map: HashMap<String, (f64, f64)> = HashMap::new();
        let current_date_folder = current_day.format("%d%m%Y").to_string();
        let td_daily_file = td_daily_files.replace("{ddmmyyyy}", &current_date_folder);

        let td_daily_input = match new_buf_rdr(&td_daily_file) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found TD Daily file: `{}` on location `{}` : {}.",
                td_daily_file,
                current_dir()
                    .unwrap_or_else(|error| {
                        panic!("Error while getting current directory path: {}", error);
                    })
                    .display(),
                error
            ),
        };

        for (line_num, lines) in td_daily_input.lines().enumerate() {
            let line = match lines {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    td_daily_file,
                    line_num + 1,
                    error
                ),
            };
            let fields: Vec<&str> = line.split('|').collect();

            if fields.len() < 20 {
                log_debug!(
                    log,
                    "Insufficient fields detected for Entity ID {}: Expected 20 fields, found {} at line num {} in TD daily file for date {}.",
                    fields[1],
                    fields.len(),
                    line_num+1,
                    current_day
                );
                continue;
            }

            td_input_map.insert(
                fields[1].to_string(),
                (
                    fields[6].to_string().parse().unwrap_or(0.0),
                    fields[8].to_string().parse().unwrap_or(0.0),
                ),
            );
        }

        td_daily_basis_input_map.insert(current_day, td_input_map);

        current_day += Duration::days(1)
    }
    td_daily_basis_input_map
}
