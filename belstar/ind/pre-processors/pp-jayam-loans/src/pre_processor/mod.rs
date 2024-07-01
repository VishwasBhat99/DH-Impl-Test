use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::DateParser;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_timer = SystemTime::now();
    let mut cf_fields: HashMap<String, Vec<String>> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_acc_skipped: i64 = 0;
    let mut tot_acc_succ: i64 = 0;
    let mut tot_cfs: i64 = 0;

    let dt_parser = DateParser::new("%d-%b-%y".to_string(), false);

    let input_master_file = match new_buf_rdr(config_param.input_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input master file: `{}` on location `{}` : {}.",
            config_param.input_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let cashflow_file = match new_buf_rdr(config_param.input_cashflow_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found input cashflow file: `{}` on location `{}` : {}.",
            config_param.input_cashflow_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
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

    for (line_num, lines) in cashflow_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_info!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_cashflow_file_path(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = line.split(config_param.cf_delimiter()).collect();
        let mut inp_values: Vec<String> = Vec::new();
        if fields.len() >= 4 {
            for index in 0..fields.len() {
                inp_values.push(fields[index].to_string());
            }
            cf_fields.insert(fields[4].to_string(), inp_values);
        } else {
            log_info!(
                log,
                "CF file account skipped due to invalid data, line num: {}",
                line_num + 1
            );
        }
    }
    let mut op_line: String = String::new();
    //adding header to output
    let header = "Loan_account_no|Cust_id|Cust_name|Prod_name|Num_of_installments|Sanc_amt|Disb_amt|Prin OS|OdPrin|OdInt|Int_Rate|Int_type|Currency|Branch_cd|Value_dt|Mat_dt|Gl_code|CFDate|CF_Principal|CF_Interest|CF_Principal_OS|Spread|Sec_percentage|Last_payment_date|Next_reset_dt|Last_reset_dt|Division|alm_line|ia_llg|balm_llg|Tenure_in_months|Remaninig_tenure|As_on_date|cust_typ|npa_typ\n";
    op_line.push_str(&header);
    writer.write_all(op_line.as_bytes()).unwrap();
    for (line_num, lines) in input_master_file.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                log_info!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_master_file_path(),
                    line_num + 1,
                    error
                );
                tot_acc_skipped += 1;
                continue;
            }
        };
        let row: Vec<&str> = line.split(&config_param.master_delimiter()).collect();
        if row.len() >= 25 {
            let mut op_line: String = String::new();
            let mut op_line_end: String = String::new();
            let def_vec: Vec<String> = Vec::new();
            let cf_vals = match cf_fields.get(&row[24].to_string()) {
                Some(val) => val,
                None => &def_vec,
            };
            if *cf_vals == def_vec {
                tot_acc_skipped += 1;
                log_info!(
                    log,
                    "Account skipped as no cashflows encountered in Cashflow file, LOAN_ID = {}",
                    row[24]
                );
                continue;
            }
            op_line.push_str(&row[24].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[10].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[11].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[38].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[32].to_string());
            op_line.push_str("|");
            op_line.push_str(&cf_vals[16].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[27].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[45].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[49].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[50].to_string());
            op_line.push_str("|");
            op_line.push_str(&row[36].to_string());
            op_line.push_str("|Fixed|INR|");
            op_line.push_str(&row[5].to_string());
            op_line.push_str("|");
            let val_dt = match dt_parser.parse_opt(&cf_vals[8]) {
                Some(val) => val.format("%d-%m-%Y"),
                None => config_param.as_on_date().format("%d-%m-%Y"),
            };
            op_line.push_str(&val_dt.to_string());
            op_line.push_str("|");
            let mat_dt = match dt_parser.parse_opt(&row[30]) {
                Some(val) => val.format("%d-%m-%Y"),
                None => config_param.as_on_date().format("%d-%m-%Y"),
            };
            op_line.push_str(&mat_dt.to_string());
            op_line.push_str("||");

            op_line_end.push_str("||");
            let mut sec_percentage = match &cf_vals.get(6) {
                Some(val) => val.to_string(),
                None => "".to_string(),
            };
            if sec_percentage.contains("%") {
                sec_percentage = sec_percentage.trim_matches('%').to_string();
            }
            op_line_end.push_str(&sec_percentage);
            op_line_end.push_str("||||||||");
            let tenure_in_mnths = match &cf_vals.get(18) {
                Some(val) => val.to_string(),
                None => "".to_string(),
            };
            let rem_tenure = match &cf_vals.get(19) {
                Some(val) => val.to_string(),
                None => "".to_string(),
            };
            op_line_end.push_str(&tenure_in_mnths);
            op_line_end.push_str("|");
            op_line_end.push_str(&rem_tenure);
            op_line_end.push_str("|");
            op_line_end.push_str(&config_param.as_on_date().format("%d-%m-%Y").to_string());
            op_line_end.push_str("|");
            op_line_end.push_str(&row[38].to_string());
            op_line_end.push_str("|");
            op_line_end.push_str(&row[48].to_string());
            op_line_end.push_str("\n");

            let mut index = 20;
            for _ in 20..cf_vals.len() {
                let mut op_cf = String::new();

                let def_str = "NULL".to_string();
                let cf_dt = match cf_vals.get(index) {
                    Some(val) => val,
                    _ => &def_str,
                };
                let cf_prin_amt = match cf_vals.get(index + 1) {
                    Some(val) => val,
                    _ => &def_str,
                };
                let cf_int_amt = match cf_vals.get(index + 2) {
                    Some(val) => val,
                    _ => &def_str,
                };
                let cf_prin_os_amt = match cf_vals.get(index + 3) {
                    Some(val) => val,
                    _ => &def_str,
                };
                // case when the cashflows are empty
                if cf_prin_amt == "NULL" && cf_int_amt == "NULL" && cf_dt == "NULL" {
                    index += 4;
                    continue;
                }
                let cf_date = dt_parser.parse(cf_dt);
                op_cf.push_str(&cf_date.format("%d-%m-%Y").to_string());
                op_cf.push_str("|");
                op_cf.push_str(&cf_prin_amt.to_string());
                op_cf.push_str("|");
                op_cf.push_str(&cf_int_amt.to_string());
                op_cf.push_str("|");
                op_cf.push_str(&cf_prin_os_amt.to_string());

                let final_acc_with_cfs = op_line.clone() + &op_cf + &op_line_end.clone();
                writer.write_all(final_acc_with_cfs.as_bytes()).unwrap();
                tot_cfs += 1;
                index += 4;
            }
            tot_acc_succ += 1;
        } else {
            tot_acc_skipped += 1;
            log_info!(
                log,
                "Account skipped insufficient input, line num = {}",
                tot_acc_encntrd
            );
        }
    }
    let end_timer = SystemTime::now();
    let duration = end_timer
        .duration_since(start_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd, tot_acc_succ, tot_acc_skipped,
    );
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_succ,
        tot_acc_skipped,
        0.0,
        0.0,
        tot_cfs,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
}
