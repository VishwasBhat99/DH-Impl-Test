use self::derive_fields::get_op_line;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use pre_processor::reconcilation::ReconKey;
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
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open `Ora_PROD.xlsx`.");
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.ref1_sheet_name()) {
        for row in reader.rows() {
            t_ora_prod.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_gl: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.ref2_sheet_name()) {
        for row in reader.rows() {
            let mut t_ora_gl_fields: Vec<String> = Vec::new();
            t_ora_gl_fields.push(row[1].to_string());
            t_ora_gl_fields.push(row[4].to_string());
            t_ora_gl.insert(row[0].to_string(), t_ora_gl_fields);
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `ALM_COA_Master.xlsx` file.");
    let mut t_bdp_coa: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(config_param.ref4_sheet_name()) {
        for row in reader.rows() {
            t_bdp_coa.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `MIS1_Desc.xlsx` file.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range(config_param.ref5_sheet_name()) {
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
    let mut counter = 1;
    let mut op_line: String = String::new();
    let mut output_concat_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut skip_rec: i64 = 0;
    let mut tot_acc_encntrd: i64 = 0;
    //adding header to output
    let header="reference|cust|curr|val_dt|max_dt|txn_dt|npa_stats|cntrct_stats|closr_dt|due_dt_prin|amt|lcy_amt|gl|int_rt|cust_name|comp_mis1|comp_mis2|loan_type|bank|acurl_basis|div|alm_line|ia_llg|balm_llg|as_on_dt|exchange_rt|asset_class|int_st_dt|bal_os_amt_lcy|bill_amt|concat|rate_flag|comp_mis3\n";
    op_line.push_str(&header);

    let mut prod_cd: Vec<&str> = Vec::new();
    match config_param.gl_type() {
        "BH-Over-Bills" => {
            prod_cd.push("BC54");
            prod_cd.push("BC91")
        }
        "GC-Over-Bills" => {
            prod_cd.push("BC54");
            prod_cd.push("BC91")
        }
        "HK-Over-Bills" => {
            prod_cd.push("BC54");
            prod_cd.push("BC91")
        }
        _ => panic!("Invalid `gl_type`"),
    };
    let mut tot_amt: f64 = 0.0;
    let mut input_file: Xlsx<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for mut row in reader.rows() {
            tot_acc_encntrd += 1;
            if counter < 3 {
                counter += 1;
                if row[0].to_string().parse::<i64>().is_err() {
                    log_debug!(log, "Skipped record: `{:?}`.", row);
                    skip_rec += 1;
                    continue;
                }
            }
            if row[4].to_string().is_empty() || !prod_cd.contains(&row[3].to_string().as_str()) {
                log_debug!(diag_log, "Skipped record: `{:?}`.", row);
                skip_rec += 1;
                continue;
            }

            let (op, concat) = get_op_line(
                &mut row,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_bdp_coa,
                &mut div,
                config_param.alm_line().to_string(),
                *config_param.as_on_date(),
                &log,
            );
            op_line.push_str(&op);
            output_concat_line.push_str(&concat);

            let recon_key = ReconKey::new(
                row[14].to_string(),
                config_param.gl_type().to_string(),
                "".to_string(), // TODO: missing gl code
            );

            let amt: f64 = remove_comma(row[25].to_string()).parse().unwrap_or(0.0);
            tot_amt += amt;
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);
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

    match writer.write_all(op_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }

    let mut concat_writer = match buf_file_wrtr(config_param.output_concat_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output concat file: `{}` on location `{}` : {}",
            config_param.output_concat_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match concat_writer.write_all(output_concat_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_concat_file_path(),
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
        tot_acc_encntrd - skip_rec,
        skip_rec,
    );
    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec,
        skip_rec,
        tot_amt,
        tot_amt,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
    log_info!(log, "{}", report_string);
    println!("{}", report_string);
}

pub fn remove_comma(field: String) -> String {
    let mut rem_comma: String = field;
    rem_comma.retain(|amt| amt != ',');
    rem_comma
}
