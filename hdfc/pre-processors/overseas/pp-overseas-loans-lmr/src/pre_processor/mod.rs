use self::derive_fields::get_op_line;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
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
    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_gl.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `ALM_Line_Master.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[1].to_string(), row[6].to_string());
        }
    }
    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `ALM_COA_Master.xlsx` file.");
    let mut t_bdp_coa: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_bdp_coa.insert(row[0].to_string(), row[5].to_string());
        }
    }
    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range("Sheet1") {
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
    let mut output_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut acc_pro_suc: i64 = 0;
    let mut outstanding_temp = 0.0;
    let mut prev_acc = "".to_string();
    let mut tot_amt: f64 = 0.0;
    let mut prev_fields: Vec<String> = Vec::new();
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
        tot_acc_encntrd += 1;

        let skip_rec = line.to_string();
        let mut fields: Vec<String> = line.split('|').map(|s| s.to_string()).collect();
        if line_num == 0 {
            if fields[0].parse::<i64>().is_err() {
                log_debug!(log, "Skipped record: `{}`.", skip_rec);
                continue;
            }
        }
        if fields.len() != 32 {
            log_debug!(log, "Skipped record: `{}`.", skip_rec);
            continue;
        }
        let curr_acc = fields[1].to_string();
        tot_amt += fields[14].parse::<f64>().unwrap_or(0.0);
        if curr_acc == prev_acc {
            if fields[15].to_uppercase() == "PRINCIPAL" {
                if outstanding_temp == 0.0 {
                    acc_pro_suc += 1;
                    continue;
                }
                if outstanding_temp < fields[18].parse::<f64>().unwrap_or(0.0) {
                    fields[18] = outstanding_temp.to_string();
                }
                output_line.push_str(&get_op_line(
                    &mut fields,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));
                outstanding_temp -= fields[18].parse::<f64>().unwrap_or(0.0);
            } else {
                output_line.push_str(&get_op_line(
                    &mut fields,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));
            }
        } else {
            if outstanding_temp > 0.0 {
                prev_fields[9] = prev_fields[8].clone();
                prev_fields[18] = outstanding_temp.to_string();
                prev_fields[15] = "PRINCIPAL".to_string();
                output_line.push_str(&get_op_line(
                    &mut prev_fields,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));
            }
            outstanding_temp = fields[14].parse::<f64>().unwrap_or(0.0);
            if fields[15].to_uppercase() == "PRINCIPAL" {
                if outstanding_temp == 0.0 {
                    acc_pro_suc += 1;
                    continue;
                }
                if outstanding_temp < fields[18].parse::<f64>().unwrap_or(0.0) {
                    fields[18] = outstanding_temp.to_string();
                }
                output_line.push_str(&get_op_line(
                    &mut fields,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));
                outstanding_temp -= fields[18].parse::<f64>().unwrap_or(0.0);
            } else {
                output_line.push_str(&get_op_line(
                    &mut fields,
                    &mut t_ora_prod,
                    &mut t_ora_gl,
                    &mut t_bdp_coa,
                    &mut div,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                ));
            }
            prev_fields = line.split('|').map(|s| s.to_string()).collect();
        }
        if prev_acc != curr_acc {
            let recon_key = ReconKey::new(
                fields[13].to_string(),
                config_param.gl_type().to_string(),
                fields[12].to_string(),
            );
            let amt: f64 = fields[14].to_string().parse().unwrap_or(0.0);
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);
        }
        acc_pro_suc += 1;
        prev_acc = curr_acc;
    }
    if outstanding_temp > 0.0 {
        prev_fields[9] = prev_fields[8].clone();
        prev_fields[18] = outstanding_temp.to_string();
        prev_fields[15] = "PRINCIPAL".to_string();
        output_line.push_str(&get_op_line(
            &mut prev_fields,
            &mut t_ora_prod,
            &mut t_ora_gl,
            &mut t_bdp_coa,
            &mut div,
            &mut alm_line,
            *config_param.as_on_date(),
            &log,
        ));
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
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
            error,
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
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );
    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
    );
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}
