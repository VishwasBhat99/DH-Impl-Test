use self::derive_fields::get_op_line;
use self::manual_handler::remove_comma;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod manual_handler;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, diag_log: &Logger, log: &Logger) {
    let start_read_timer = SystemTime::now();
    let input_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `Ora_PROD.xlsx`.");
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_prod.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> =
        open_workbook(config_param.ref_file_path_2()).expect("Unable to open `Ora_GL.xlsx`.");
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    let mut t_ora_cat: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_gl.insert(row[0].to_string(), row[1].to_string());
            t_ora_cat.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Unable to open `ALM_Line_Master.xlsx`.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
            ia_llg.insert(row[0].to_string(), row[7].to_string());
            balm_llg.insert(row[0].to_string(), row[9].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> =
        open_workbook(config_param.ref_file_path_4()).expect("Unable to open `MIS1_Desc.xlsx`.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }

    let mut npa_flg: Vec<String> = Vec::new();
    let ref_txt1 = match new_buf_rdr(config_param.ref_file_path_5()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_5(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ref_txt1.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref_file_path_5(),
                line_num + 1,
                error
            ),
        };
        npa_flg.push(line.to_string());
    }

    let mut int_idx_name: HashMap<String, String> = HashMap::new();
    let ref_txt2 = match new_buf_rdr(config_param.ref_file_path_6()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_6(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ref_txt2.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref_file_path_6(),
                line_num + 1,
                error
            ),
        };
        let words: Vec<&str> = line.split('|').collect();
        int_idx_name.insert(words[0].to_string(), words[1].to_string());
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration for read timer.");
    info!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let prod_cd = vec!["18", "19", "92", "93"];
    let mut tot_rec = 0;
    let mut skp_rec = 0;
    let mut tot_amt = 0.0;
    for (line_num, lines) in input_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num,
                error
            ),
        };
        tot_rec += 1;
        let mut fields: Vec<&str> = line.split("~#~").collect();
        if line_num == 0 && fields[0].parse::<i64>().is_err() {
            skp_rec += 1;
            continue;
        }
        if fields.len() == 26 && !prod_cd.contains(&fields[0]) {
            fields[0] = fields[0].trim();
            output_line.push_str(&get_op_line(
                &mut fields,
                &mut div,
                &mut alm_line,
                &mut ia_llg,
                &mut balm_llg,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_ora_cat,
                &mut npa_flg,
                &mut int_idx_name,
                &log,
            ));

            let ccy = match fields[3] {
                "1" => "INR",
                _ => "FCY",
            };

            let book_bal: f64 = remove_comma(&fields[4]).parse().unwrap_or(DEFAULT_FLOAT);
            tot_amt += book_bal;
            let gl = if book_bal >= 0.0 {
                &fields[15]
            } else {
                &fields[16]
            };

            let recon_key = ReconKey::new(ccy.to_string(), "UBSCASAOD".to_string(), gl.to_string());
            recon
                .entry(recon_key)
                .and_modify(|val| *val += book_bal)
                .or_insert(book_bal);
        } else {
            log_debug!(log, "Skipped record: `{:?}`.", fields);
            skp_rec += 1;
            continue;
        }
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    info!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match writer.write_all(output_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
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
            "INP016_BPRG_CASA_Report",
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
            error
        ),
    };
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing records and reconcilation file for UBS CASA OD.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File for UBS CASA OD, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(tot_rec, tot_rec - skp_rec, skp_rec, tot_amt, tot_amt, 0);
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());
}
