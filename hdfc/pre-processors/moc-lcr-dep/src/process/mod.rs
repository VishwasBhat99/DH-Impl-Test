use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
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

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut op_writer = get_writer(&config_params.output_file());
    let sls_reader =
        fs::read_to_string(&config_params.sls_file()).expect("Could Not Read SLS File");
    let mut sls_total_hrc1009 = 0.0;
    let mut sls_total_hrc1099 = 0.0;
    for line in sls_reader.lines().skip(1) {
        let vec_sls = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        if vec_sls[1] == "HRC1009".to_string(){
            sls_total_hrc1009 = vec_sls[21].to_string().parse().unwrap_or(0.0);
        }
        if vec_sls[1] == "HRC1099".to_string(){
            sls_total_hrc1099 = vec_sls[21].to_string().parse().unwrap_or(0.0);
        }
    }

    let nwd_reader =
        fs::read_to_string(&config_params.nwd_file()).expect("Could Not Read NWD File");
    let mut nwd_amt = 0.0;
    for line in nwd_reader.lines().skip(1) {
        let vec_nwd = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        nwd_amt = vec_nwd[5].to_string().parse().unwrap_or(0.0);
    }
    let sls_nwd_amt_diff = sls_total_hrc1009 - sls_total_hrc1099 -nwd_amt;
    let tot_dep_bal_reader =
        fs::read_to_string(&config_params.tot_dep_bal_file()).expect("Could Not Read Total Deposit Balance File");
    let mut tot_constant_amt = 0.0;
    let mut cons_amt_71100019 = 0.0;
    let mut cons_amt_71100020 = 0.0;
    let mut cons_amt_71100021 = 0.0;
    let mut cons_amt_71100022 = 0.0;
    let mut cons_amt_71100023 = 0.0;
    let mut cons_amt_71100024 = 0.0;
    let mut cons_amt_71100025 = 0.0;
    let mut cons_amt_71100026 = 0.0;
    for line in tot_dep_bal_reader.lines() {
        let vec_dep_bal = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        tot_constant_amt = tot_constant_amt + vec_dep_bal[5].to_string().parse().unwrap_or(0.0);
        match vec_dep_bal[3] {
            "11340001" => cons_amt_71100019 = vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11340005" => cons_amt_71100020 = vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11380001" => cons_amt_71100021 = vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11380005" => cons_amt_71100022 = vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11380033" => cons_amt_71100023 = vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11380037" => cons_amt_71100024 = vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11380041" | "11380045" | "11380049" | "11380053" => cons_amt_71100025 = cons_amt_71100025 + vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            "11380009" | "11380013" | "11380017" | "11380021" | "11380025" |  "11380029" | "11380057" | "11380061" => cons_amt_71100026 = cons_amt_71100026 + vec_dep_bal[5].to_string().parse().unwrap_or(0.0),
            _ => ()
        }
    }
    if  (sls_nwd_amt_diff-tot_constant_amt) > config_params.amount().to_string().parse().unwrap_or(0.0){
        error!(
            logger,
            "Amount exceeded than 10000cr"
        );
        panic!("Amount exceeded than 10000cr");
    } else {
        let llgid_vec = vec![71100019,71100020,71100021,71100022,71100023,71100024,71100025,71100026];
        let mut native_amt = 0.0;
        for id in llgid_vec {
            native_amt = match id {
                71100019 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100019)/tot_constant_amt,
                71100020 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100020)/tot_constant_amt,
                71100021 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100021)/tot_constant_amt,
                71100022 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100022)/tot_constant_amt,
                71100023 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100023)/tot_constant_amt,
                71100024 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100024)/tot_constant_amt,
                71100025 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100025)/tot_constant_amt,
                71100026 => ((sls_nwd_amt_diff-tot_constant_amt)*cons_amt_71100026)/tot_constant_amt,
                _ => 0.0
            };
            write!(
                op_writer,
                "{}|{}|{}|{}|{}|{}\n",
                config_params.as_on_date().format("%d-%m-%Y"),
                config_params.country(),
                config_params.currency(),
                id,
                native_amt,
                native_amt
            );
        }
    }
}
