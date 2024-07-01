use self::defesance_logic::implemenation::apply_defesance;
use self::defesance_logic::split::split_by_defesance;
use self::defesance_logic::TradingAccount;
use self::derive_fields::get_op_line;
use self::output_lines::OutputLines;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, Reader, Xls, Xlsx};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;

mod defesance_logic;
mod derive_fields;
mod output_lines;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;
    let mut tot_acc_skpd = 0;
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Unable to open MIS1_Desc.xlsx.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_mis1: HashMap<String, String> = HashMap::new();
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    let mut t_ora_cat: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_mis1.insert(row[0].to_string(), row[2].to_string());
            t_ora_prod.insert(row[0].to_string(), row[4].to_string());
            t_ora_gl.insert(row[0].to_string(), row[1].to_string());
            t_ora_cat.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `ALM_Line_Master.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `Etrsry.xlsx` file.");
    let mut o_sys_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            o_sys_gl.insert(row[0].to_string(), row[4].to_string());
        }
    }
    let mut ref_excel5: Xlsx<_> = open_workbook(config_param.ref_file_path_5())
        .expect("Error while opening `Defeasance.xlsx` file.");
    let mut defeasance: HashMap<String, Vec<f64>> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range(config_param.ref_file_5_sheet_name()) {
        for row in reader.rows() {
            let mut def_amt: Vec<f64> = Vec::new();
            def_amt.push(row[2].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[3].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[4].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[5].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[6].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[7].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(row[8].to_string().parse::<f64>().expect("Not a number!"));
            def_amt.push(std::f64::MAX);
            defeasance.insert(row[1].to_string(), def_amt);
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
    let mut concats: Vec<String> = Vec::new();
    let mut op_line: String = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd: i64 = 0;
    let mut input_file: Xls<_> =
        open_workbook(config_param.input_file_path()).expect("Unable to open `input file`.");
    let mut trading_accounts: HashMap<String, Vec<TradingAccount>> = HashMap::new();
    if let Some(Ok(reader)) = input_file.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            if row[1].to_string().parse::<i64>().is_err() {
                log_debug!(log, "Skipped record: `{:?}`.", row);
                continue;
            }
            if row.len() != 42 {
                tot_acc_skpd += 1;
                continue;
            }
            let mut processed_line = get_op_line(
                &row,
                &mut t_ora_mis1,
                &mut t_ora_prod,
                &mut t_ora_gl,
                &mut t_ora_cat,
                &mut o_sys_gl,
                &mut div,
                &mut alm_line,
                *config_param.as_on_date(),
                &log,
            );
            if let Some(concat) = processed_line.concat_lines.pop() {
                concats.push(concat);
            }
            let fields: Vec<&str> = processed_line.processed_lines.split("|").collect();
            if fields[47] == "Trading\n" {
                let mut desc = fields[1];
                if fields[23] == "TBILL" {
                    desc = "TBILL";
                }
                let mat_date = NaiveDate::parse_from_str(fields[21], "%d-%m-%Y")
                    .expect("Cannot parse mat dt string as Naivedate.");
                if trading_accounts.contains_key(desc) {
                    let value = trading_accounts
                        .get_mut(desc)
                        .expect("Unexpected fail of unwrap while reading traiding accounts.");
                    let new_value = TradingAccount {
                        mat_dt: mat_date,
                        acc_pt: processed_line.processed_lines,
                    };
                    value.push(new_value);
                } else {
                    let value = TradingAccount {
                        mat_dt: mat_date,
                        acc_pt: processed_line.processed_lines.to_string(),
                    };
                    let new_val = vec![value];
                    trading_accounts.insert(desc.to_string(), new_val);
                }
            } else {
                op_line.push_str(&processed_line.processed_lines);
            }
            let recon_key = ReconKey::new(
                "INR".to_string(),
                "INVESTMENTS".to_string(),
                row[19].to_string(),
            );
            let amt: f64 = row[38].to_string().parse().unwrap_or(0.0);
            tot_amt_ip += amt;
            tot_amt_op += amt;
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);

            tot_acc_encntrd += 1;
        }
        for (desc, mut acc_infos) in trading_accounts.drain() {
            acc_infos.sort_by(|a, b| a.mat_dt.cmp(&b.mat_dt));
            let mut prod_desc = "";
            for account in &acc_infos {
                let fields: Vec<&str> = account.acc_pt.split("|").collect();
                prod_desc = fields[23];
            }
            if prod_desc == "GILTS" || prod_desc == "GSFRB" || prod_desc == "TBILL" {
                apply_defesance(
                    desc,
                    acc_infos,
                    defeasance.clone(),
                    log,
                    config_param.as_on_date,
                    &mut op_line,
                );
            } else {
                for account in acc_infos {
                    let fields: Vec<&str> = account.acc_pt.split("|").collect();
                    let amt = fields[16].parse::<f64>().unwrap_or(0.0);
                    split_by_defesance(
                        &fields,
                        fields[0].to_string(),
                        amt,
                        account.mat_dt.format("%d-%m-%Y").to_string(),
                        account.mat_dt.format("%d-%m-%Y").to_string(),
                        &mut op_line,
                        config_param.as_on_date,
                    );
                }
            }
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
            "Unable to write processed lines to the file `{}`: {}.",
            config_param.output_file_path(),
            error,
        ),
    }

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
            "INP010_HST3073",
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
            "Unable to write reconcilation lines to the file `{}`: {}.",
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
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skpd,
        tot_acc_skpd,
    );
    info!(log, "{}", report_string);
    println!("{}", report_string);
    let health_stat = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - tot_acc_skpd,
        tot_acc_skpd,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
}
