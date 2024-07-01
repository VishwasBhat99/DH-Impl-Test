use self::derive_fields::*;
use self::io::*;
use self::structs::{alm_master::*, input_account::InputAccount};
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::new_buf_rdr;
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::default::Default;
use std::io::{prelude::*, BufReader};
use std::time::SystemTime;

mod derive_fields;
mod io;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let st_tm_read = SystemTime::now();
    let mut op_line: String = String::new();
    let mut tot_rec = DEFAULT_INT;
    let skp_rec = DEFAULT_INT;

    let mut alm_master: HashMap<AlmMasterKey, AlmMaster> = HashMap::new();
    let mut special_map: HashMap<String, Vec<i64>> = HashMap::new();
    let mut alm_master_excel: Xlsx<_> =
        open_workbook(config_param.alm_master()).expect("Unable to open Alm Master File.");
    if let Some(Ok(reader)) = alm_master_excel.worksheet_range(config_param.alm_master_sheet_name())
    {
        for row in reader.rows() {
            get_alm_master_data(row, &mut alm_master);
            let gl = row[0].to_string().parse().unwrap_or(DEFAULT_INT);
            let fin_map = row[5].to_string();
            let method = row
                .get(8)
                .expect("CR-DR field not found for BALM `cf_type` stamping.")
                .to_string()
                .to_uppercase();
            if method == "NET" {
                special_map
                    .entry(fin_map.to_string())
                    .and_modify(|val| val.push(gl))
                    .or_insert_with(|| vec![gl]);
            }
        }
    }

    let mut is_header: bool = config_param.is_header();
    let mut gl_map: HashMap<i64, f64> = HashMap::new();
    let mut input_reader = read_file(config_param.input_file_path());
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        if is_header {
            is_header = false;
            continue;
        }
        let net_bal: f64 = input_account.os_bal.parse().unwrap_or(DEFAULT_FLOAT);
        let gl = input_account.gl_cd.parse().unwrap_or(DEFAULT_INT);
        gl_map
            .entry(gl)
            .and_modify(|val| *val += net_bal)
            .or_insert(net_bal);
    }

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

    let gl_ex_master_file = match new_buf_rdr(config_param.gl_ex_master()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open `GL Exclude Master` file : {:?}", error),
    };

    let mut ex_concat: Vec<i64> = Vec::new();
    for line in BufReader::new(gl_ex_master_file).lines() {
        let record = match line {
            Ok(ln) => ln,
            Err(error) => {
                panic!("Error while reading `GL Exclude Master` file : {:?}", error);
            }
        };
        ex_concat.push(record.parse().unwrap_or(0));
    }

    let mut concats = String::new();
    let mut input_reader = read_file(config_param.input_file_path());
    let mut tot_amt = 0.0;
    is_header = config_param.is_header();
    for (line_num, lines) in input_reader.deserialize().enumerate() {
        let mut input_account: InputAccount =
            extract_lines(line_num, lines, config_param.input_file_path(), log);
        if is_header {
            is_header = false;
            continue;
        }
        tot_rec += 1;

        tot_amt += input_account.os_bal.parse().unwrap_or(DEFAULT_FLOAT);
        let gl = input_account.gl_cd.parse().unwrap_or(DEFAULT_INT);
        let c_typ = if cf_type.contains_key(&gl) {
            let amt = cf_type.get(&gl).unwrap_or(&DEFAULT_FLOAT);
            if *amt > 0.0 {
                "L"
            } else {
                "A"
            }
        } else {
            "N"
        };
        let is_acc_gl: &str = if ex_concat.contains(&gl) { "Y" } else { "N" };
        op_line.push_str(&get_op_line(
            &mut input_account,
            &mut alm_master,
            &mut concats,
            c_typ,
            is_acc_gl,
        ));
    }

    let ed_tm_read = SystemTime::now();
    let duration = ed_tm_read
        .duration_since(st_tm_read)
        .expect("Could not calculate total read process duration.");
    debug!(diag_log, "Read Process Total Duration: {:?}.", duration);

    let st_tm_writer = SystemTime::now();
    let mut op_writer = get_writer(config_param.output_file_path());
    output_writer(&mut op_writer, op_line, config_param.output_file_path());

    let mut concat_writer = get_writer(config_param.concat_file_path());
    output_writer(&mut concat_writer, concats, config_param.concat_file_path());

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let ed_tm_writer = SystemTime::now();
    let duration = ed_tm_writer
        .duration_since(st_tm_writer)
        .expect("Could not calculate total duration for write process.");
    debug!(diag_log, "Writing GL, Total Duration: {:?}.", duration);
}
