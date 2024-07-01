use super::output_lines::OutputLines;
use chrono::Datelike;
use macros;
use rbdate::date_from_timestamp;
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;
use statics::*;
use std::collections::HashMap;

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

pub fn get_op_line(
    rec: &mut Vec<&str>,
    div: &mut HashMap<String, String>,
    alm_line: &mut HashMap<String, String>,
    ia_llg: &mut HashMap<String, String>,
    balm_llg: &mut HashMap<String, String>,
    t_ora_prod: &mut HashMap<String, String>,
    t_ora_gl: &mut HashMap<String, String>,
    t_ora_cat: &mut HashMap<String, String>,
    cost_center: &mut HashMap<String, String>,
    asset_class: &mut HashMap<String, String>,
    rt_cd: &mut HashMap<String, RateCodeMaster>,
    ccy: &str,
    override_sys_reset_dt: String,
    as_on_dt: NaiveDate,
    bm_id: &str,
    log: &Logger,
    bm_id_lookup: &str,
    val_date: NaiveDate,
    mis2_code: String,
    sprd_map: &mut HashMap<String, Vec<HashMap<NaiveDate, f32>>>,
    npa_map: &mut HashMap<String, String>,
    od_study_map: &mut HashMap<String, Vec<f64>>,
) -> OutputLines {
    let mut output_lines = OutputLines::new();
    let mut t_ora_mis1 = String::new();
    if rec[30].trim() != "" {
        t_ora_mis1.push('1');
        t_ora_mis1.push_str(rec[30]);
    } else {
        t_ora_mis1.push_str("9999");
    }

    let div = div
        .entry(t_ora_mis1.to_string())
        .or_insert_with(|| "".to_string());

    let alm_concat = get_alm_concat(
        &t_ora_mis1,
        t_ora_prod
            .entry(rec[16].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_gl
            .entry(rec[16].to_string())
            .or_insert_with(|| "".to_string()),
        t_ora_cat
            .entry(rec[16].to_string())
            .or_insert_with(|| "".to_string()),
    );

    let alm_line = alm_line
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let ia_llg = ia_llg
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let balm_llg = balm_llg
        .entry(alm_concat.to_string())
        .or_insert_with(|| "NONE".to_string());

    let cost_center = cost_center
        .entry(rec[16].to_string())
        .or_insert_with(|| "".to_string());

    log_debug!(
        log,
        "account: `{}`, alm_concat: `{}`, alm_line: `{}`, division: `{}`.",
        rec[0],
        alm_concat,
        alm_line,
        div
    );

    output_lines.processed_lines = get_line(
        rec,
        &div,
        alm_line,
        ia_llg,
        balm_llg,
        cost_center,
        asset_class,
        rt_cd,
        override_sys_reset_dt,
        as_on_dt,
        ccy,
        bm_id,
        log,
        bm_id_lookup,
        val_date,
        &alm_concat,
        mis2_code,
        sprd_map,
        npa_map,
        od_study_map,
    );

    if alm_line == "NONE" {
        output_lines
            .concat_lines
            .push(get_concat_line(rec[0], rec[16], &alm_concat));
    }

    output_lines
}

fn get_concat_line(deal_no: &str, gl: &str, alm_concat: &str) -> String {
    let mut op_line = String::new();
    op_line.push_str("FC_RD_OD|");
    op_line.push_str(deal_no);
    op_line.push('|');
    op_line.push_str(gl);
    op_line.push('|');
    op_line.push_str(alm_concat);
    op_line
}

fn get_line(
    rec: &mut Vec<&str>,
    div: &str,
    alm_line: &mut String,
    ia_llg: &mut String,
    balm_llg: &mut String,
    cost_center: &str,
    asset_class: &mut HashMap<String, String>,
    rt_cd: &mut HashMap<String, RateCodeMaster>,
    override_sys_reset_dt: String,
    dt: NaiveDate,
    ccy: &str,
    bm_id: &str,
    log: &Logger,
    bm_id_lookup: &str,
    val_date: NaiveDate,
    alm_concat: &str,
    mis2_code: String,
    sprd_map: &mut HashMap<String, Vec<HashMap<NaiveDate, f32>>>,
    npa_map: &mut HashMap<String, String>,
    od_study_map: &mut HashMap<String, Vec<f64>>,
) -> String {
    let mut output_line = String::new();
    output_line.push_str(rec[0]);
    output_line.push('|');
    output_line.push_str(rec[1]);
    output_line.push('|');
    output_line.push_str(rec[2]);
    output_line.push('|');
    output_line.push_str(rec[4]);
    output_line.push('|');
    output_line.push_str(rec[5]);
    output_line.push('|');
    output_line.push_str(rec[6]);
    output_line.push('|');
    output_line.push_str(rec[7]);
    output_line.push('|');
    output_line.push_str(rec[8]);
    output_line.push('|');
    output_line.push_str(rec[9]);
    output_line.push('|');

    let acc_open_dt = NaiveDate::parse_from_str(rec[10], "%d-%b-%Y");
    if let Ok(dt) = acc_open_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_error!(
            log,
            "`account_open_date` is not well-formatted for account: `{}`.",
            rec[0]
        );
    }
    output_line.push('|');

    output_line.push_str(rec[11]);
    output_line.push('|');
    output_line.push_str(rec[14]);
    output_line.push('|');

    let acc_close_dt = NaiveDate::parse_from_str(rec[20], "%d-%b-%Y");
    if let Ok(dt) = acc_close_dt {
        output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    } else {
        log_debug!(
            log,
            "`account_close_date` is not well-formatted for account: `{}`.",
            rec[0]
        );
    }
    output_line.push('|');

    output_line.push_str(rec[23]);
    output_line.push('|');
    output_line.push_str(rec[24]);
    output_line.push('|');
    output_line.push_str(&dt.format("%d-%m-%Y").to_string());
    output_line.push('|');
    output_line.push_str(cost_center);
    output_line.push_str("|15");
    output_line.push_str(rec[16]);
    output_line.push_str("00|V|");
    output_line.push_str(ccy);
    output_line.push('|');
    output_line.push_str(rec[5]);
    output_line.push('|');
    output_line.push_str(rec[11]);
    output_line.push('|');

    let int_rt: f64 = if rec[19].parse::<f64>().unwrap_or(DEFAULT_FLOAT) == 0.0 {
        rec[18].parse().unwrap_or(DEFAULT_FLOAT)
    } else {
        rec[19].parse().unwrap_or(DEFAULT_FLOAT)
    };
    output_line.push_str(&int_rt.to_string());
    output_line.push('|');

    output_line.push_str(div);
    output_line.push('|');
    output_line.push_str(&alm_line);
    output_line.push('|');
    output_line.push_str(&ia_llg);
    output_line.push('|');
    output_line.push_str(&balm_llg);
    output_line.push('|');
    output_line.push_str(rec[30]);
    output_line.push('|');

    let npa_flg = &asset_class
        .entry(rec[0].to_string())
        .or_insert_with(|| "P".to_string());
    output_line.push_str(&npa_flg);
    output_line.push('|');

    let benchmark = bm_id;
    output_line.push_str(benchmark);
    output_line.push('|');

    let rep_freq = append_rep_freq(rt_cd, benchmark, rec[0], log);
    output_line.push_str(&rep_freq[..]);
    output_line.push('|');

    //maturity date missing
    let rate_flag: &str;
    if rt_cd.contains_key(benchmark) {
        rate_flag = rt_cd.get(benchmark).unwrap().rate_flag.as_str();
    } else {
        rate_flag = "FIXED";
    }
    let mut next_rep_date: NaiveDate = NaiveDate::from_ymd(2099, 12, 31);
    if rate_flag.to_uppercase() != "FIXED" {
        next_rep_date =
            append_next_rep_dt(rt_cd, &rep_freq.to_uppercase(), benchmark, &dt, "V", log);
        if next_rep_date == NaiveDate::from_ymd(1900, 1, 1) {
            output_line.push_str("|");
        } else {
            output_line.push_str(&format!("{}", next_rep_date.format("%d-%m-%Y")));
            output_line.push_str("|");
        }
    } else {
        output_line.push_str(&format!("{}", next_rep_date.format("%d-%m-%Y")));
        output_line.push_str("|");
    }
    let mut rep_date = NaiveDate::from_ymd(1970, 01, 01);
    if override_sys_reset_dt == "Y" {
        rep_date = append_last_rep_date(
            &mut output_line,
            &rep_freq.to_uppercase(),
            next_rep_date,
            val_date,
        );
    } else {
        rep_date = val_date;
    }
    output_line.push_str(&format!("{}", rep_date.format("%d-%m-%Y")));
    output_line.push_str("|");
    output_line.push_str("||");
    output_line.push_str(bm_id_lookup);
    output_line.push('|');
    output_line.push_str(alm_concat);
    output_line.push('|');
    output_line.push_str(&mis2_code);
    output_line.push_str("|");
    //Passthrough Derived Interest Rate
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let mut der_ir: f32 = 0.0;
    if !npa_map.contains_key(&npa_flg.to_string()) {
        der_ir = rec[19]
            .parse::<f32>()
            .expect("Could not parse derived int_rate");
    }
    output_line.push_str(&der_ir.to_string());
    output_line.push_str("|");
    //Passthrough Benchmark Rates
    let rep_date = &format!("{}", rep_date.format("%d-%m-%Y"));
    let bnchmrk_date = date_parser.parse(&rep_date.to_string());
    let sprd_bnchmrk = bm_id_lookup.to_string();
    let mut bnchmrk_rate = 0.0;
    let mut prev_dt = date_from_timestamp(0);
    if sprd_map.contains_key(&sprd_bnchmrk) {
        for bnchmrk_val in sprd_map
            .get(&sprd_bnchmrk)
            .expect("Could not find spread benchmark")
        {
            for (key, val) in bnchmrk_val {
                if key <= &bnchmrk_date && prev_dt <= *key {
                    prev_dt = *key;
                    bnchmrk_rate = *val;
                }
            }
        }
    }
    output_line.push_str(&bnchmrk_rate.to_string());
    output_line.push_str("|");
    //Passthrough Spread
    output_line.push_str(&(&der_ir - &bnchmrk_rate).to_string());
    output_line.push_str("|");
    //Passthrough Fully Floating flag
    let mut ff_flag: String = String::new();
    if rate_flag.to_uppercase() == "FIXED" {
        ff_flag = "NA".to_string();
    } else {
        if rt_cd.contains_key(benchmark.to_string().to_uppercase().trim_matches('"')) {
            let ff_struct = rt_cd
                .get(benchmark.to_string().to_uppercase().trim_matches('"'))
                .unwrap();
            if ff_struct.days_added_to_bus_dt == "" && ff_struct.reset_month == "" {
                ff_flag = "YES".to_string();
            } else {
                ff_flag = "NO".to_string();
            }
        } else {
            ff_flag = "NA".to_string();
        }
    }
    output_line.push_str(&ff_flag);
    let default_od_study_val = vec![0.0000, 0.0000, 100.0000];
    let od_study_val = od_study_map.get(balm_llg).unwrap_or(&default_od_study_val);
    output_line.push_str("|");
    output_line.push_str(&od_study_val[0].to_string());
    output_line.push_str("|");
    output_line.push_str(&od_study_val[1].to_string());
    output_line.push_str("|");
    output_line.push_str(&od_study_val[2].to_string());
    output_line.push_str("|");
    output_line.push_str(rec[33]);
    output_line.push_str("|");
    output_line.push_str(rec[34]);
    output_line.push_str("|");
    output_line.push_str(rec[35]);
    output_line.push_str("|");
    output_line.push_str(rec[36]);
    output_line.push_str("|");
    output_line.push_str(rec[37]);
    output_line
}

