use chrono::Datelike;
use macros;
use rbdate::{get_days_from_month, NaiveDate};
use slog::Logger;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AlmMaster {
    pub alm: String,
    pub coa: String,
    pub al_line: String,
    pub balm_l2: String,
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

pub fn append_as_on_date(output_line: &mut String, as_on_date: NaiveDate) {
    output_line.push_str(&as_on_date.format("%d-%m-%Y").to_string());
    output_line.push_str("|");
}

pub fn append_final_interest_rate(output_line: &mut String, dpd: f64, int_rate: &str) {
    if dpd > 90.0 {
        output_line.push_str("0");
    } else {
        output_line.push_str(int_rate);
    }
    output_line.push_str("|");
}

pub fn append_cost_centre(output_line: &mut String, ref_map2: &HashMap<String, String>, key: &str) {
    let cost_center2 = match ref_map2.get(key) {
        Some(val) => val.to_string(),
        None => "101".to_string(),
    };
    output_line.push_str(&cost_center2[..]);
    output_line.push_str("|");
}

pub fn append_alm_line(
    output_line: &mut String,
    ref_map1: &HashMap<String, AlmMaster>,
    key: &str,
    log: &Logger,
    acc_num: &str,
    gl: &str,
) -> String {
    let mut concat = String::new();
    let alm_line = match ref_map1.get(key) {
        Some(val) => {
            if val.alm == "" {
                "NONE".to_string()
            } else {
                val.alm.to_string()
            }
        }
        None => "NONE".to_string(),
    };
    output_line.push_str(&alm_line[..]);
    output_line.push_str("|");
    log_debug!(
        log,
        "Alm line for account: {} is {} for key: {}",
        acc_num,
        alm_line,
        key
    );

    if alm_line == "NONE" {
        concat.push_str(&get_concat_line(acc_num, gl, key))
    }
    concat
}

fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("FinLoans|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}
pub fn append_coa(output_line: &mut String, ref_map1: &HashMap<String, AlmMaster>, key: &str) {
    let coa = match ref_map1.get(key) {
        Some(val) => {
            if val.coa == "" {
                "NONE".to_string()
            } else {
                val.coa.to_string()
            }
        }
        None => "NONE".to_string(),
    };
    output_line.push_str(&coa[..]);
    output_line.push_str("|");
}

pub fn apppend_division(output_line: &mut String, division: &HashMap<String, String>, key: &str) {
    let division = match division.get(key) {
        Some(div) => {
            if div.is_empty() {
                String::from("NONE")
            } else {
                div.to_string()
            }
        }
        None => "NONE".to_string(),
    };
    output_line.push_str(&division[..]);
    output_line.push_str("|");
}

pub fn append_al_line(output_line: &mut String, ref_map1: &HashMap<String, AlmMaster>, key: &str) {
    output_line.push_str("|");
    let al_line = match ref_map1.get(key) {
        Some(val) => {
            if val.al_line.is_empty() {
                String::from("NONE")
            } else {
                val.al_line.to_string()
            }
        }
        None => String::from("NONE"),
    };
    output_line.push_str(&al_line[..]);
}

pub fn append_balm_l2(output_line: &mut String, ref_map1: &HashMap<String, AlmMaster>, key: &str) {
    output_line.push_str("|");
    let balm_l2 = match ref_map1.get(key) {
        Some(val) => {
            if val.balm_l2.is_empty() {
                String::from("NONE")
            } else {
                val.balm_l2.to_string()
            }
        }
        None => String::from("NONE"),
    };
    output_line.push_str(&balm_l2[..]);
}

pub fn append_rep_freq(
    output_line: &mut String,
    ref_map3: &HashMap<String, RateCodeMaster>,
    key: &str,
) -> String {
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
    match &rep_freq[..].to_uppercase().as_str() {
        &"MONTHLY" => {
            output_line.push_str(&1.to_string());
        }
        &"BI MONTHLY" => {
            output_line.push_str(&2.to_string());
        }
        &"QUARTERLY" => {
            output_line.push_str(&3.to_string());
        }
        &"HALF YEARLY" => {
            output_line.push_str(&6.to_string());
        }
        &"ANNUAL" => {
            output_line.push_str(&12.to_string());
        }
        _ => {
            output_line.push_str("");
        }
    }
    output_line.push_str("|");
    rep_freq
}

pub fn append_last_rep_date(
    output_line: &mut String,
    rep_freq: &str,
    next_rep_date: NaiveDate,
) -> NaiveDate {
    let last_rep_date: NaiveDate = match rep_freq.to_uppercase().as_str() {
        "ANNUAL" => rbdate::decr_dt_by_mon_presrv_eom(next_rep_date, 12)
            .expect("Cannot derive `last repricing date`."),
        "MONTHLY" => rbdate::decr_dt_by_mon_presrv_eom(next_rep_date, 1)
            .expect("Cannot derive `last repricing date`."),
        "BI MONTHLY" => rbdate::decr_dt_by_mon_presrv_eom(next_rep_date, 2)
            .expect("Cannot derive `last repricing date`."),
        "QUARTERLY" => rbdate::decr_dt_by_mon_presrv_eom(next_rep_date, 3)
            .expect("Cannot derive `last repricing date`."),
        "HALF YEARLY" => rbdate::decr_dt_by_mon_presrv_eom(next_rep_date, 6)
            .expect("Cannot derive `last repricing date`."),
        _ => next_rep_date,
    };
    last_rep_date
}

pub fn append_next_rep_dt(
    output_line: &mut String,
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
                next_rep_dt = NaiveDate::from_ymd(2099, 12, 31);
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
                                temp_date = NaiveDate::from_ymd(
                                    as_on_date.year(),
                                    reset_month,
                                    reset_day - 6,
                                );
                            } else {
                                temp_date =
                                    NaiveDate::from_ymd(as_on_date.year(), reset_month, reset_day);
                            }
                            let last_day = get_days_from_month(temp_date) as u32;
                            if reset_day > last_day {
                                reset_day = last_day;
                            }
                            if reset_month < as_on_month {
                                next_rep_dt = NaiveDate::from_ymd(
                                    as_on_date.year() + 1,
                                    reset_month,
                                    reset_day,
                                );
                            } else {
                                next_rep_dt =
                                    NaiveDate::from_ymd(as_on_date.year(), reset_month, reset_day);
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
                                            temp_date = NaiveDate::from_ymd(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day - 6,
                                            );
                                        } else {
                                            temp_date = NaiveDate::from_ymd(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day,
                                            );
                                        }
                                        let last_day = get_days_from_month(temp_date) as u32;
                                        if reset_day > last_day {
                                            reset_day = last_day;
                                        }
                                        if reset_month > as_on_month {
                                            next_rep_dt = NaiveDate::from_ymd(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day,
                                            );
                                        } else {
                                            next_rep_dt = NaiveDate::from_ymd(
                                                as_on_date.year() + 1,
                                                reset_month,
                                                reset_day,
                                            );
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
                                        temp_date = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day - 6,
                                        );
                                    } else {
                                        temp_date = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    }
                                    let last_day = get_days_from_month(temp_date) as u32;
                                    if reset_day > last_day {
                                        reset_day = last_day;
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        );
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
                                        temp_date = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day - 6,
                                        );
                                    } else {
                                        temp_date = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    }
                                    let last_day = get_days_from_month(temp_date) as u32;
                                    if reset_day > last_day {
                                        reset_day = last_day;
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        );
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
                                        temp_date = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day - 6,
                                        );
                                    } else {
                                        temp_date = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    }
                                    let last_day = get_days_from_month(temp_date) as u32;
                                    if reset_day > last_day {
                                        reset_day = last_day;
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        );
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        );
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
            next_rep_dt = NaiveDate::from_ymd(1900, 1, 1);
        }
    }
    if next_rep_dt == NaiveDate::from_ymd(1900, 1, 1) {
        output_line.push_str("|");
    } else {
        output_line.push_str(&format!("{}", next_rep_dt.format("%d-%m-%Y")));
        output_line.push_str("|");
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
            next_rep_date = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month(), 7);
        } else {
            if as_on_date.month() == 12 {
                next_rep_date = NaiveDate::from_ymd(as_on_date.year() + 1, 1, 7);
            } else {
                next_rep_date = NaiveDate::from_ymd(as_on_date.year(), as_on_date.month() + 1, 7);
            }
        }
    } else {
        next_rep_date = NaiveDate::parse_from_str(maturity_date, "%d-%b-%Y")
            .expect("Cannot parse mat date parameter as a valid date format.");
    }
    next_rep_date
}

fn add_days(as_on_date: NaiveDate, days_added_to_bus_dt: i64) -> NaiveDate {
    let mut days_to_add = days_added_to_bus_dt;
    let mut next_date = as_on_date;
    while days_to_add != 0 {
        next_date = next_date.succ();
        days_to_add -= 1;
    }
    next_date
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
