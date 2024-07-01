use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use process::account::{AccData, Val};
use process::agg_key::AGGKey;
use process::tenor::*;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
mod account;
mod agg_key;
mod grp_key;
mod tenor;
pub fn process(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut tot_amt = 0.0;
    let tot_cfs: usize = 0;
    let mut summary_rows = 0;

    //Output file.
    let mut output_report_path = String::new();
    output_report_path.push_str(config_params.output_file_path());
    let mut output_writer = match buf_file_wrtr(&output_report_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create output file: `{}` at location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    let input_file = match File::open(config_params.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!(
            "unable to open {} Error: {}",
            config_params.input_file_path(),
            error
        ),
    };

    let reader = BufReader::new(input_file);
    let edw_alm_td_file = match new_buf_rdr(config_params.edw_alm_td_file()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to read EDW_ALM_TD_CLOSED_DDMMYYYY file: {}", error),
    };

    let mut gl_map: HashMap<String, String> = HashMap::new();
    let mut mis1_map: HashMap<String, String> = HashMap::new();
    let csv_reader = BufReader::new(edw_alm_td_file);
    for line in csv_reader.lines().skip(1) {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!(
                    "Cannot read line from EDW_ALM_TD_CLOSED_DDMMYYYY.csv file: {:?}",
                    error
                );
            }
        };
        let fields: Vec<&str> = acc_info.split("~#~").map(|s| s.trim()).collect();
        //Skip footer in reference file.
        if fields.len() > 1 {
            gl_map.insert(fields[0].to_string(), fields[28].to_string());
            mis1_map.insert(fields[0].to_string(), fields[37].to_string());
        }
    }
    let mut almconcat_map: HashMap<String, String> = HashMap::new();
    let mut ora_gl_excel = open_workbook_auto(config_params.ora_gl_master())
        .expect("Unable to open ORA GL Master file.");
    if let Some(Ok(reader)) = ora_gl_excel.worksheet_range(config_params.ora_sheet()) {
        for row in reader.rows().skip(1) {
            let mut alm_concat = String::new();
            alm_concat.push_str(&row[2].to_string());
            alm_concat.push('_');
            alm_concat.push_str(&row[4].to_string());
            alm_concat.push('_');
            alm_concat.push_str(&row[1].to_string());
            almconcat_map.insert(row[0].to_string().trim().to_string(), alm_concat);
        }
    }

    let mut div_map: HashMap<String, String> = HashMap::new();
    let mut mis_desc_excel = open_workbook_auto(config_params.mis_desc_file())
        .expect("Unable to open MIS DESC Master file.");
    if let Some(Ok(reader)) = mis_desc_excel.worksheet_range(config_params.mis_sheet()) {
        for row in reader.rows().skip(1) {
            div_map.insert(row[0].to_string().trim().to_string(), row[2].to_string());
        }
    }

    let mut alm_line_map: HashMap<String, String> = HashMap::new();
    let mut ia_line_map: HashMap<String, String> = HashMap::new();
    let mut master_llg_excel = open_workbook_auto(config_params.llg_master_file())
        .expect("Unable to open Master LLG Updated file.");
    if let Some(Ok(reader)) = master_llg_excel.worksheet_range(config_params.llg_sheet()) {
        for row in reader.rows().skip(1) {
            alm_line_map.insert(row[1].to_string().trim().to_string(), row[6].to_string());
            ia_line_map.insert(row[1].to_string().trim().to_string(), row[7].to_string());
        }
    }

    let mut org_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut pp_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut ia_tenor_map: HashMap<Tenor, String> = HashMap::new();
    let mut tenor_master_excel = open_workbook_auto(config_params.tenor_desc_file())
        .expect("Unable to open Tenor Master File.");
    if let Some(Ok(reader)) = tenor_master_excel.worksheet_range(config_params.tenor_sheet()) {
        for row in reader.rows().skip(1) {
            let org_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let pp_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            let ia_tenor = Tenor::new(row[0].to_string(), row[1].to_string());
            org_tenor_map.insert(org_tenor, row[3].to_string());
            pp_tenor_map.insert(pp_tenor, row[5].to_string());
            ia_tenor_map.insert(ia_tenor, row[2].to_string());
        }
    }

    let mut cat_map: HashMap<String, String> = HashMap::new();
    let mut cust_cat_excel = open_workbook_auto(config_params.cust_cat_master())
        .expect("Unable to open Cust Category Master file.");
    if let Some(Ok(reader)) = cust_cat_excel.worksheet_range(config_params.cust_sheet()) {
        for row in reader.rows().skip(1) {
            cat_map.insert(row[0].to_string().trim().to_string(), row[1].to_string());
        }
    }

    let lcr_master = match new_buf_rdr(config_params.lcr_master()) {
        Ok(file) => file,
        Err(error) => panic!("Unable to read TD_LCR_Classification.txt file: {}", error),
    };
    let mut lcr_map: HashMap<String, String> = HashMap::new();
    let lcr_reader = BufReader::new(lcr_master);
    for line in lcr_reader.lines().skip(1) {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from LCR Master file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').map(|s| s.trim()).collect();
        lcr_map.insert(fields[1].to_string(), fields[2].to_string());
    }

    let mut wd_nwd_map: HashMap<String, String> = HashMap::new();
    let mut wd_nwd_excel = open_workbook_auto(config_params.wd_nwd_master())
        .expect("Unable to open WD/NWD Master file.");
    if let Some(Ok(reader)) = wd_nwd_excel.worksheet_range(config_params.wd_nwd_sheet()) {
        for row in reader.rows().skip(1) {
            wd_nwd_map.insert(row[0].to_string().trim().to_string(), row[1].to_string());
        }
    }

    let _header_count = 0;
    let _is_header = false;

    let mut output_map: HashMap<AGGKey, Val> = HashMap::new();
    // In the txt input file We will always get one header line on the basis of metadata
    //so here skipping the only one line
    for line in reader.lines().skip(1) {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                log_error!(logger, "Failed to read line from input file: {:?}", error);
                continue;
            }
        };
        acc_enc += 1;
        let split_val: Vec<&str> = acc_info.split('|').map(|s| s.trim()).collect();

        let acc_data: AccData = grp_key::fetch_acc_data(
            &mut gl_map,
            &mut mis1_map,
            &mut almconcat_map,
            &mut div_map,
            &mut alm_line_map,
            &mut ia_line_map,
            &mut org_tenor_map,
            &mut pp_tenor_map,
            &mut ia_tenor_map,
            &mut cat_map,
            &mut lcr_map,
            &mut wd_nwd_map,
            split_val,
            &config_params,
            logger,
        );
        output_map
            .entry(acc_data.grp_key.clone())
            .and_modify(|data| data.append_data(acc_data.data.clone()))
            .or_insert(acc_data.data);
        acc_succ += 1;
    }
    for (key, data) in output_map.drain() {
        tot_amt += data.balance;
        let mut opstr: String = String::new();
        opstr.push_str(&key.to_string());
        opstr.push('|');
        opstr.push_str(&data.to_string());
        //Write to output report.
        write_data(&mut output_writer, opstr, logger);
        summary_rows += 1;
    }
    let timestamp = Local::now().naive_local().to_string();
    let as_on_date = config_params.as_on_date();
    let footer_summary = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        summary_rows
    );
    write_data(&mut output_writer, footer_summary, logger);

    let health_report = HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        tot_cfs as i64,
    );
    health_report.gen_health_rpt(&output_report_path);

    fn write_data(writer: &mut BufWriter<File>, op: String, logger: &Logger) {
        let output_as_bytes = op.as_bytes();
        match writer.write(output_as_bytes) {
            Ok(_val) => {}
            Err(err) => {
                log_info!(logger, "Error writing to output file. Error: {}", err);
            }
        }
    }
}
