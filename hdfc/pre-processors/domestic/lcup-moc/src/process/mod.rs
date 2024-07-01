use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use calamine::Reader;
use calamine::{Xlsx,Xls};
use slog::Logger;
mod io;
use rbdate::DateParser;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use calamine::{open_workbook, open_workbook_auto};
use health_report::HealthReport; 
use crate::macros;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file());
    let mut master1_excel: Xlsx<_> = open_workbook(config_params.master1_file()).expect("Can not open Ora GL File");
    let mut master1_map: HashMap<String,String> = HashMap::new();
    if let Some(Ok(reader)) = master1_excel.worksheet_range(config_params.master1_sheet()) {
        for row in reader.rows().skip(1) {
            let source_gl = row[1].to_string();
            master1_map.insert(row[0].to_string(), source_gl);
        }
    }
    let mut master2_excel: Xlsx<_> = open_workbook(config_params.master2_file()).expect("Can not open Master LLG Updated File");
    let mut master2_map: HashMap<String,(String,String)> = HashMap::new();
    if let Some(Ok(reader)) = master2_excel.worksheet_range(config_params.master2_sheet()) {
        for row in reader.rows().skip(1) {
            let finnancial_mapping = row[3].to_string();
            let balm_l2 = row[9].to_string();
            let pair = (finnancial_mapping,balm_l2);
            master2_map.insert(row[2].to_string(), pair);
        }
    }
    write!(
        op_writer,
        "Natural Account|Description|Financial Mapping|DR Adj|CR Adj|NET|LLG|CCY\n",
    );
    let mut tot_acc_enc = 0;
    let mut acc_processed = 0;
    let mut total_bal = 0.0;
    let mut input_excel= open_workbook_auto(config_params.input_file()).expect("Can not open LCUPSKIP INDIA CE File");
    if let Some(Ok(reader)) = input_excel.worksheet_range(config_params.input_sheet()) {
        for row in reader.rows().skip(1) {
            tot_acc_enc += 1;
            let mut gl = row[0].to_string();
            let mut char_in_gl = gl.len();
            let mut natural_acc = "".to_string();
            let mut desc = "".to_string();
            let mut finn_mapping = "".to_string();
            let mut dr_adj = "".to_string();
            let mut cr_adj = "".to_string();
            let mut net = "".to_string();
            let mut llg = "".to_string();
            let mut ccy = "".to_string();
            if char_in_gl <= 9 {
                natural_acc = match master1_map.get(&gl.to_string()) {
                    Some(val) => val.to_string(),
                    None => {
                        log_debug!(logger, "Can not get Source GL in Ora GL file for gl type :- {}.", gl);
                        "".to_string()
                    }
                };
            } else {
                natural_acc = config_params.default_gl_code().to_string();
            }
            desc = match master2_map.get(&natural_acc.to_string()) {
                Some(val) => val.0.to_string(),
                None => {
                    log_debug!(logger, "Can not get GL in Master llg updated file for Natural Acc :- {}.", natural_acc);
                    "".to_string()
                }
            };
            finn_mapping = match master2_map.get(&natural_acc.to_string()) {
                Some(val) => val.0.to_string(),
                None => {
                    log_debug!(logger, "Can not get GL in Master llg updated file for Natural Acc :- {}.", natural_acc);
                    "".to_string()
                }
            };
            dr_adj = row[1].to_string();
            cr_adj = row[2].to_string();
            net = row[3].to_string();
            llg = match master2_map.get(&natural_acc.to_string()) {
                Some(val) => val.1.to_string(),
                None => {
                    log_debug!(logger, "Can not get GL in Master llg updated file for Natural Acc :- {}.", natural_acc);
                    "".to_string()
                }
            };
            ccy = row[4].to_string();
            write!(
                op_writer,
                "{}|{}|{}|{}|{}|{}|{}|{}\n",
                natural_acc,
                desc,
                finn_mapping,
                dr_adj,
                cr_adj,
                net,
                llg,
                ccy
            );
            acc_processed+=1;
        }
    }
    let health_report = HealthReport::new(
        tot_acc_enc,
        acc_processed,
        tot_acc_enc - acc_processed,
        total_bal,
        total_bal,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_params.output_file()); 
}
