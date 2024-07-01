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
            "Could not found input file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut ora_gl_file: Xlsx<_> =
        open_workbook(config_param.ora_gl_file()).expect("Unable to open `Ora_GL.xlsx`.");
    let mut ora_gl: HashMap<String, String> = HashMap::new();
    let mut ora_cat: HashMap<String, String> = HashMap::new();
    let mut ora_prod: HashMap<String, String> = HashMap::new();
    let mut ora_mis1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ora_gl_file.worksheet_range("Sheet1") {
        for row in reader.rows() {
            ora_gl.insert(row[0].to_string(), row[1].to_string());
            ora_cat.insert(row[0].to_string(), row[5].to_string());
            ora_prod.insert(row[0].to_string(), row[4].to_string());
            ora_mis1.insert(row[0].to_string(), row[2].to_string());
        }
    }

    let mut alm_master_file: Xlsx<_> = open_workbook(config_param.alm_master_file())
        .expect("Unable to open `ALM_Line_Master.xlsx`.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = alm_master_file.worksheet_range(config_param.alm_master_sheet_name())
    {
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

    let start_derive_timer = SystemTime::now();
    let mut output_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut acc_pro_suc: i64 = 0;
    let mut concats: Vec<String> = Vec::new();
    for (line_num, lines) in file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read input file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num,
                error
            ),
        };
        tot_acc_encntrd += 1;
        let skip_rec = line.to_string();
        let mut fields: Vec<&str> = line.split('~').collect();
        let cf_dt = NaiveDate::parse_from_str(fields[25], "%d-%m-%Y")
            .expect("Error while getting `cf_date`.");
        let mat_dt = NaiveDate::parse_from_str(fields[6], "%d-%m-%Y")
            .expect("Error while getting `mat_date`.");
        if (*config_param.as_on_date() >= cf_dt || cf_dt > mat_dt) && !config_param.is_closed() {
            log_debug!(log, "Skipped record: `{}`.", skip_rec);
            continue;
        }
        if fields.len() == 34 {
            let mut output = get_op_line(
                &mut fields,
                &mut alm_line,
                &mut ora_mis1,
                &mut ora_prod,
                &mut ora_gl,
                &mut ora_cat,
                log,
                &sma_map,
            );
            output_line.push_str(&output.processed_lines);
            if let Some(concat) = output.concat_lines.pop() {
                concats.push(concat);
            }

            let recon_key = ReconKey::new(
                "INR".to_string(),
                "SEC-INVST".to_string(),
                fields[10].to_string(),
            );
            let amt: f64 = fields[27].parse().unwrap_or(0.0);
            tot_amt_ip += amt;
            tot_amt_op += amt;
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert_with(|| amt);

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

    let start_time = SystemTime::now();
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
    writeln!(
        writer,
        "fc_ubs_acc|cust_name|pout_ball|acc_int|st_dt|c_dt|gl_cd|int_rt|int_type|int_bmark|spread|rt_flag|prod_cd|br_id|nxt_pay_dt|comp_freq|comp_freq_incr|mis1|mis2|mis3|ccy|dt|int_portion|prin_pay|ratings|rating_agency|asset_class|div|typ|originator|rep_freq|nxt_rep_dt|portfolio|alm_line|txn_mis2|old_fc_ubs_acc|deal_name|cf_start_date|ubs_acct_no|sma_flag"
    );
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
            "Pcode019(FCUBS accounts)",
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
    let end_time = SystemTime::now();
    let duration = end_time
        .duration_since(start_time)
        .expect("Could not calculate total duration for writing records and reconcilation file.");
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