fn get_alm_concat(ora_mis1: &str, ora_prod: &str, ora_gl: &str, ora_cat: &str) -> String {
    let mut alm_concat: String = String::new();
    alm_concat.push_str(ora_mis1);
    alm_concat.push('_');
    alm_concat.push_str(ora_prod);
    alm_concat.push('_');
    alm_concat.push_str(ora_gl);
    alm_concat.push('_');
    alm_concat.push_str(ora_cat);
    alm_concat
}

pub fn append_rep_freq(
    rt_cd: &HashMap<String, RateCodeMaster>,
    key: &str,
    acc_no: &str,
    log: &Logger,
) -> String {
    match rt_cd.get(key) {
        Some(val) => {
            if val.reset_freq == "" {
                "NONE".to_string()
            } else {
                val.reset_freq.to_string()
            }
        }
        None => {
            log_error!(
                log,
                "Rate_Code: `{}` for account: `{}` is not present in Rate Code Master File.",
                key,
                acc_no,
            );
            "NONE".to_string()
        }
    }
}

pub fn append_last_rep_date(
    output_line: &mut String,
    rep_freq: &str,
    next_rep_date: NaiveDate,
    val_date: NaiveDate,
) -> NaiveDate {
    //Converting to upper case removes the '-' sign. Thus using BI MONTHLY insted of BI-MONTHLY
    let mut last_rep_date: NaiveDate = match rep_freq {
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
        _ => val_date,
    };
    if last_rep_date < val_date {
        last_rep_date = val_date;
    }
    last_rep_date
}

