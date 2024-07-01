extern crate chrono;
extern crate rbdate;
extern crate sdb_dyn_proto_rdr;
extern crate sdb_io;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use chrono::{Datelike, Duration};
use rbdate::{date_from_timestamp, get_days_from_month, NaiveDate};
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccFieldNames {
    pub account_number: String,
    pub concat: String,
    pub rate_flag: String,
    pub repricing_index: String,
    pub customer_id: String,
    pub maturity_date: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file =
            sdb_io::open_file_read(_path).expect("Cannot open the account metadata file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string");
        let req_fields: AccFieldNames = serde_json::from_str(&buf[..])
            .expect("Account metadata json file was not well-formatted");
        req_fields
    }
}

pub fn get_alm_data_map(
    alm_input_path: &str,
    concat_pos: usize,
    alm_line_pos: usize,
    delimiter: char,
) -> HashMap<String, String> {
    let mut alm_map = HashMap::new();
    let file = File::open(alm_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let scheme_id = split[concat_pos - 1].to_string();
        let alm_line = split[alm_line_pos - 1].to_string();
        alm_map.insert(scheme_id, alm_line);
    }
    alm_map
}

pub fn get_alm_line<'a>(
    map: &'a HashMap<String, String>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
) -> Option<&'a String> {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let comcat: String = account
        .get_string_for_key(&keys.concat)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    map.get(&comcat)
}

pub fn get_npa_class_map(
    npa_class_input_path: &str,
    account_number_pos: usize,
    asset_class_pos: usize,
    delimiter: char,
) -> HashMap<String, String> {
    let mut npa_map = HashMap::new();
    let file = File::open(npa_class_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[account_number_pos - 1].to_string();
        let asset_class = split[asset_class_pos - 1].to_string();
        npa_map.insert(account_number, asset_class);
    }
    npa_map
}

pub fn get_npa_class<'a>(
    map: &'a HashMap<String, String>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
) -> Option<&'a String> {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let account_number: String = account
        .get_string_for_key(&keys.account_number)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    map.get(&account_number)
}

pub fn get_cust_id_map(
    cust_id_input_path: &str,
    hdfcltd_custid_pos: usize,
    hdfcbank_custid_pos: usize,
    delimiter: char,
) -> HashMap<String, String> {
    let mut alm_map = HashMap::new();
    let file = File::open(cust_id_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let hdfcltd_custid = split[hdfcltd_custid_pos - 1].to_string();
        let hdfcbank_custid = split[hdfcbank_custid_pos - 1].to_string();
        alm_map.insert(hdfcltd_custid, hdfcbank_custid);
    }
    alm_map
}

pub fn get_hdfcbank_custid<'a>(
    map: &'a HashMap<String, String>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
) -> String {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let hdfcltd_custid: String = account
        .get_string_for_key(&keys.customer_id)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    map.get(&hdfcltd_custid)
        .unwrap_or(&hdfcltd_custid)
        .to_string()
}

#[derive(Debug)]
pub struct RateCodeMaster {
    pub interpretation: String,
    pub rate_flag: String,
    pub days_added_to_bus_dt: String,
    pub reset_freq: String,
    pub reset_month: String,
    pub reset_day: String,
    pub override_sys_reset_dt: String,
}

#[derive(Debug)]
pub struct RateCodeMasterFieldPosition {
    pub rate_code_pos: usize,
    pub interpretation_pos: usize,
    pub rate_flag_pos: usize,
    pub days_added_to_bus_dt_pos: usize,
    pub reset_freq_pos: usize,
    pub reset_month_pos: usize,
    pub reset_day_pos: usize,
    pub override_sys_reset_dt_pos: usize,
}

