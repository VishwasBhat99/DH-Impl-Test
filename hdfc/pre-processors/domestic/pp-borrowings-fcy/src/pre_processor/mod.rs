use self::derive_fields::get_op_line;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xls, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod derive_fields;
mod output_lines;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `Etrsry.xlsx`.");
    let mut o_sys_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            o_sys_gl.insert(row[0].to_string(), row[4].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> =
        open_workbook(config_param.ref_file_path_2()).expect("Unable to open `ORA_GL.xlsx`.");
    let mut alm_concat: HashMap<String, String> = HashMap::new();
    let mut ora_mis1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let mut concat = String::new();
            concat.push_str(&row[2].to_string());
            concat.push('_');
            concat.push_str(&row[4].to_string());
            concat.push('_');
            concat.push_str(&row[1].to_string());
            concat.push('_');
            concat.push_str(&row[5].to_string());
            alm_concat.insert(row[0].to_string(), concat);
            ora_mis1.insert(row[0].to_string(), row[2].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.ref_file_path_3()).expect("Unable to open `MIS1_Desc.xlsx`.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Unable to open `ALM_Line_Master.xlsx`.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
        }
    }
    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let start_derive_timer = SystemTime::now();
    let mut op_bor: String = String::new();
    let mut op_len: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_bor_acc: i64 = 0;
    let mut tot_len_acc: i64 = 0;
    let mut skp_rec: i64 = 0;
    let mut tot_amt: f64 = 0.0;
    let mut concats: Vec<String> = Vec::new();
    let mut input_file: Xls<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            let acc_info = row[0].to_string();
            let mut fields: Vec<&str> = acc_info.split('#').collect();
            if fields[0] == "" || fields[0] == "DEALID" {
                continue;
            }
            if fields.len() != 24 {
                log_debug!(log, "Skipped Record: `{:?}`", fields);
                skp_rec += 1;
                continue;
            }
            let oper_type = fields[2];
            if oper_type.to_uppercase() == "BORROWING" {
                tot_bor_acc += 1;
                let mut output = get_op_line(
                    &mut fields,
                    &mut o_sys_gl,
                    &mut ora_mis1,
                    &mut div,
                    &mut alm_concat,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                );
                op_bor.push_str(&output.processed_lines);
                if let Some(concat) = output.concat_lines.pop() {
                    concats.push(concat);
                }

                let ccy = match fields[7] {
                    "INR" => "INR",
                    "USD" => "USD",
                    _ => "FCY",
                };
                let recon_key = ReconKey::new(
                    ccy.to_string(),
                    "BORR-LEND-FCY".to_string(),
                    o_sys_gl
                        .entry(fields[1].to_string())
                        .or_insert_with(|| "".to_string())
                        .to_string(),
                );
                let amt: f64 = fields[10].parse().unwrap_or(0.0);
                tot_amt += amt;
                recon
                    .entry(recon_key)
                    .and_modify(|val| *val += amt)
                    .or_insert(amt);
                output.clear();
            } else if oper_type.to_uppercase() == "LENDING" {
                tot_len_acc += 1;
                let mut output = get_op_line(
                    &mut fields,
                    &mut o_sys_gl,
                    &mut ora_mis1,
                    &mut div,
                    &mut alm_concat,
                    &mut alm_line,
                    *config_param.as_on_date(),
                    &log,
                );
                op_len.push_str(&output.processed_lines);
                if let Some(concat) = output.concat_lines.pop() {
                    concats.push(concat);
                }

                let ccy = match fields[7] {
                    "INR" => "INR",
                    "USD" => "USD",
                    _ => "FCY",
                };
                let recon_key = ReconKey::new(
                    ccy.to_string(),
                    "BORR-LEND-FCY".to_string(),
                    o_sys_gl
                        .entry(fields[1].to_string())
                        .or_insert_with(|| "".to_string())
                        .to_string(),
                );
                let amt: f64 = fields[10].parse().unwrap_or(0.0);
                tot_amt += amt;
                recon
                    .entry(recon_key)
                    .and_modify(|val| *val += amt)
                    .or_insert(amt);
            } else {
                skp_rec += 1;
                log_error!(
                    log,
                    "`Operation Type` not well-formatted for account: `{}`.",
                    fields[2]
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
            "Unable to create borrowings output file: `{}` on location `{}` : {}",
            config_param.output_file_path_borrowings(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match bor_writer.write_all(op_bor.as_bytes()) {
        Ok(_) => println!("Successfully processed all borrowing accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path_borrowings(),
            error,
        ),
    }

    let mut len_writer = match buf_file_wrtr(config_param.output_file_path_lendings(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create lendings output file: `{}` on location `{}` : {}",
            config_param.output_file_path_borrowings(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    match len_writer.write_all(op_len.as_bytes()) {
        Ok(_) => println!("Successfully processed all lending accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path_borrowings(),
            error,
        ),
    }
    let mut recon_writer = match buf_file_wrtr(config_param.rec_output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create reconcilation file: `{}` on location `{}` : {}",
            config_param.rec_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };

    let mut recon_op_line = String::new();
    for (key, value) in recon {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "HMM3020",
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

    concats.sort();
    concats.dedup();
    for concat in concats.drain(..) {
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
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing records and reconcilation file for Securitisation Loans.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File for Securitisation Loans, Total Duration: {:?}.",
        duration
    );
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing records and reconcilation file.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_bor_acc + tot_len_acc + skp_rec,
        tot_bor_acc + tot_len_acc,
        skp_rec,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(
        &config_param
            .rec_output_file_path()
            .replace("BORR-LEND-FCYReconRpt.txt", "BORR-LEND-FCY"),
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
