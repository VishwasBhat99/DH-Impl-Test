use self::account::Account;
use self::account_appenders::account_appender::create_account_without_cashflows;
use self::account_appenders::account_appender_moc::create_account_without_cashflows_moc;
use self::account_writer::AccountWithoutCashflows;
use self::derive_fields::derive_fields::get_output_line;
use self::derive_fields::derive_moc_fields::get_moc_output_line;
use self::output_lines::OutputLines;
use self::total_balance::*;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::{prelude::*, BufReader};
use std::time::SystemTime;

mod account;
mod account_appenders;
mod account_writer;
mod derive_fields;
mod output_lines;
mod total_balance;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut writer = AccountWithoutCashflows::new(config_param.output_file_path(), log);
    let mut output_lines = OutputLines::new();

    let mut ref_map: HashMap<String, String> = HashMap::new();
    let mut ia_map: HashMap<String, String> = HashMap::new();
    let mut nsfr_map: HashMap<String, String> = HashMap::new();
    let mut nsfr_moc_map: HashMap<String, String> = HashMap::new();
    let mut cr_dr: Vec<i64> = Vec::new();
    let mut ia_cr_dr: Vec<i64> = Vec::new();
    let mut special_map: HashMap<String, Vec<i64>> = HashMap::new();
    let mut ia_special_map: HashMap<String, Vec<i64>> = HashMap::new();
    let mut spl_dr_cr: HashMap<String, Vec<i64>> = HashMap::new();
    let mut ia_spl_dr_cr: HashMap<String, Vec<i64>> = HashMap::new();
    let mut spl_net: HashMap<String, Vec<i64>> = HashMap::new();
    let mut ia_spl_net: HashMap<String, Vec<i64>> = HashMap::new();

    if config_param.ref_file_path_1().contains(".txt") {
        let alm_master_file = match new_buf_rdr(config_param.ref_file_path_1()) {
            Ok(file) => file,
            Err(error) => panic!("Unable to open alm_master file : {:?}", error),
        };

        for line in BufReader::new(alm_master_file).lines() {
            let record = match line {
                Ok(ln) => ln,
                Err(error) => {
                    panic!("Error while reading alm master file : {:?}", error);
                }
            };
            let fields: Vec<&str> = record.split("|").collect();
            if fields.len() < 16 {
                continue;
            }

            ref_map.insert(fields[0].to_string(), fields[9].to_string());
            ia_map.insert(fields[0].to_string(), fields[7].to_string());
            nsfr_map.insert(fields[0].to_string(), fields[15].to_string());
            nsfr_moc_map.insert(fields[2].to_string(), fields[15].to_string());
            let gl = fields[2].to_string().parse().unwrap_or(DEFAULT_INT);
            let fin_map = fields[3].to_string();
            let method = fields
                .get(13)
                .expect("CR/DR/NET field not found for BALM `cf_type` stamping.")
                .to_string()
                .to_uppercase();
            if method == "NET" {
                special_map
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }
            if method == "CRDR" || method == "DRCR" {
                cr_dr.push(gl);
            }
            if method == "DCRDR" || method == "DDRCR" {
                spl_dr_cr
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }
            if method == "DNET" {
                spl_net
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }

            let ia_method = fields
                .get(14)
                .expect("CR/DR/NET field not found for IA `cf_type` stamping.")
                .to_string()
                .to_uppercase();
            if ia_method == "NET" {
                ia_special_map
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }
            if ia_method == "CRDR" || ia_method == "DRCR" {
                ia_cr_dr.push(gl);
            }
            if ia_method == "DCRDR" || ia_method == "DDRCR" {
                ia_spl_dr_cr
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }
            if ia_method == "DNET" {
                ia_spl_net
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }
        }
    } else {
        let mut ref_excel1: Xlsx<_> = open_workbook(config_param.ref_file_path_1())
            .expect("Error while opening `ALM Master File`.");
        if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.alm_master_sheet_name()) {
            for row in reader.rows() {
                ref_map.insert(row[0].to_string(), row[9].to_string());
                ia_map.insert(row[0].to_string(), row[7].to_string());
                nsfr_map.insert(row[0].to_string(), row[15].to_string());
                nsfr_moc_map.insert(row[2].to_string(), row[15].to_string());
                let gl = row[2].to_string().parse().unwrap_or(DEFAULT_INT);
                let fin_map = row[3].to_string();
                let method = row
                    .get(13)
                    .expect("CR/DR/NET field not found for BALM `cf_type` stamping.")
                    .to_string()
                    .to_uppercase();
                if method == "NET" {
                    special_map
                        .entry(fin_map.to_string())
                        .and_modify(|val| val.push(gl))
                        .or_insert_with(|| vec![gl]);
                }
                if method == "CRDR" || method == "DRCR" {
                    cr_dr.push(gl);
                }
                if method == "DCRDR" || method == "DDRCR" {
                    spl_dr_cr
                        .entry(fin_map.to_string())
                        .and_modify(|val| val.push(gl))
                        .or_insert_with(|| vec![gl]);
                }
                if method == "DNET" {
                    spl_net
                        .entry(fin_map.to_string())
                        .and_modify(|val| val.push(gl))
                        .or_insert_with(|| vec![gl]);
                }

                let ia_method = row
                    .get(14)
                    .expect("CR/DR/NET field not found for IA `cf_type` stamping.")
                    .to_string()
                    .to_uppercase();
                if ia_method == "NET" {
                    ia_special_map
                        .entry(fin_map.to_string())
                        .and_modify(|val| val.push(gl))
                        .or_insert_with(|| vec![gl]);
                }
                if ia_method == "CRDR" || ia_method == "DRCR" {
                    ia_cr_dr.push(gl);
                }
                if ia_method == "DCRDR" || ia_method == "DDRCR" {
                    ia_spl_dr_cr
                        .entry(fin_map.to_string())
                        .and_modify(|val| val.push(gl))
                        .or_insert_with(|| vec![gl]);
                }
                if ia_method == "DNET" {
                    ia_spl_net
                        .entry(fin_map.to_string())
                        .and_modify(|val| val.push(gl))
                        .or_insert_with(|| vec![gl]);
                }
            }
        }
    }

    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    let mut gl_map: HashMap<i64, f64> = HashMap::new();
    let mut ia_gl_map: HashMap<i64, f64> = HashMap::new();
    let mut spl_gl_map: HashMap<i64, TotalBalance> = HashMap::new();
    let mut ia_spl_gl_map: HashMap<i64, IATotalBalance> = HashMap::new();
    for line in BufReader::new(input_file).lines().skip(1) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("~#~").collect();
        if fields.len() != 10 {
            continue;
        }
        let dr_bal: f64 = fields[7].parse().unwrap_or(DEFAULT_FLOAT);
        let cr_bal: f64 = fields[8].parse().unwrap_or(DEFAULT_FLOAT);
        let net_bal: f64 = fields[9].parse().unwrap_or(DEFAULT_FLOAT);
        let gl = fields[0].to_string().parse().unwrap_or(DEFAULT_INT);
        gl_map
            .entry(gl)
            .and_modify(|val| *val += net_bal)
            .or_insert(net_bal);

        let mut ttl_amt = TotalBalance::new();
        ttl_amt.add(dr_bal, cr_bal, net_bal);
        spl_gl_map
            .entry(gl)
            .and_modify(|val| val.add(dr_bal, cr_bal, net_bal))
            .or_insert(ttl_amt);

        ia_gl_map
            .entry(gl)
            .and_modify(|val| *val += net_bal)
            .or_insert(net_bal);

        let mut ia_ttl_amt = IATotalBalance::new();
        ia_ttl_amt.add(dr_bal, cr_bal, net_bal);
        ia_spl_gl_map
            .entry(gl)
            .and_modify(|val| val.add(dr_bal, cr_bal, net_bal))
            .or_insert(ia_ttl_amt);
    }

    let mut gl_moc: Xlsx<_> = open_workbook(config_param.gl_moc_entry_file())
        .expect("Error while opening `GL MOC Entry File`.");
    if let Some(Ok(reader)) = gl_moc.worksheet_range(config_param.gl_moc_sheet_name()) {
        for row in reader.rows().skip(1) {
            let cr_bal: f64 = row[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            let dr_bal: f64 = row[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            let net_bal = dr_bal - cr_bal;
            let gl = row[0].to_string().parse().unwrap_or(DEFAULT_INT);
            gl_map
                .entry(gl)
                .and_modify(|val| *val += net_bal)
                .or_insert(net_bal);

            let mut ttl_amt = TotalBalance::new();
            ttl_amt.add(dr_bal, cr_bal, net_bal);
            spl_gl_map
                .entry(gl)
                .and_modify(|val| val.add(dr_bal, cr_bal, net_bal))
                .or_insert(ttl_amt);

            ia_gl_map
                .entry(gl)
                .and_modify(|val| *val += net_bal)
                .or_insert(net_bal);

            let mut ia_ttl_amt = IATotalBalance::new();
            ia_ttl_amt.add(dr_bal, cr_bal, net_bal);
            ia_spl_gl_map
                .entry(gl)
                .and_modify(|val| val.add(dr_bal, cr_bal, net_bal))
                .or_insert(ia_ttl_amt);
        }
    }
    let gl_ex_master_file = match new_buf_rdr(config_param.gl_ex_master()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open `GL Exclude Master` file : {:?}", error),
    };

    let mut ex_concat: Vec<String> = Vec::new();
    for line in BufReader::new(gl_ex_master_file).lines() {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading `GL Exclude Master` file : {:?}", error);
            }
        };
        ex_concat.push(record);
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let mut cf_type: HashMap<i64, f64> = HashMap::new();
    let mut bal: f64;
    let mut gl_vec: Vec<i64> = Vec::new();
    for (_, mut gls) in special_map.drain() {
        bal = 0.0;
        gls.sort();
        gls.dedup();
        for gl in gls.iter() {
            bal += gl_map.get(gl).unwrap_or(&DEFAULT_FLOAT);
            gl_vec.push(*gl);
        }
        for gl in gl_vec.iter() {
            cf_type.insert(*gl, bal);
            log_debug!(log, "gl: `{}`, bal: `{}`", gl, bal);
        }
        gl_vec.clear();
    }

    let mut ia_cf_type: HashMap<i64, f64> = HashMap::new();
    let mut ia_gl_vec: Vec<i64> = Vec::new();
    for (_, mut gls) in ia_special_map.drain() {
        bal = 0.0;
        gls.sort();
        gls.dedup();
        for gl in gls.iter() {
            bal += ia_gl_map.get(gl).unwrap_or(&DEFAULT_FLOAT);
            ia_gl_vec.push(*gl);
        }
        for gl in ia_gl_vec.iter() {
            ia_cf_type.insert(*gl, bal);
            log_debug!(log, "gl: `{}`, bal: `{}`", gl, bal);
        }
        ia_gl_vec.clear();
    }

    let mut d_dr_cr_cf_type: HashMap<i64, TotalBalance> = HashMap::new();
    let mut ia_d_dr_cr_cf_type: HashMap<i64, IATotalBalance> = HashMap::new();
    let mut is_covered: HashMap<String, bool> = HashMap::new();
    let mut is_ia_covered: HashMap<String, bool> = HashMap::new();
    let mut dr_bal: f64;
    let mut cr_bal: f64;
    let mut net_bal: f64;
    for (fin_map, gls) in spl_dr_cr.iter_mut() {
        dr_bal = DEFAULT_FLOAT;
        cr_bal = DEFAULT_FLOAT;
        net_bal = DEFAULT_FLOAT;
        gls.sort();
        gls.dedup();
        for gl in gls.iter() {
            let defaut_ttl_bal = TotalBalance::new();
            let ttl_bal = spl_gl_map.get(gl).unwrap_or(&defaut_ttl_bal);
            dr_bal += ttl_bal.ttl_dr;
            net_bal += ttl_bal.ttl_net;
            cr_bal += ttl_bal.ttl_cr;
            gl_vec.push(*gl);
        }
        is_covered.insert(fin_map.to_string(), false);
        for gl in gl_vec.iter() {
            let mut ttl_bal = TotalBalance::new();
            d_dr_cr_cf_type.insert(*gl, ttl_bal.get_sum(dr_bal, cr_bal, net_bal));
        }
        gl_vec.clear();
    }

    for (fin_map, gls) in ia_spl_dr_cr.iter_mut() {
        dr_bal = DEFAULT_FLOAT;
        cr_bal = DEFAULT_FLOAT;
        net_bal = DEFAULT_FLOAT;
        gls.sort();
        gls.dedup();
        for gl in gls.iter() {
            let defaut_ttl_bal = IATotalBalance::new();
            let ttl_bal = ia_spl_gl_map.get(gl).unwrap_or(&defaut_ttl_bal);
            dr_bal += ttl_bal.ttl_dr;
            net_bal += ttl_bal.ttl_net;
            cr_bal += ttl_bal.ttl_cr;
            ia_gl_vec.push(*gl);
        }
        is_ia_covered.insert(fin_map.to_string(), false);
        for gl in ia_gl_vec.iter() {
            let mut ttl_bal = IATotalBalance::new();
            ia_d_dr_cr_cf_type.insert(*gl, ttl_bal.get_sum(dr_bal, cr_bal, net_bal));
        }
        ia_gl_vec.clear();
    }

    let mut d_net_cf_type: HashMap<i64, TotalBalance> = HashMap::new();
    for (fin_map, gls) in spl_net.iter_mut() {
        dr_bal = DEFAULT_FLOAT;
        cr_bal = DEFAULT_FLOAT;
        net_bal = DEFAULT_FLOAT;
        gls.sort();
        gls.dedup();
        for gl in gls.iter() {
            let defaut_ttl_bal = TotalBalance::new();
            let ttl_bal = spl_gl_map.get(gl).unwrap_or(&defaut_ttl_bal);
            dr_bal += ttl_bal.ttl_dr;
            net_bal += ttl_bal.ttl_net;
            cr_bal += ttl_bal.ttl_cr;
            gl_vec.push(*gl);
        }
        is_covered.insert(fin_map.to_string(), false);
        for gl in gl_vec.iter() {
            let mut ttl_bal = TotalBalance::new();
            d_net_cf_type.insert(*gl, ttl_bal.get_sum(dr_bal, cr_bal, net_bal));
        }
        gl_vec.clear();
    }

    let mut ia_d_net_cf_type: HashMap<i64, IATotalBalance> = HashMap::new();
    for (fin_map, gls) in ia_spl_net.iter_mut() {
        dr_bal = DEFAULT_FLOAT;
        cr_bal = DEFAULT_FLOAT;
        net_bal = DEFAULT_FLOAT;
        gls.sort();
        gls.dedup();
        for gl in gls.iter() {
            let defaut_ttl_bal = IATotalBalance::new();
            let ttl_bal = ia_spl_gl_map.get(gl).unwrap_or(&defaut_ttl_bal);
            dr_bal += ttl_bal.ttl_dr;
            net_bal += ttl_bal.ttl_net;
            cr_bal += ttl_bal.ttl_cr;
            ia_gl_vec.push(*gl);
        }
        is_ia_covered.insert(fin_map.to_string(), false);
        for gl in ia_gl_vec.iter() {
            let mut ttl_bal = IATotalBalance::new();
            ia_d_net_cf_type.insert(*gl, ttl_bal.get_sum(dr_bal, cr_bal, net_bal));
        }
        ia_gl_vec.clear();
    }
    let start_process_timer = SystemTime::now();
    //create temp concat for deriving alm line
    let mut temp_concat: String = String::with_capacity(15);
    let mut total_positive_bal = DEFAULT_FLOAT;
    let mut total_negative_bal = DEFAULT_FLOAT;
    let mut output_acc_info: String = String::new();
    let mut ttl_acc_encntrd: i64 = DEFAULT_INT;
    let mut skp_acc: i64 = DEFAULT_INT;
    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };

    let mut output_file = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` : {}.",
            config_param.output_file_path(),
            error
        ),
    };

    for line in BufReader::new(input_file).lines().skip(1) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("~#~").collect();
        ttl_acc_encntrd += 1;
        if fields.len() != 10 {
            skp_acc += 1;
            continue;
        }
        if fields[4].to_string() == "\u{0}".to_string() {
            temp_concat.push_str("");
        } else {
            temp_concat.push_str(fields[4]);
        }
        temp_concat.push_str("_");
        if fields[2].to_string() == "\u{0}".to_string() {
            temp_concat.push_str("");
        } else {
            temp_concat.push_str(fields[2]);
        }
        temp_concat.push_str("_");
        if fields[0].to_string() == "\u{0}".to_string() {
            temp_concat.push_str("");
        } else {
            temp_concat.push_str(fields[0]);
        }
        temp_concat.push_str("_");
        if fields[3].to_string() == "\u{0}".to_string() {
            temp_concat.push_str("");
        } else {
            temp_concat.push_str(fields[3]);
        }
        let alm_line = ref_map
            .entry(temp_concat.to_string())
            .or_insert_with(|| "NONE".to_string());
        let ia_line = ia_map
            .entry(temp_concat.to_string())
            .or_insert_with(|| "NONE".to_string());
        let nsfr = nsfr_map
            .entry(temp_concat.to_string())
            .or_insert_with(|| "".to_string());
        log_debug!(
            diag_log,
            "Concat for Account no: {} is {}",
            fields[1],
            temp_concat
        );
        log_debug!(
            diag_log,
            "Alm line for Account no: {} is {}",
            fields[1],
            alm_line
        );
        log_debug!(
            diag_log,
            "IA line for Account no: {} is {}",
            fields[1],
            ia_line
        );
        let bal_total = fields[9].parse().unwrap_or(DEFAULT_FLOAT);
        if bal_total < 0.0 {
            total_negative_bal += bal_total;
        } else {
            total_positive_bal += bal_total;
        }

        let mut cr_bal: f64 = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
        let mut dr_bal: f64 = fields[7].to_string().parse().unwrap_or(DEFAULT_FLOAT);
        let mut c_typ: &str;
        let is_acc_gl: &str = if ex_concat.contains(&fields[0].to_string()) {
            "Y"
        } else {
            "N"
        };
        let gl = fields[0].parse().unwrap_or(DEFAULT_INT);
        let mut accounts: Vec<Account> = Vec::new();
        let ccy: &str = fields[5].trim_matches('"');
        let mut financial_mapping = String::new();
        if cf_type.contains_key(&gl) {
            let amt = cf_type.get(&gl).unwrap_or(&DEFAULT_FLOAT);
            c_typ = if *amt < 0.0 { "L" } else { "A" };
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            );
        } else if d_net_cf_type.contains_key(&gl) {
            for (fin_map, gls) in spl_net.iter() {
                if gls.contains(&gl) {
                    financial_mapping = fin_map.to_string();
                }
            }
            if !*is_covered.get(&financial_mapping).unwrap_or(&true) {
                is_covered.insert(financial_mapping.to_string(), true);
                let def_val = TotalBalance::new();
                let ttl_bal = d_net_cf_type.get(&gl).unwrap_or(&def_val);
                c_typ = if ttl_bal.ttl_net < 0.0 { "L" } else { "A" };
                accounts.push(create_account_without_cashflows(
                    &fields,
                    ttl_bal.ttl_dr,
                    ttl_bal.ttl_cr,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                    ia_line,
                    nsfr,
                ));
                get_output_line(
                    &fields,
                    &mut output_lines,
                    ttl_bal.ttl_dr,
                    ttl_bal.ttl_cr,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                    ia_line,
                    nsfr,
                )
            }

            cr_bal = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            dr_bal = fields[7].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            c_typ = "N";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            )
        } else if d_dr_cr_cf_type.contains_key(&gl) {
            for (fin_map, gls) in spl_dr_cr.iter() {
                if gls.contains(&gl) {
                    financial_mapping = fin_map.to_string();
                }
            }
            if !*is_covered.get(&financial_mapping).unwrap_or(&true) {
                is_covered.insert(financial_mapping.to_string(), true);
                let def_val = TotalBalance::new();
                let ttl_bal = d_dr_cr_cf_type.get(&gl).unwrap_or(&def_val);
                let mut d_dr_bal = ttl_bal.ttl_dr;
                let mut d_cr_bal = DEFAULT_FLOAT;
                c_typ = "D";
                accounts.push(create_account_without_cashflows(
                    &fields,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                ));
                get_output_line(
                    &fields,
                    &mut output_lines,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                );
                d_dr_bal = DEFAULT_FLOAT;
                d_cr_bal = ttl_bal.ttl_cr;
                c_typ = "C";
                accounts.push(create_account_without_cashflows(
                    &fields,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                ));
                get_output_line(
                    &fields,
                    &mut output_lines,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                )
            }
            cr_bal = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            dr_bal = fields[7].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            c_typ = "N";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            );
        } else if cr_dr.contains(&gl) {
            cr_bal = DEFAULT_FLOAT;
            c_typ = "D";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            );
            dr_bal = DEFAULT_FLOAT;
            cr_bal = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            c_typ = "C";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            )
        } else {
            c_typ = "N";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            );
        }

        write!(output_file, "{}", output_lines.processed_lines)
            .expect("Error while writing output line.");

        for acc in accounts.iter() {
            writer.write(acc.clone());
        }
        temp_concat.clear();
        accounts.clear();
        output_lines.processed_lines.clear();
    }

    if let Some(Ok(reader)) = gl_moc.worksheet_range(config_param.gl_moc_sheet_name()) {
        for fields in reader.rows().skip(1) {
            ttl_acc_encntrd += 1;

            let bal_total = fields[5].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            if bal_total < 0.0 {
                total_negative_bal += bal_total;
            } else {
                total_positive_bal += bal_total;
            }

            let mut cr_bal: f64 = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            let mut dr_bal: f64 = fields[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            let mut c_typ: &str;
            let is_acc_gl: &str = if ex_concat.contains(&fields[0].to_string()) {
                "Y"
            } else {
                "N"
            };
            let mut financial_mapping = String::new();
            let gl = fields[0].to_string().parse().unwrap_or(DEFAULT_INT);
            let mut accounts: Vec<Account> = Vec::new();
            let gl_moc_ccy = if let Some(ccy) = fields.get(7) {
                let moc_ccy = ccy.to_string();
                if moc_ccy.is_empty() {
                    String::from(config_param.gl_moc_ccy())
                } else {
                    moc_ccy
                }
            } else {
                String::from(config_param.gl_moc_ccy())
            };
            let moc_ia_line = fields
                .get(8)
                .unwrap_or(&calamine::DataType::String(String::from("NONE")))
                .to_string();
            let nsfr = nsfr_moc_map
                .entry(gl.to_string())
                .or_insert_with(|| "NONE".to_string());
            if cf_type.contains_key(&gl) {
                let amt = cf_type.get(&gl).unwrap_or(&DEFAULT_FLOAT);
                c_typ = if *amt < 0.0 { "L" } else { "A" };
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else if d_net_cf_type.contains_key(&gl) {
                for (fin_map, gls) in spl_dr_cr.iter() {
                    if gls.contains(&gl) {
                        financial_mapping = fin_map.to_string();
                    }
                }
                if !*is_covered.get(&financial_mapping).unwrap_or(&true) {
                    is_covered.insert(financial_mapping.to_string(), true);
                    let def_val = TotalBalance::new();
                    let ttl_bal = d_net_cf_type.get(&gl).unwrap_or(&def_val);
                    c_typ = if ttl_bal.ttl_net < 0.0 { "L" } else { "A" };
                    accounts.push(create_account_without_cashflows_moc(
                        &fields,
                        ttl_bal.ttl_dr,
                        ttl_bal.ttl_cr,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                        &moc_ia_line,
                        nsfr,
                    ));
                    output_acc_info.push_str(
                        get_moc_output_line(
                            &fields,
                            ttl_bal.ttl_dr,
                            ttl_bal.ttl_cr,
                            &gl_moc_ccy,
                            c_typ,
                            is_acc_gl,
                            ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                            &moc_ia_line,
                            nsfr,
                        )
                        .as_str(),
                    );
                }
                cr_bal = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                dr_bal = fields[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                c_typ = "N";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    DEFAULT_FLOAT,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        DEFAULT_FLOAT,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else if d_dr_cr_cf_type.contains_key(&gl) {
                for (fin_map, gls) in spl_dr_cr.iter() {
                    if gls.contains(&gl) {
                        financial_mapping = fin_map.to_string();
                    }
                }
                if !*is_covered.get(&financial_mapping).unwrap_or(&true) {
                    is_covered.insert(financial_mapping.to_string(), true);
                    let def_val = TotalBalance::new();
                    let ttl_bal = d_dr_cr_cf_type.get(&gl).unwrap_or(&def_val);
                    let mut d_dr_bal = ttl_bal.ttl_dr;
                    let mut d_cr_bal = DEFAULT_FLOAT;
                    c_typ = "D";
                    accounts.push(create_account_without_cashflows_moc(
                        &fields,
                        d_dr_bal,
                        d_cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        d_dr_bal - d_cr_bal,
                        &moc_ia_line,
                        nsfr,
                    ));
                    output_acc_info.push_str(
                        get_moc_output_line(
                            &fields,
                            d_dr_bal,
                            d_cr_bal,
                            &gl_moc_ccy,
                            c_typ,
                            is_acc_gl,
                            d_dr_bal - d_cr_bal,
                            &moc_ia_line,
                            nsfr,
                        )
                        .as_str(),
                    );
                    d_dr_bal = DEFAULT_FLOAT;
                    d_cr_bal = ttl_bal.ttl_cr;
                    c_typ = "C";
                    accounts.push(create_account_without_cashflows_moc(
                        &fields,
                        d_dr_bal,
                        d_cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        d_dr_bal - d_cr_bal,
                        &moc_ia_line,
                        nsfr,
                    ));
                    output_acc_info.push_str(
                        get_moc_output_line(
                            &fields,
                            d_dr_bal,
                            d_cr_bal,
                            &gl_moc_ccy,
                            c_typ,
                            is_acc_gl,
                            d_dr_bal - d_cr_bal,
                            &moc_ia_line,
                            nsfr,
                        )
                        .as_str(),
                    );
                }
                cr_bal = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                dr_bal = fields[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                c_typ = "N";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    DEFAULT_FLOAT,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        DEFAULT_FLOAT,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else if cr_dr.contains(&gl) {
                cr_bal = DEFAULT_FLOAT;
                c_typ = "D";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
                dr_bal = DEFAULT_FLOAT;
                cr_bal = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                c_typ = "C";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else {
                c_typ = "N";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            }

            write!(output_file, "{}", output_acc_info).expect("Error while writing output line.");

            for acc in accounts.iter() {
                writer.write(acc.clone());
            }
            accounts.clear();
            output_acc_info.clear();
        }
    }
    writer.close();

    let mut concat_lines = String::new();
    let mut concat_writer = match buf_file_wrtr(config_param.concat_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create concat file: `{}` on location `{}` : {}",
            config_param.concat_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    output_lines.concat_lines.sort();
    output_lines.concat_lines.dedup();
    for concat in output_lines.concat_lines.drain(..) {
        concat_lines.push_str(&concat);
        concat_lines.push('\n');
    }
    match concat_writer.write_all(concat_lines.as_bytes()) {
        Ok(_) => println!("Successfully written concats for missing alm lines."),
        Err(error) => panic!(
            "Unable to write concat lines to the file `{}`: {}.",
            config_param.concat_file_path(),
            error,
        ),
    }

    output_lines.clear();
    let input_file = match new_buf_rdr(config_param.input_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open input file : {:?}", error),
    };
    let ia_out_path = config_param.output_file_path().replace(".txt", "") + "IA";
    let mut ia_output_file = match buf_file_wrtr(&ia_out_path, None) {
        Ok(output_file) => output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` : {}.",
            ia_out_path, error
        ),
    };
    let mut writer = AccountWithoutCashflows::new(&ia_out_path, log);

    for line in BufReader::new(input_file).lines().skip(1) {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("error while reading input file : {:?}", error);
            }
        };
        let fields: Vec<&str> = record.split("~#~").collect();
        if fields.len() != 10 {
            skp_acc += 1;
            continue;
        }

        temp_concat.push_str(fields[4]);
        temp_concat.push_str("_");
        temp_concat.push_str(fields[2]);
        temp_concat.push_str("_");
        temp_concat.push_str(fields[0]);
        temp_concat.push_str("_");
        temp_concat.push_str(fields[3]);

        let alm_line = ref_map
            .entry(temp_concat.to_string())
            .or_insert_with(|| "NONE".to_string());
        let ia_line = ia_map
            .entry(temp_concat.to_string())
            .or_insert_with(|| "NONE".to_string());

        let nsfr = nsfr_map
            .entry(temp_concat.to_string())
            .or_insert_with(|| "NONE".to_string());
        let mut cr_bal: f64 = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
        let mut dr_bal: f64 = fields[7].to_string().parse().unwrap_or(DEFAULT_FLOAT);
        let mut c_typ: &str;
        let is_acc_gl: &str = if ex_concat.contains(&fields[0].to_string()) {
            "Y"
        } else {
            "N"
        };
        let gl = fields[0].parse().unwrap_or(DEFAULT_INT);
        let mut accounts: Vec<Account> = Vec::new();
        let ccy: &str = fields[5].trim_matches('"');
        let mut financial_mapping = String::new();
        if ia_cf_type.contains_key(&gl) {
            let amt = ia_cf_type.get(&gl).unwrap_or(&DEFAULT_FLOAT);
            c_typ = if *amt < 0.0 { "L" } else { "A" };
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            );
        } else if ia_d_net_cf_type.contains_key(&gl) {
            for (fin_map, gls) in ia_spl_net.iter() {
                if gls.contains(&gl) {
                    financial_mapping = fin_map.to_string();
                }
            }
            if !*is_ia_covered.get(&financial_mapping).unwrap_or(&true) {
                is_ia_covered.insert(financial_mapping.to_string(), true);
                let def_val = IATotalBalance::new();
                let ttl_bal = ia_d_net_cf_type.get(&gl).unwrap_or(&def_val);
                c_typ = if ttl_bal.ttl_net < 0.0 { "L" } else { "A" };
                accounts.push(create_account_without_cashflows(
                    &fields,
                    ttl_bal.ttl_dr,
                    ttl_bal.ttl_cr,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                    ia_line,
                    nsfr,
                ));
                get_output_line(
                    &fields,
                    &mut output_lines,
                    ttl_bal.ttl_dr,
                    ttl_bal.ttl_cr,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                    ia_line,
                    nsfr,
                )
            }

            cr_bal = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            dr_bal = fields[7].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            c_typ = "N";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            )
        } else if ia_d_dr_cr_cf_type.contains_key(&gl) {
            for (fin_map, gls) in ia_spl_dr_cr.iter() {
                if gls.contains(&gl) {
                    financial_mapping = fin_map.to_string();
                }
            }
            if !*is_ia_covered.get(&financial_mapping).unwrap_or(&true) {
                is_ia_covered.insert(financial_mapping.to_string(), true);
                let def_val = IATotalBalance::new();
                let ttl_bal = ia_d_dr_cr_cf_type.get(&gl).unwrap_or(&def_val);
                let mut d_dr_bal = ttl_bal.ttl_dr;
                let mut d_cr_bal = DEFAULT_FLOAT;
                c_typ = "D";
                accounts.push(create_account_without_cashflows(
                    &fields,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                ));
                get_output_line(
                    &fields,
                    &mut output_lines,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                );
                d_dr_bal = DEFAULT_FLOAT;
                d_cr_bal = ttl_bal.ttl_cr;
                c_typ = "C";
                accounts.push(create_account_without_cashflows(
                    &fields,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                ));
                get_output_line(
                    &fields,
                    &mut output_lines,
                    d_dr_bal,
                    d_cr_bal,
                    &temp_concat,
                    alm_line,
                    c_typ,
                    is_acc_gl,
                    config_param.currency(),
                    d_dr_bal - d_cr_bal,
                    ia_line,
                    nsfr,
                )
            }
            cr_bal = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            dr_bal = fields[7].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            c_typ = "N";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                DEFAULT_FLOAT,
                ia_line,
                nsfr,
            );
        } else if ia_cr_dr.contains(&gl) {
            cr_bal = DEFAULT_FLOAT;
            c_typ = "D";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            );
            dr_bal = DEFAULT_FLOAT;
            cr_bal = fields[8].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            c_typ = "C";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            )
        } else {
            c_typ = "N";
            accounts.push(create_account_without_cashflows(
                &fields,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            ));
            get_output_line(
                &fields,
                &mut output_lines,
                dr_bal,
                cr_bal,
                &temp_concat,
                alm_line,
                c_typ,
                is_acc_gl,
                ccy,
                dr_bal - cr_bal,
                ia_line,
                nsfr,
            );
        }

        write!(ia_output_file, "{}", output_lines.processed_lines)
            .expect("Error while writing output line.");

        for acc in accounts.iter() {
            writer.write(acc.clone());
        }
        temp_concat.clear();
        accounts.clear();
        output_lines.processed_lines.clear();
    }

    output_acc_info.clear();
    if let Some(Ok(reader)) = gl_moc.worksheet_range(config_param.gl_moc_sheet_name()) {
        for fields in reader.rows().skip(1) {
            let mut cr_bal: f64 = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            let mut dr_bal: f64 = fields[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
            let mut c_typ: &str;
            let is_acc_gl: &str = if ex_concat.contains(&fields[0].to_string()) {
                "Y"
            } else {
                "N"
            };
            let mut financial_mapping = String::new();
            let gl = fields[0].to_string().parse().unwrap_or(DEFAULT_INT);
            let mut accounts: Vec<Account> = Vec::new();
            let gl_moc_ccy = if let Some(ccy) = fields.get(7) {
                let moc_ccy = ccy.to_string();
                if moc_ccy.is_empty() {
                    String::from(config_param.gl_moc_ccy())
                } else {
                    moc_ccy
                }
            } else {
                String::from(config_param.gl_moc_ccy())
            };
            let moc_ia_line = fields
                .get(8)
                .unwrap_or(&calamine::DataType::String(String::from("NONE")))
                .to_string();
            let nsfr = nsfr_moc_map
                .entry(gl.to_string())
                .or_insert_with(|| "NONE".to_string());
            if ia_cf_type.contains_key(&gl) {
                let amt = ia_cf_type.get(&gl).unwrap_or(&DEFAULT_FLOAT);
                c_typ = if *amt < 0.0 { "L" } else { "A" };
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else if ia_d_net_cf_type.contains_key(&gl) {
                for (fin_map, gls) in ia_spl_dr_cr.iter() {
                    if gls.contains(&gl) {
                        financial_mapping = fin_map.to_string();
                    }
                }
                if !*is_ia_covered.get(&financial_mapping).unwrap_or(&true) {
                    is_ia_covered.insert(financial_mapping.to_string(), true);
                    let def_val = IATotalBalance::new();
                    let ttl_bal = ia_d_net_cf_type.get(&gl).unwrap_or(&def_val);
                    c_typ = if ttl_bal.ttl_net < 0.0 { "L" } else { "A" };
                    accounts.push(create_account_without_cashflows_moc(
                        &fields,
                        ttl_bal.ttl_dr,
                        ttl_bal.ttl_cr,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                        &moc_ia_line,
                        nsfr,
                    ));
                    output_acc_info.push_str(
                        get_moc_output_line(
                            &fields,
                            ttl_bal.ttl_dr,
                            ttl_bal.ttl_cr,
                            &gl_moc_ccy,
                            c_typ,
                            is_acc_gl,
                            ttl_bal.ttl_dr - ttl_bal.ttl_cr,
                            &moc_ia_line,
                            nsfr,
                        )
                        .as_str(),
                    );
                }
                cr_bal = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                dr_bal = fields[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                c_typ = "N";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    DEFAULT_FLOAT,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        DEFAULT_FLOAT,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else if ia_d_dr_cr_cf_type.contains_key(&gl) {
                for (fin_map, gls) in ia_spl_dr_cr.iter() {
                    if gls.contains(&gl) {
                        financial_mapping = fin_map.to_string();
                    }
                }
                if !*is_ia_covered.get(&financial_mapping).unwrap_or(&true) {
                    is_ia_covered.insert(financial_mapping.to_string(), true);
                    let def_val = IATotalBalance::new();
                    let ttl_bal = ia_d_dr_cr_cf_type.get(&gl).unwrap_or(&def_val);
                    let mut d_dr_bal = ttl_bal.ttl_dr;
                    let mut d_cr_bal = DEFAULT_FLOAT;
                    c_typ = "D";
                    accounts.push(create_account_without_cashflows_moc(
                        &fields,
                        d_dr_bal,
                        d_cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        d_dr_bal - d_cr_bal,
                        &moc_ia_line,
                        nsfr,
                    ));
                    output_acc_info.push_str(
                        get_moc_output_line(
                            &fields,
                            d_dr_bal,
                            d_cr_bal,
                            &gl_moc_ccy,
                            c_typ,
                            is_acc_gl,
                            d_dr_bal - d_cr_bal,
                            &moc_ia_line,
                            nsfr,
                        )
                        .as_str(),
                    );
                    d_dr_bal = DEFAULT_FLOAT;
                    d_cr_bal = ttl_bal.ttl_cr;
                    c_typ = "C";
                    accounts.push(create_account_without_cashflows_moc(
                        &fields,
                        d_dr_bal,
                        d_cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        d_dr_bal - d_cr_bal,
                        &moc_ia_line,
                        nsfr,
                    ));
                    output_acc_info.push_str(
                        get_moc_output_line(
                            &fields,
                            d_dr_bal,
                            d_cr_bal,
                            &gl_moc_ccy,
                            c_typ,
                            is_acc_gl,
                            d_dr_bal - d_cr_bal,
                            &moc_ia_line,
                            nsfr,
                        )
                        .as_str(),
                    );
                }
                cr_bal = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                dr_bal = fields[3].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                c_typ = "N";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    DEFAULT_FLOAT,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        DEFAULT_FLOAT,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else if ia_cr_dr.contains(&gl) {
                cr_bal = DEFAULT_FLOAT;
                c_typ = "D";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
                dr_bal = DEFAULT_FLOAT;
                cr_bal = fields[4].to_string().parse().unwrap_or(DEFAULT_FLOAT);
                c_typ = "C";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            } else {
                c_typ = "N";
                accounts.push(create_account_without_cashflows_moc(
                    &fields,
                    dr_bal,
                    cr_bal,
                    &gl_moc_ccy,
                    c_typ,
                    is_acc_gl,
                    dr_bal - cr_bal,
                    &moc_ia_line,
                    nsfr,
                ));
                output_acc_info.push_str(
                    get_moc_output_line(
                        &fields,
                        dr_bal,
                        cr_bal,
                        &gl_moc_ccy,
                        c_typ,
                        is_acc_gl,
                        dr_bal - cr_bal,
                        &moc_ia_line,
                        nsfr,
                    )
                    .as_str(),
                );
            }

            write!(ia_output_file, "{}", output_acc_info)
                .expect("Error while writing output line.");

            for acc in accounts.iter() {
                writer.write(acc.clone());
            }
            accounts.clear();
            output_acc_info.clear();
        }
    }
    writer.close();
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for deriving fields and writing output.");
    debug!(
        diag_log,
        "Total Duration for deriving fields and writing output: {:?}.", duration
    );

    println!("Total Positive Balance: {:.2}", total_positive_bal);
    println!("Total Negative Balance: {:.2}", total_negative_bal);
    println!(
        "Total net Balance: {:.2}",
        total_negative_bal + total_positive_bal
    );

    let health_report = HealthReport::new(
        ttl_acc_encntrd,
        ttl_acc_encntrd - skp_acc,
        skp_acc,
        total_negative_bal + total_positive_bal,
        total_negative_bal + total_positive_bal,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
