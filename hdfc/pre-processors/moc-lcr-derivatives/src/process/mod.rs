use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::open_workbook_auto;
use calamine::Reader;
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
    let mut tot_amt_outflow: f64 = 0.0;
    let mut tot_amt_inflow: f64 = 0.0;
    let mut tot_amt_outflow_usd: f64 = 0.0;
    let mut tot_amt_inflow_usd: f64 = 0.0;
    let mut roe: f64 = 0.0;
    let mut coll_mtm: f64 = 0.0;
    let mut mtm_threshold: f64 = 0.0;
    let mut ccil: f64 = 0.0;
    let mut tot_ref_dep: f64 = 0.0;

    //Reading exchange rate file
    let exchange_rate_reader = fs::read_to_string(config_params.exchange_rate_file())
        .expect("Could Not Read Exchange rate file");
    let mut usd_rate = 0.0;
    for (line_no, line) in exchange_rate_reader.lines().enumerate() {
        let exchange_rate_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let from_ccy = exchange_rate_vec[0].trim().to_string().to_uppercase();
        let rate = exchange_rate_vec[2]
            .trim()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        if from_ccy == "USD" {
            usd_rate = rate;
        }
    }

    let mut input_file = open_workbook_auto(config_params.input_file()).expect(&format!(
        "Could not open file at: {}",
        config_params.input_file()
    ));
    if let Some(Ok(reader)) = input_file.worksheet_range(config_params.input_sheet()) {
        let mut row_num = 0;
        for row in reader.rows() {
            if row_num == 2 {
                tot_amt_outflow = row[4].to_string().parse().unwrap_or(0.0);
            } else if row_num == 3 {
                tot_amt_inflow = row[4].to_string().parse().unwrap_or(0.0);
            } else if row_num == 6 {
                tot_amt_outflow_usd = row[4].to_string().parse().unwrap_or(0.0);
            } else if row_num == 7 {
                tot_amt_inflow_usd = row[4].to_string().parse().unwrap_or(0.0);
            }
            row_num += 1;
        }
    }

    let mut csa_file = open_workbook_auto(config_params.csa_file()).expect(&format!(
        "Could not open file at: {}",
        config_params.csa_file()
    ));
    if let Some(Ok(reader)) = csa_file.worksheet_range(config_params.csa_sheet()) {
        let mut row_num = 0;
        for row in reader.rows() {
            if row_num == 1 {
                roe = row[1].to_string().parse().unwrap_or(0.0);
            } else if row_num == 2 {
                coll_mtm = row[1].to_string().parse().unwrap_or(0.0);
            } else if row_num == 3 {
                mtm_threshold = row[1].to_string().parse().unwrap_or(0.0);
            } else if row_num == 4 {
                ccil = row[1].to_string().parse().unwrap_or(0.0);
            } else if row_num == 5 {
                tot_ref_dep = row[1].to_string().parse().unwrap_or(0.0);
            }
            if row_num >= 6 {
                break;
            }
            row_num += 1;
        }
    }
    let mut native_amt_usd = 0.0;
    let mut const_amt_usd = 0.0;
    let mut const_amt = 0.0;
    let llgid_vec = vec![11460001, 11700001, 11460017, 11460021];
    let mut native_amt = 0.0;
    for id in llgid_vec {
        if id == 11460001 {
            const_amt = tot_amt_inflow;
            native_amt = tot_amt_outflow - (tot_amt_outflow_usd * usd_rate);
            native_amt_usd = tot_amt_outflow_usd;
            const_amt_usd = native_amt_usd * usd_rate;
        } else if id == 11700001 {
            native_amt = tot_amt_inflow - (tot_amt_inflow_usd * usd_rate);
            const_amt = tot_amt_inflow;
            native_amt_usd = tot_amt_inflow_usd;
            const_amt_usd = native_amt_usd * usd_rate;
        } else if id == 11460017 {
            native_amt = roe + tot_ref_dep;
        } else if id == 11460021 {
            if (mtm_threshold + ccil) < 0.0 {
                native_amt = coll_mtm - (mtm_threshold + ccil);
            } else {
                native_amt = coll_mtm - 0.0;
            }
        }
        if id == 11460001 || id == 11700001 {
            write!(
                op_writer,
                "{}|{}|{}|{}|{}|{}\n",
                config_params.as_on_date().format("%d-%m-%Y"),
                config_params.country(),
                "USD",
                id,
                native_amt_usd,
                const_amt_usd
            );
        }
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