pub fn append_next_rep_dt(
    rt_cd: &HashMap<String, RateCodeMaster>,
    rep_freq: &str,
    reprice_index: &str,
    as_on_date: &NaiveDate,
    rate_flag: &str,
    log: &Logger,
) -> NaiveDate {
    let next_rep_dt: NaiveDate;
    if rep_freq == "" {
        next_rep_dt = default_next_repricing_date(reprice_index, as_on_date);
    } else {
        if rate_flag == "V" {
            let rate_code = rt_cd.get(reprice_index);
            if let Some(rt_cd) = rate_code {
                let days_added_to_bus_dt: i64 = rt_cd.days_added_to_bus_dt.parse().unwrap_or(0);
                if days_added_to_bus_dt != 0 {
                    next_rep_dt = add_days(*as_on_date, days_added_to_bus_dt);
                } else {
                    if rep_freq == "" {
                        next_rep_dt = default_next_repricing_date(reprice_index, as_on_date);
                    } else {
                        let mut reset_month: u32 =
                            get_month_value(&rt_cd.reset_month[..].to_uppercase().as_str());
                        let as_on_month = as_on_date.month();
                        let reset_day: u32 = rt_cd.reset_day.parse().unwrap_or(7);
                        if reset_month == 0 && !&rate_code.unwrap().reset_month[..].is_empty() {
                            let mut def_reset_month: u32 = 13;
                            let month_vec: Vec<&str> =
                                rate_code.unwrap().reset_month[..].split('-').collect();
                            let mut new_month_vec: Vec<u32> = Vec::with_capacity(4);
                            for month in month_vec.iter() {
                                let month_value: u32 =
                                    get_month_value(month.to_uppercase().as_str());
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
                                        next_rep_dt =
                                            default_next_repricing_date(reprice_index, as_on_date);
                                    } else {
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
                                    log_error!(log, "`Taking default next reprising date : '{}' for reprising frequency : `{}` .",as_on_date, rep_freq);
                                    next_rep_dt =
                                        default_next_repricing_date(reprice_index, as_on_date);
                                }
                            }
                        }
                    }
                }
            } else {
                next_rep_dt = default_next_repricing_date(reprice_index, as_on_date);
            }
        } else {
            next_rep_dt = NaiveDate::from_ymd(1900, 1, 1);
        }
    }

    next_rep_dt
}

fn default_next_repricing_date(reprice_index: &str, as_on_date: &NaiveDate) -> NaiveDate {
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
        next_rep_date = *as_on_date;
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
