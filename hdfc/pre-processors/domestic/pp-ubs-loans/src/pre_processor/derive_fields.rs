use chrono::Datelike;
use macros;
use rbdate::NaiveDate;
use slog::Logger;
use std::collections::HashMap;

use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug)]
pub struct RateCodeMaster {
    pub interpretation: String,
    pub rate_type: String,
    pub rate_flag: String,
    pub days_added_to_bus_dt: String,
    pub reset_freq: String,
    pub reset_month: String,
    pub reset_day: String,
    pub override_sys_reset_dt: String,
}

pub fn append_as_on_date(output_line: &mut String, as_on_date: NaiveDate) {
    output_line.push_str(&as_on_date.format("%d-%m-%Y").to_string());
    // To Add: Missing Interest Basis, Interest Calc Type,
    output_line.push_str("|||");
}

pub fn append_alm_ia_balm_line(
    output_line: &mut String,
    ora_mis1: &str,
    ref_map3: &HashMap<String, String>,
    ora_prod: &str,
    alm_line: &HashMap<String, String>,
    ia_llg: &HashMap<String, String>,
    balm_llg: &HashMap<String, String>,
    key: &str,
    log: &Logger,
    acc_num: &str,
) -> String {
    let mut temp_alm_concat: String = String::new();
    temp_alm_concat.push_str(ora_mis1);
    temp_alm_concat.push('_');
    temp_alm_concat.push_str(ora_prod);
    temp_alm_concat.push('_');
    match ref_map3.get(key) {
        Some(val) => temp_alm_concat.push_str(val),
        None => temp_alm_concat.push_str(""),
    };
    let alm_line = match alm_line.get(&temp_alm_concat) {
        Some(val) => val,
        None => "NONE",
    };
    let ia_llg = match ia_llg.get(&temp_alm_concat) {
        Some(val) => val,
        None => "NONE",
    };
    let balm_llg = match balm_llg.get(&temp_alm_concat) {
        Some(val) => val,
        None => "NONE",
    };
    log_debug!(
        log,
        "Account: `{}`, concat: `{}`, Alm Line: `{}`, IA LLG: `{}`, BALM LLG: `{}`",
        acc_num,
        temp_alm_concat,
        alm_line,
        ia_llg,
        balm_llg
    );
    log_debug!(log, "Alm line for account {} is {}", acc_num, alm_line);
    output_line.push_str(&temp_alm_concat);
    output_line.push_str("|");
    output_line.push_str(alm_line);
    output_line.push_str("|");
    output_line.push_str(ia_llg);
    output_line.push_str("|");
    output_line.push_str(balm_llg);
    output_line.push_str("|");

    if alm_line == "NONE" {
        get_concat_line(acc_num, key, &temp_alm_concat)
    } else {
        String::new()
    }
}

fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("UBSLoans|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

pub fn append_rep_freq(rate_code_master: &HashMap<String, RateCodeMaster>, key: &str) -> String {
    match rate_code_master.get(key) {
        Some(val) => {
            if val.reset_freq == "" {
                "NONE".to_string()
            } else {
                val.reset_freq.to_string()
            }
        }
        None => "NONE".to_string(),
    }
}

pub fn append_last_rep_date(
    rep_freq: &str,
    next_rep_date: NaiveDate,
    val_date: NaiveDate,
    log: &Logger,
) -> String {
    let last_rep_date: NaiveDate = match rep_freq {
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
        _ => {
            log_error!(
                log,
                "`Taking default last reprising date : '{}' for reprising frequency : `{}` .",
                val_date,
                rep_freq
            );
            val_date
        }
    };
    format!("{}", last_rep_date.format("%d-%m-%Y")).to_string()
}

pub fn append_next_rep_date(
    rep_freq: &str,
    last_rep_date: NaiveDate,
    as_on_dt: NaiveDate,
    log: &Logger,
) -> String {
    let next_rep_date: NaiveDate = match rep_freq {
        "ANNUAL" => rbdate::incr_dt_by_mon_presrv_eom(last_rep_date, 12)
            .expect("Cannot derive `next repricing date`."),
        "MONTHLY" => rbdate::incr_dt_by_mon_presrv_eom(last_rep_date, 1)
            .expect("Cannot derive `next repricing date`."),
        "BI MONTHLY" => rbdate::incr_dt_by_mon_presrv_eom(last_rep_date, 2)
            .expect("Cannot derive `next repricing date`."),
        "QUARTERLY" => rbdate::incr_dt_by_mon_presrv_eom(last_rep_date, 3)
            .expect("Cannot derive `next repricing date`."),
        "HALF YEARLY" => rbdate::incr_dt_by_mon_presrv_eom(last_rep_date, 6)
            .expect("Cannot derive `next repricing date`."),
        _ => {
            log_error!(
                log,
                "`Taking default last reprising date : '{}' for reprising frequency : `{}` .",
                as_on_dt,
                rep_freq
            );
            as_on_dt
        }
    };
    format!("{}", next_rep_date.format("%d-%m-%Y")).to_string()
}

pub fn append_next_rep_dt(
    rate_code_master: &HashMap<String, RateCodeMaster>,
    rep_freq: &str,
    reprice_index: &str,
    as_on_date: NaiveDate,
    maturity_date: &str,
    rate_flag: &str,
    log: &Logger,
) -> NaiveDate {
    let next_rep_dt: NaiveDate;
    if rep_freq == "" {
        next_rep_dt = default_next_repricing_date(reprice_index, as_on_date, maturity_date);
    } else {
        if rate_flag == "V" {
            let rate_code = rate_code_master.get(reprice_index);
            if rate_code.is_none() {
                next_rep_dt = default_next_repricing_date(reprice_index, as_on_date, maturity_date);
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
                        get_month_value(&rate_code.unwrap().reset_month[..]);
                        let mut reset_month: u32 =
                            get_month_value(&rate_code.unwrap().reset_month[..]);
                        let as_on_month = as_on_date.month();
                        let reset_day: u32 = rate_code.unwrap().reset_day.parse().unwrap_or(7);
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
                            if reset_month < as_on_month {
                                next_rep_dt = NaiveDate::from_ymd_opt(
                                    as_on_date.year() + 1,
                                    reset_month,
                                    reset_day,
                                )
                                .unwrap_or(as_on_date);
                            } else {
                                next_rep_dt = NaiveDate::from_ymd_opt(
                                    as_on_date.year(),
                                    reset_month,
                                    reset_day,
                                )
                                .unwrap_or(as_on_date);
                            }
                        } else {
                            match &rep_freq[..] {
                                "ANNUAL" => {
                                    if reset_month == 0 {
                                        next_rep_dt = default_next_repricing_date(
                                            reprice_index,
                                            as_on_date,
                                            maturity_date,
                                        );
                                    } else {
                                        if reset_month > as_on_month {
                                            next_rep_dt = NaiveDate::from_ymd_opt(
                                                as_on_date.year(),
                                                reset_month,
                                                reset_day,
                                            )
                                            .unwrap_or(as_on_date);
                                        } else {
                                            next_rep_dt = NaiveDate::from_ymd_opt(
                                                as_on_date.year() + 1,
                                                reset_month,
                                                reset_day,
                                            )
                                            .unwrap_or(as_on_date);
                                        }
                                    }
                                }
                                "MONTHLY" => {
                                    if reset_month == 0 {
                                        if as_on_month == 12 {
                                            reset_month = 1;
                                        } else {
                                            reset_month = as_on_month + 1;
                                        }
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap_or(as_on_date);
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap_or(as_on_date);
                                    }
                                }
                                "QUARTERLY" => {
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
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap_or(as_on_date);
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap_or(as_on_date);
                                    }
                                }
                                "HALF YEARLY" => {
                                    if reset_month == 0 {
                                        if as_on_month < 7 {
                                            reset_month = 7;
                                        } else {
                                            reset_month = 1;
                                        }
                                    }
                                    if reset_month > as_on_month {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year(),
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap_or(as_on_date);
                                    } else {
                                        next_rep_dt = NaiveDate::from_ymd_opt(
                                            as_on_date.year() + 1,
                                            reset_month,
                                            reset_day,
                                        )
                                        .unwrap_or(as_on_date);
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
            log_error!(
                log,
                "`Taking default next reprising date : 01-01-1900 for reprising frequency : `{}` .",
                rep_freq
            );
            next_rep_dt = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap_or(as_on_date);
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
            next_rep_date = NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month(), 7)
                .unwrap_or(as_on_date);
        } else {
            if as_on_date.month() == 12 {
                next_rep_date =
                    NaiveDate::from_ymd_opt(as_on_date.year() + 1, 1, 7).unwrap_or(as_on_date);
            } else {
                next_rep_date =
                    NaiveDate::from_ymd_opt(as_on_date.year(), as_on_date.month() + 1, 7)
                        .unwrap_or(as_on_date);
            }
        }
    } else {
        next_rep_date = NaiveDate::parse_from_str(maturity_date, "%d-%m-%Y")
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

//TODO: Add this to rbdate
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
