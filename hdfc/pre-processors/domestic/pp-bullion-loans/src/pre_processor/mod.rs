use self::derive_fields::get_op_line;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::time::SystemTime;
mod derive_fields;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;
    //Reading SMA FILE
    let data_src_name = config_param.data_src_name();
    let mut sma_map: HashMap<String, String> = HashMap::new();
    let sma_file_reader =
        fs::read_to_string(config_param.sma_file_path()).expect("Could not read sma file");
    for (line_no, line) in sma_file_reader.lines().enumerate() {
        let sma_data_vec: Vec<&str> = line.split(',').collect::<Vec<&str>>();
        let data_src_name_1 = get_str(config_param.sma_file_path(), &sma_data_vec, 1, line_no);
        let acc_id = get_str(config_param.sma_file_path(), &sma_data_vec, 2, line_no);
        let sma_stamping = get_str(config_param.input_file_path(), &sma_data_vec, 14, line_no);
        if data_src_name_1.to_uppercase() == data_src_name.trim().to_uppercase() {
            sma_map.insert(acc_id, sma_stamping);
        }
    }
    let file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut alm_line = String::new();
    let mut ia_line = String::new();
    let mut div = String::new();
    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.ref_file_path_1())
        .expect("Unable to open `GL_ALM_Mapping.xlsx`.");
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            if row[0].to_string() == "1003" {
                alm_line.push_str(&row[3].to_string());
                ia_line.push_str(&row[5].to_string());
                div.push_str(&row[4].to_string());
            }
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
    let mut op_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut acc_pro_suc: i64 = 0;
    let mut skp_rec: i64 = 0;
    //adding header to output
    let header = "acc_no|cntr_party|ccy|gl_no|amt|int_rt|st_dt|mat_dt|des|alm_line|div|cust_typ|compmis1|compmis2|compmis3|prod_category|ia_line|sma_flag\n";
    op_line.push_str(&header);
    
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
        let mut fields: Vec<&str> = line.split(&['|', ','][..]).collect();
        if fields.len() != 27 {
            skp_rec += 1;
            continue;
        }
        if fields[26].parse::<f64>().is_err() {
            skp_rec += 1;
            continue;
        }
        let acc_id = get_str(config_param.input_file_path(), &fields, 4, line_num);
        let sma_flag = sma_map.get(&acc_id).unwrap_or(&"P".to_string()).to_string();
        op_line.push_str(&get_op_line(
            &mut fields,
            config_param.currency(),
            &div,
            &alm_line,
            &ia_line,
            &log,
        ));
        op_line.push_str("|");
        op_line.push_str(&sma_flag);
        op_line.push('\n');
        let recon_key = ReconKey::new(
            config_param.currency().to_string(),
            "BullionLoans".to_string(),
            fields[24].to_string(),
        );
        let amt: f64 = fields[26].parse().unwrap_or(0.0);
        tot_amt_ip += amt;
        tot_amt_op += amt;
        recon
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);

        acc_pro_suc += 1;
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

    match writer.write_all(op_line.as_bytes()) {
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
            "Unable to create reconcilation file: `{}` on location `{}` : {}",
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
            "Bullion_Loan",
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
        .expect("Could not calculate total duration for writing records and reconcilation file.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd, acc_pro_suc, skp_rec,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        skp_rec,
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

