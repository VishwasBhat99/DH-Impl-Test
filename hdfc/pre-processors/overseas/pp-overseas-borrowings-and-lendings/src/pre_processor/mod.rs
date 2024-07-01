use self::derive_fields::get_op_line;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `Ora_GL.xlsx`.");
    let mut t_ora_mis1: HashMap<String, String> = HashMap::new();
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_mis1.insert(row[2].to_string(), row[0].to_string());
            t_ora_prod.insert(row[4].to_string(), row[0].to_string());
            t_ora_gl.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `ALM_Line_Master.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[1].to_string(), row[6].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `ALM_COA_Master.xlsx` file.");
    let mut t_bdp_coa: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_bdp_coa.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut op_bor: String = String::new();
    let mut op_len: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_bor_acc: i32 = 0;
    let mut tot_len_acc: i32 = 0;
    let mut input_file: Xlsx<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    let mut ccy = String::new();
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for mut row in reader.rows() {
            if row[0].to_string().to_uppercase().trim() == "CURRENCY :" {
                ccy = row[2].to_string();
                continue;
            }
            let inst_name = row[2].to_string().to_uppercase();
            if (inst_name == "CSA BORROWING" || inst_name == "CSA LENDING")
                && config_param.gl_type() == "BH-Over-Borr-Lend"
            {
                continue;
            }
            if row[3].to_string().to_uppercase() == "BORROWING" {
                op_bor.push_str(&get_op_line(
                    &mut row,
                    &mut t_ora_mis1,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut ccy,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));

                let recon_key = ReconKey::new(
                    ccy.to_string(),
                    config_param.gl_type().to_string(),
                    "".to_string(), // TODO: missing gl code
                );
                let amt: f64 = row[11].to_string().parse().unwrap_or(0.0);
                recon
                    .entry(recon_key)
                    .and_modify(|val| *val += amt)
                    .or_insert(amt);

                tot_bor_acc += 1;
            } else if row[3].to_string().to_uppercase() == "LENDING" {
                op_len.push_str(&get_op_line(
                    &mut row,
                    &mut t_ora_mis1,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut ccy,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));

                let recon_key = ReconKey::new(
                    ccy.to_string(),
                    config_param.gl_type().to_string(),
                    "".to_string(), // TODO: missing gl code
                );
                let amt: f64 = remove_comma(row[11].to_string());
                recon
                    .entry(recon_key)
                    .and_modify(|val| *val += amt)
                    .or_insert(amt);

                tot_len_acc += 1;
            } else {
                log_warn!(
                    log,
                    "`Operation Type`: `{}` is not well-formatted for account: `{}`.",
                    row[3],
                    row[0]
                );
                continue;
            }
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut bor_writer = match buf_file_wrtr(config_param.output_file_path_borrowings(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create borrowings output file `{}` on location `{}` : {}",
            config_param.output_file_path_borrowings(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match bor_writer.write_all(op_bor.as_bytes()) {
        Ok(_) => println!("Successfully processed all borrowing accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.output_file_path_borrowings(),
            error
        ),
    }

    let mut len_writer = match buf_file_wrtr(config_param.output_file_path_lendings(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create lendings output file `{}` on location `{}` : {}",
            config_param.output_file_path_lendings(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match len_writer.write_all(op_len.as_bytes()) {
        Ok(_) => println!("Successfully processed all lending accounts."),
        Err(error) => panic!(
            "Unable to write processed lines on file `{}`: {}.",
            config_param.output_file_path_lendings(),
            error
        ),
    }
    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut recon_op_line = String::new();
    for (key, value) in recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            config_param.input_file_name(),
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push_str("\n");
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error,
        ),
    };
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Borrowings & Lendings Records and Reconcilation File, Total Duration: {:?}.",
        duration
    );

    let report_string = format!(
        "Total Accounts encountered: {}\n\
         Borrowings Accounts: {}\n\
         Lendings Accounts: {}",
        tot_bor_acc + tot_len_acc,
        tot_bor_acc,
        tot_len_acc,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}

pub fn remove_comma(amt: String) -> f64 {
    let mut val: String = amt.to_string();
    val.retain(|dig| dig != ',');
    val.parse().unwrap_or(0.0)
}