pub fn get_rate_code_map(
    rate_code_master_input_path: &str,
    ratecodeposition: RateCodeMasterFieldPosition,
    delimiter: char,
) -> HashMap<String, RateCodeMaster> {
    let mut rate_code_map = HashMap::new();
    let file = File::open(rate_code_master_input_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let rate_code = split[ratecodeposition.rate_code_pos - 1].to_string();
        let rate_code_master = RateCodeMaster {
            interpretation: split[ratecodeposition.interpretation_pos - 1]
                .to_string()
                .trim_matches('"')
                .to_uppercase(),
            rate_flag: split[ratecodeposition.rate_flag_pos - 1].to_string(),
            days_added_to_bus_dt: split[ratecodeposition.days_added_to_bus_dt_pos - 1].to_string(),
            reset_freq: split[ratecodeposition.reset_day_pos - 1].to_string(),
            reset_month: split[ratecodeposition.reset_month_pos - 1].to_string(),
            reset_day: split[ratecodeposition.reset_day_pos - 1].to_string(),
            override_sys_reset_dt: split[ratecodeposition.override_sys_reset_dt_pos - 1]
                .to_string(),
        };
        rate_code_map.insert(rate_code, rate_code_master);
    }
    rate_code_map
}

pub fn get_next_rep_rate(
    map: &HashMap<String, RateCodeMaster>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
    as_on_date: &NaiveDate,
) -> NaiveDate {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let rate_flag: String = account
        .get_string_for_key(&keys.rate_flag)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    let reprice_index: String = account
        .get_string_for_key(&keys.repricing_index)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    let maturity_date: &str =
        &date_from_timestamp(account.get_i64_for_key(&keys.maturity_date).unwrap_or(0))
            .format("%d-%m-%Y")
            .to_string();
    if rate_flag != "Fixed" {
        let rep_freq: &str = &append_rep_freq(&map, &reprice_index);
        let next_rep_date = append_next_rep_dt(
            map,
            rep_freq,
            &reprice_index,
            *as_on_date,
            maturity_date,
            &rate_flag,
        );
        return next_rep_date;
    } else {
        NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()
    }
}

pub fn append_rep_freq(ref_map3: &HashMap<String, RateCodeMaster>, key: &str) -> String {
    let rep_freq = match ref_map3.get(key) {
        Some(val) => {
            if val.reset_freq == "" {
                "NONE".to_string()
            } else {
                val.reset_freq.to_string()
            }
        }
        None => "NONE".to_string(),
    };
    rep_freq
}

