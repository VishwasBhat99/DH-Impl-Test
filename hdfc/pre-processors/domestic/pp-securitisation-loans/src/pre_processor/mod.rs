use self::derive_fields::get_op_line;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
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
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;
    let file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error,
        ),
    };

    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `MIS1_Desc.xlsx`.");
    let mut ora_mis1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            ora_mis1.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> =
        open_workbook(config_param.ref_file_path_2()).expect("Unable to open `Ora_PROD.xlsx`.");
    let mut ora_prod: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            ora_prod.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.ref_file_path_3()).expect("Unable to open `Ora_GL.xlsx`.");
    let mut ora_gl: HashMap<String, String> = HashMap::new();
    let mut prod_code: HashMap<String, String> = HashMap::new();
    let mut ora_cat: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            ora_gl.insert(row[0].to_string(), row[1].to_string());
            ora_cat.insert(row[0].to_string(), row[5].to_string());
            prod_code.insert(row[0].to_string(), row[4].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Unable to open `Master_LLG_Updated.xlsx`.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    let mut ia_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
            ia_line.insert(row[0].to_string(), row[7].to_string());
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
    let mut output_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut acc_pro_suc: i64 = 0;
    let mut concats: Vec<String> = Vec::new();
    //adding header to Output
    let header="acc_id|cust_name|pout_bal|acc_int|st_dt|c_dt|gl_cd|int_rt|int_typ|int_bmark|spread|rt_flag|prod_cd|br_id|nxt_pay_dt|comp_freq|comp_freq_incr|mis1|mis2|mis3|ccy|dt|int_portion|prin_pay|ratings|rating_agency|asset_class|div|typ|originator|as_on_dt|rep_freq|nxt_rep_dt|alm_line|yeild|deal_name|ia_line|sma_flag\n";
    output_line.push_str(header);

    //Reading SMA FILE
    let mut sma_map: HashMap<String, String> = HashMap::new();
    let sma_file_reader =
        std::fs::read_to_string(config_param.sma_file_path()).expect("Could not read SMA File");
    for (line_no, line) in sma_file_reader.lines().enumerate() {
        let sma_data_vec: Vec<&str> = line.split(',').collect::<Vec<&str>>();
        let data_src_name_1 = get_str(config_param.sma_file_path(), &sma_data_vec, 1, line_no);
        if data_src_name_1.to_uppercase() == config_param.data_src_name().trim().to_uppercase() {
            sma_map.insert(
                get_str(config_param.sma_file_path(), &sma_data_vec, 2, line_no),
                get_str(config_param.sma_file_path(), &sma_data_vec, 14, line_no),
            );
        }
    }

    for (line_num, lines) in file.lines().enumerate() {
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
        let mut fields: Vec<&str> = line.split('~').collect();
        let cf_dt = NaiveDate::parse_from_str(fields[23], "%d-%m-%Y")
            .expect("Error while getting `cf_date`.");
        let mat_dt = NaiveDate::parse_from_str(fields[5], "%d-%m-%Y")
            .expect("Error while getting `mat_date`.");
        if *config_param.as_on_date() >= cf_dt || cf_dt > mat_dt && !config_param.is_closed() {
            log_debug!(log, "Skipped record: `{}`.", skip_rec);
            continue;
        }
        if fields.len() == 29 {
            let mut output = get_op_line(
                &mut fields,
                &mut alm_line,
                &mut ia_line,
                &mut ora_gl,
                &mut ora_cat,
                &mut prod_code,
                *config_param.as_on_date(),
                log,
                &sma_map,
            );
            output_line.push_str(&output.processed_lines);
            if let Some(concat) = output.concat_lines.pop() {
                concats.push(concat);
            }

            let recon_key = ReconKey::new(
                "INR".to_string(),
                "SEC-LOANS".to_string(),
                fields[9].to_string(),
            );

            let amt: f64 = fields[25].parse().unwrap_or(0.0);
            tot_amt_ip += amt;
            tot_amt_op += amt;
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);
            output.clear();
            acc_pro_suc += 1;
        } else {
            log_debug!(log, "Skipped record: `{}`.", skip_rec);
            continue;
        }
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
            "p_code_018_total",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_op_line.push_str(&op[..]);
        recon_op_line.push('\n');
    }
    match recon_writer.write_all(recon_op_line.as_bytes()) {
        Ok(_) => println!("Successfully written reconcilation file."),
        Err(error) => panic!(
            "Unable to write reconcilation lines to file `{}`: {}.",
            config_param.rec_output_file_path(),
            error
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
    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