pub fn append_next_rep_dt(
    ref_map3: &HashMap<String, RateCodeMaster>,
    rep_freq: &str,
    reprice_index: &str,
    as_on_date: NaiveDate,
    maturity_date: &str,
    rate_flag: &str,
) -> NaiveDate {
    let next_rep_dt: NaiveDate;
    if rep_freq == "" {
        next_rep_dt = default_next_repricing_date(reprice_index, as_on_date, maturity_date);
    } else {
        if rate_flag == "Floating" {
            let rate_code = ref_map3.get(reprice_index);
            if rate_code.is_none() {
                next_rep_dt = NaiveDate::from_ymd_opt(2099, 12, 31).unwrap();
            } else {
                let days_added_to_bus_dt: i64 =
                    rate_code.unwrap().days_added_to_bus_dt.parse().unwrap_or(0);
                if days_added_to_bus_dt != 0 {
                    next_rep_dt = add_days(as_on_date, days_added_to_bus_dt);
                } else {
                    if rep_freq == "" {
                        next_rep_dt =
                            default_next_repricing_date(reprice_index, as_on_date, maturity_date);
                    } else {
                        let mut reset_month: u32 =
                            get_month_value(&rate_code.unwrap().reset_month[..]);
                        let as_on_month = as_on_date.month();
                        let mut reset_day: u32 = rate_code.unwrap().reset_day.parse().unwrap_or(7);
                        if reset_month == 0 && !&rate_code.unwrap().reset_month[..].is_empty() {
                            let mut def_reset_month: u32 = 13;
                            let month_vec: Vec<&str> =
                                rate_code.unwrap().reset_month[..].split('-').collect();
                            let mut new_month_vec: Vec<u32> = Vec::with_capacity(4);
                            for month in month_vec.iter() {
                                let month_value: u32 = get_month_value(month);
                                new_month_vec.push(month_value);
                            }
                            for month in new_month_vec.iter() {
                                if month > &as_on_month && month < &def_reset_month {
                                    reset_month = *month;
                                    def_reset_month = reset_month;
                                }
                            }
                            if reset_month == 0 {
                                reset_month = *new_month_vec
                                    .iter()
                                    .min()
                                    .expect("Cannot find minimum month");
                            }
                            let temp_date;
                            if reset_day > 7 {
                                temp_date = NaiveDate::from_ymd_opt(
                                    as_on_date.year(),
                                    reset_month,
                                    reset_day - 6,
                                );
                            } else {
                                temp_date = NaiveDate::from_ymd_opt(
                                    as_on_date.year(),
                                    reset_month,
                                    reset_day,
                                );
                            }
                            let last_day = get_days_from_month(temp_date.unwrap()) as u32;
                            if reset_day > last_day {
                                reset_day = last_day;
                            }
                            if reset_month < as_on_month {
                                next_rep_dt = NaiveDate::from_ymd_opt(
                                    as_on_date.year() + 1,
                                    reset_month,
                                    reset_day,
                                )
                                .unwrap();
                            } else {
                                next_rep_dt = NaiveDate::from_ymd_opt(
                                    as_on_date.year(),
                                    reset_month,
                                    reset_day,
                                )
                                .unwrap();
                            }
                        } else {
                            match &rep_freq[..].to_uppercase().as_str() {
                                &"ANNUAL" => {
                                    if reset_month == 0 {
                                        next_rep_dt = default_next_repricing_date(
                                            reprice_index,
                                            as_on_date,
                                            maturity_date,
                                        );
                                    } else {
                                        let temp_date;
                                        if reset_day > 7 {
                                            temp_date = NaiveDate::from_ymd_opt(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day - 6,
                                            );
                                        } else {
                                            temp_date = NaiveDate::from_ymd_opt(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day,
                                            );
                                        }
                                        let last_day =
                                            get_days_from_month(temp_date.unwrap()) as u32;
                                        if reset_day > last_day {
                                            reset_day = last_day;
                                        }
                                        if reset_month > as_on_month {
                                            next_rep_dt = NaiveDate::from_ymd_opt(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day,
                                            )
                                            .unwrap();
                                        } else {
                                            next_rep_dt = NaiveDate::from_ymd_opt(
                                                as_on_date.year() + 1,
                                                reset_month,
                                                reset_day,
                                            )
                                            .unwrap();
                                        }
                                    }
                                }
                                &"MONTHLY" => {
                                    if reset_month == 0 {
                                        if as_on_month == 12 {
                                            reset_month = 1;
                                        } else {
                                            reset_month = as_on_month + 1;
                                        }
                                    }
                                    let temp_date;
                                    if reset_day > 7 {
                                        temp_date = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day - 6,
                                        );
                                    } else {
                                        temp_date = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    }
                                    let last_day = get_days_from_month(temp_date.unwrap()) as u32;
                                    if reset_day > last_day {
                                        reset_day = last_day;
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap();
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap();
                                    }
                                }
                                &"QUARTERLY" => {
                                    if reset_month == 0 {
                                        if as_on_month < 4 {
                                            reset_month = 4;
                                        } else if as_on_month < 7 {
                                            reset_month = 7;
                                        } else if as_on_month < 10 {
                                            reset_month = 10;
                                        } else {
                                            reset_month = 1;
                                        }
                                    }
                                    let temp_date;
                                    if reset_day > 7 {
                                        temp_date = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day - 6,
                                        );
                                    } else {
                                        temp_date = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    }
                                    let last_day = get_days_from_month(temp_date.unwrap()) as u32;
                                    if reset_day > last_day {
                                        reset_day = last_day;
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap();
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap();
                                    }
                                }
                                &"HALF YEARLY" => {
                                    if reset_month == 0 {
                                        if as_on_month < 7 {
                                            reset_month = 7;
                                        } else {
                                            reset_month = 1;
                                        }
                                    }
                                    let temp_date;
                                    if reset_day > 7 {
                                        temp_date = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day - 6,
                                        );
                                    } else {
                                        temp_date = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    }
                                    let last_day = get_days_from_month(temp_date.unwrap()) as u32;
                                    if reset_day > last_day {
                                        reset_day = last_day;
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap();
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap();
                                    }
                                }
                                _ => {
                                    next_rep_dt = default_next_repricing_date(
                                        reprice_index,
                                        as_on_date,
                                        maturity_date,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        } else {
            next_rep_dt = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        }
    }
    next_rep_dt
}

fn default_next_repricing_date(
    reprice_index: &str,
    as_on_date: NaiveDate,
    maturity_date: &str,
) -> NaiveDate {
    let next_rep_date;
    if reprice_index.contains("MCLR") {
        if as_on_date.day() < 7 {
            next_rep_date = NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month(), 7);
        } else {
            if as_on_date.month() == 12 {
                next_rep_date = NaiveDate::from_ymd_opt(as_on_date.year() + 1, 1, 7);
            } else {
                next_rep_date =
                    NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month() + 1, 7);
            }
        }
    } else {
        next_rep_date = Some(
            NaiveDate::parse_from_str(maturity_date, "%d-%b-%Y")
                .expect("Cannot parse mat date parameter as a valid date format."),
        );
    }
    next_rep_date.unwrap()
}

fn add_days(as_on_date: NaiveDate, days_added_to_bus_dt: i64) -> NaiveDate {
    as_on_date + Duration::days(days_added_to_bus_dt)
}

// TODO: Add this method to rbdate library
fn get_month_value(month: &str) -> u32 {
    match month {
        "JAN" => 1,
        "FEB" => 2,
        "MAR" => 3,
        "APR" => 4,
        "MAY" => 5,
        "JUN" => 6,
        "JUL" => 7,
        "AUG" => 8,
        "SEP" => 9,
        "OCT" => 10,
        "NOV" => 11,
        "DEC" => 12,
        _ => 0,
    }
}

pub fn get_risk_weight_map(
    risk_weight_file_path: &str,
    delimiter: char,
) -> HashMap<String, String> {
    let mut rw_map: HashMap<String, String> = HashMap::new();
    let file = File::open(risk_weight_file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[0].to_string();
        let rw = split[1].to_string();
        rw_map.insert(account_number, rw);
    }
    rw_map
}

pub fn get_resid_map(resid_file_path: &str, delimiter: char) -> HashMap<String, String> {
    let mut resid_map: HashMap<String, String> = HashMap::new();
    let file = File::open(resid_file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[0].to_string();
        let resid = split[1].to_string();
        resid_map.insert(account_number, resid);
    }
    resid_map
}

pub fn get_restructure_flag_map(
    restructure_flag_file_path: &str,
    delimiter: char,
) -> HashMap<String, String> {
    let mut rf_map: HashMap<String, String> = HashMap::new();
    let file = File::open(restructure_flag_file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let split: Vec<&str> = line.split(delimiter).collect();
        let account_number = split[0].to_string();
        let rf = split[1].to_string();
        rf_map.insert(account_number, rf);
    }
    rf_map
}

pub fn get_rw_data<'a>(
    map: &'a HashMap<String, String>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
) -> String {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let account_number: String = account
        .get_string_for_key(&keys.account_number)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    map.get(&account_number)
        .unwrap_or(&"99.9".to_string())
        .to_string()
}

pub fn get_rf_data<'a>(
    map: &'a HashMap<String, String>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
) -> String {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let account_number: String = account
        .get_string_for_key(&keys.account_number)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    map.get(&account_number)
        .unwrap_or(&"N".to_string())
        .to_string()
}

pub fn get_resid_data<'a>(
    map: &'a HashMap<String, String>,
    required_fields_file_path: &str,
    account: &AccountWithCFs,
) -> String {
    let keys = AccFieldNames::new_from_path(required_fields_file_path);
    let account_number: String = account
        .get_string_for_key(&keys.account_number)
        .unwrap_or(&"NONE".to_string())
        .to_string();
    map.get(&account_number)
        .unwrap_or(&"N".to_string())
        .to_string()
}
