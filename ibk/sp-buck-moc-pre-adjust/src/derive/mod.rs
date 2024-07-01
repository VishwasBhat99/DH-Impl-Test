use calamine::{open_workbook, Reader, Xlsx};
use chrono::{Datelike, NaiveDate};
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;
use xlsxwriter::*;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();

    let start_derive_timer = SystemTime::now();
    let mut skip_rec: i64 = 0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut tot_amt_ip = 0.0;
    let mut tot_amt_op = 0.0;

    let inp_file = match new_buf_rdr(config_param.input_file_path()) {
        Ok(inp_file) => inp_file,
        Err(error) => panic!(
            "Could not found inp file: `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut op_line = String::new();
    op_line.push_str(
        "SchemeID|BucketID|BucketDescription|StartPeriod|EndPeriod|StartUOM|EndUOM|MaturityDate\n",
    );
    let mut moc_buc_mat_dt: HashMap<String, NaiveDate> = HashMap::new();

    for (line_num, lines) in inp_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };

        let mut fields: Vec<&str> = line.split('|').collect();

        op_line.push_str(&line);
        op_line.push('|');

        let mut cf_date;
        match fields[6].trim().to_lowercase().as_str() {
            "days" => {
                cf_date = rbdate::incr_dt_by_days(
                    *config_param.as_on_date(),
                    fields[4].parse().expect("days need to be integer."),
                );
            }
            "month" => {
                cf_date = rbdate::incr_dt_by_mon_presrv_eom_checked(
                    *config_param.as_on_date(),
                    fields[4].parse().expect("months need to be integer."),
                )
                .expect("could not add months to the date.");
            }
            _ => {
                cf_date = rbdate::incr_dt_by_yrs(
                    *config_param.as_on_date(),
                    fields[4].parse().expect("years need to be integer."),
                );
            }
        }

        op_line.push_str(&cf_date.format("%d-%m-%Y").to_string());
        op_line.push('\n');
        moc_buc_mat_dt.insert(fields[1].to_string(), cf_date);
    }

    let mut buc_def_out_file =
        File::create(config_param.buc_def_out_file()).expect("Unable to create buc_def_out file");
    buc_def_out_file
        .write_all(op_line.as_bytes())
        .expect("Unable to write data in buc_def_out file");

    let mut ovrdu_llgids: Vec<String> = Vec::new();

    let mut ovrd_llg: Xlsx<_> =
        open_workbook(config_param.ovrd_llg()).expect("Unable to open `OverdueLLG.xlsx`.");
    if let Some(Ok(reader)) = ovrd_llg.worksheet_range(config_param.ovrd_llg_sheet_name()) {
        for row in reader.rows().skip(1) {
            if !row[1].is_empty() {
                ovrdu_llgids.push(row[1].to_string())
            }
        }
    }

    let mut ovrd_kwd_writer = match buf_file_wrtr(config_param.ovrd_kwd_out(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Overdue KWD file: `{}` on location `{}` : {}",
            config_param.buc_def_out_file(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut moc_buc_kwd_writer = match buf_file_wrtr(config_param.buc_moc_kwd_out(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create BucketDefinitionOut file: `{}` on location `{}` : {}",
            config_param.buc_def_out_file(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let op_workbook =
        Workbook::new(config_param.output_file_path()).expect("Error in Adding Worksheet");
    let sheet = op_workbook.add_worksheet(Some(config_param.output_sheet_name()));

    let mut op_sheet;

    if sheet.is_err() {
        panic!("Could not configure output file sheet.");
    } else {
        op_sheet = sheet.unwrap();
    }

    let mut row_num = 0;
    let mut vec_moc_kwd_op: Vec<Vec<String>> = Vec::new();
    let mut ovrd_kwd_op = String::new();
    let mut moc_kwd_op = String::new();
    let mut liq_rpt_kwd: Xlsx<_> = open_workbook(config_param.liq_rpt_kwd())
        .expect("Error while opening `CBK_Liquidity_Report_Adj_KWD.xlsx` file.");
    if let Some(Ok(reader)) = liq_rpt_kwd.worksheet_range(config_param.liq_rpt_kwd_sheet_name()) {
        let mut itr = 0;
        for row in reader.rows().skip(1) {
            if !row[1].is_empty() {
                tot_acc_encntrd += 1;

                ovrd_kwd_op.push_str(&format!(
                    "{}|{}|{}|{}|{}\n",
                    &ovrdu_llgids[itr],
                    &row[2].to_string(),
                    &row[3].to_string(),
                    '0',
                    &rbdate::dcr_dt_by_days(*config_param.as_on_date(), 1)
                        .format("%d-%m-%Y")
                        .to_string()
                ));

                op_sheet.write_string(row_num, 0, &ovrdu_llgids[itr], None);
                op_sheet.write_string(row_num, 1, &row[2].to_string(), None);
                op_sheet.write_string(row_num, 2, &row[3].to_string(), None);
                op_sheet.write_string(row_num, 3, "0", None);
                op_sheet.write_string(
                    row_num,
                    4,
                    &rbdate::dcr_dt_by_days(*config_param.as_on_date(), 1)
                        .format("%d-%m-%Y")
                        .to_string(),
                    None,
                );
                row_num += 1;

                let amt = &row[3].to_string().parse::<f64>();

                if !amt.is_err() {
                    tot_amt_ip += &row[3].to_string().parse().unwrap();
                    tot_amt_op += &row[3].to_string().parse().unwrap();
                } else {
                    error!(log, "Amt could not be parsed in f64");
                }

                for i in 4..row.len() {
                    tot_acc_encntrd += 1;

                    let llg_id = row[1].to_string().to_owned();

                    moc_kwd_op.push_str(&format!(
                        "{}|{}|{}|{}|{}\n",
                        &llg_id,
                        &row[2].to_string(),
                        &row[i].to_string(),
                        '0',
                        &moc_buc_mat_dt
                            .get(&(i - 3).to_string())
                            .unwrap()
                            .format("%d-%m-%Y")
                            .to_string()
                    ));

                    let mut vec_op: Vec<String> = Vec::new();

                    vec_op.push(llg_id);
                    vec_op.push(row[2].to_string());
                    vec_op.push(row[i].to_string());
                    vec_op.push("0".to_string());
                    vec_op.push(
                        moc_buc_mat_dt
                            .get(&(i - 3).to_string())
                            .unwrap()
                            .format("%d-%m-%Y")
                            .to_string(),
                    );
                    vec_moc_kwd_op.push(vec_op);

                    let amt = &row[3].to_string().parse::<f64>();

                    if !amt.is_err() {
                        tot_amt_ip += &row[3].to_string().parse().unwrap();
                        tot_amt_op += &row[3].to_string().parse().unwrap();
                    } else {
                        error!(log, "Amt could not be parsed in f64");
                    }
                }
                itr += 1;
            }
        }
    }

    for vec in &vec_moc_kwd_op {
        let mut col_num = 0;
        for op_val in vec {
            op_sheet.write_string(row_num, col_num, op_val, None);
            col_num += 1;
        }
        row_num += 1;
    }

    match ovrd_kwd_writer.write_all(ovrd_kwd_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully overdue KWD file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.ovrd_kwd_out(),
            error
        ),
    }

    match moc_buc_kwd_writer.write_all(moc_kwd_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully bucket moc KWD file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.buc_moc_kwd_out(),
            error
        ),
    }

    let mut ovrd_fcy_writer = match buf_file_wrtr(config_param.ovrd_fcy_out(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Overdue fcy file: `{}` on location `{}` : {}",
            config_param.buc_def_out_file(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut moc_buc_fcy_writer = match buf_file_wrtr(config_param.buc_moc_fcy_out(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create BucketDefinitionOut file: `{}` on location `{}` : {}",
            config_param.buc_def_out_file(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    let mut ovrd_fcy_op = String::new();
    let mut moc_fcy_op = String::new();
    let mut vec_moc_fcy_op: Vec<Vec<String>> = Vec::new();
    let mut itr = 0;
    let mut liq_rpt_fcy: Xlsx<_> = open_workbook(config_param.liq_rpt_fcy())
        .expect("Error while opening `CBK_Liquidity_Report_Adj_FCY.xlsx` file.");
    if let Some(Ok(reader)) = liq_rpt_fcy.worksheet_range(config_param.liq_rpt_fcy_sheet_name()) {
        for row in reader.rows().skip(1) {
            if !row[1].is_empty() {
                tot_acc_encntrd += 1;

                ovrd_fcy_op.push_str(&format!(
                    "{}|{}|{}|{}|{}\n",
                    &ovrdu_llgids[itr],
                    &row[2].to_string(),
                    &row[3].to_string(),
                    '0',
                    &rbdate::dcr_dt_by_days(*config_param.as_on_date(), 1)
                        .format("%d-%m-%Y")
                        .to_string()
                ));

                op_sheet.write_string(row_num, 0, &ovrdu_llgids[itr], None);
                op_sheet.write_string(row_num, 1, &row[2].to_string(), None);
                op_sheet.write_string(row_num, 2, &row[3].to_string(), None);
                op_sheet.write_string(row_num, 3, "0", None);
                op_sheet.write_string(
                    row_num,
                    4,
                    &rbdate::dcr_dt_by_days(*config_param.as_on_date(), 1)
                        .format("%d-%m-%Y")
                        .to_string(),
                    None,
                );
                row_num += 1;

                let amt = &row[3].to_string().parse::<f64>();

                if !amt.is_err() {
                    tot_amt_ip += &row[3].to_string().parse().unwrap();
                    tot_amt_op += &row[3].to_string().parse().unwrap();
                } else {
                    error!(log, "Amt could not be parsed in f64");
                }

                for i in 4..row.len() {
                    tot_acc_encntrd += 1;

                    moc_fcy_op.push_str(&format!(
                        "{}|{}|{}|{}|{}\n",
                        &row[1].to_string(),
                        &row[2].to_string(),
                        &row[i].to_string(),
                        '0',
                        &moc_buc_mat_dt
                            .get(&(i - 3).to_string())
                            .unwrap()
                            .format("%d-%m-%Y")
                            .to_string()
                    ));

                    let mut vec_op: Vec<String> = Vec::new();

                    vec_op.push(row[1].to_string());
                    vec_op.push(row[2].to_string());
                    vec_op.push(row[i].to_string());
                    vec_op.push("0".to_string());
                    vec_op.push(
                        moc_buc_mat_dt
                            .get(&(i - 3).to_string())
                            .unwrap()
                            .format("%d-%m-%Y")
                            .to_string(),
                    );
                    vec_moc_fcy_op.push(vec_op);

                    let amt = &row[3].to_string().parse::<f64>();

                    if !amt.is_err() {
                        tot_amt_ip += &row[3].to_string().parse().unwrap();
                        tot_amt_op += &row[3].to_string().parse().unwrap();
                    } else {
                        error!(log, "Amt could not be parsed in f64");
                    }
                }
                itr += 1;
            }
        }
    }

    for vec in &vec_moc_fcy_op {
        let mut col_num = 0;
        for op_val in vec {
            op_sheet.write_string(row_num, col_num, op_val, None);
            col_num += 1;
        }
        row_num += 1;
    }

    match ovrd_fcy_writer.write_all(ovrd_fcy_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully written overdue FCY file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.ovrd_fcy_out(),
            error
        ),
    }

    match moc_buc_fcy_writer.write_all(moc_fcy_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully written moc bucket FCY file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.buc_moc_fcy_out(),
            error
        ),
    }

    let mut ovrd_con_writer = match buf_file_wrtr(config_param.ovrd_con_out(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Overdue Consol file: `{}` due to: {}",
            config_param.ovrd_con_out(),
            error
        ),
    };

    let mut moc_buc_con_writer = match buf_file_wrtr(config_param.buc_moc_con_out(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create Bucket Definition Consol file: `{}` due to: {}",
            config_param.buc_moc_con_out(),
            error
        ),
    };

    let mut ovrd_con_op = String::new();
    let mut moc_con_op = String::new();
    let mut vec_moc_con_op: Vec<Vec<String>> = Vec::new();
    let mut itr = 0;
    let mut liq_rpt_con: Xlsx<_> = open_workbook(config_param.liq_rpt_con())
        .expect("Error while opening `CBK_Liquidity_Report_Adj_CON.xlsx` file.");
    if let Some(Ok(reader)) = liq_rpt_con.worksheet_range(config_param.liq_rpt_con_sheet_name()) {
        for row in reader.rows().skip(1) {
            if !row[1].is_empty() {
                tot_acc_encntrd += 1;

                ovrd_con_op.push_str(&format!(
                    "{}|{}|{}|{}|{}\n",
                    &ovrdu_llgids[itr],
                    &row[2].to_string(),
                    &row[3].to_string(),
                    '0',
                    &rbdate::dcr_dt_by_days(*config_param.as_on_date(), 1)
                        .format("%d-%m-%Y")
                        .to_string()
                ));

                op_sheet.write_string(row_num, 0, &ovrdu_llgids[itr], None);
                op_sheet.write_string(row_num, 1, &row[2].to_string(), None);
                op_sheet.write_string(row_num, 2, &row[3].to_string(), None);
                op_sheet.write_string(row_num, 3, "0", None);
                op_sheet.write_string(
                    row_num,
                    4,
                    &rbdate::dcr_dt_by_days(*config_param.as_on_date(), 1)
                        .format("%d-%m-%Y")
                        .to_string(),
                    None,
                );
                row_num += 1;

                let amt = &row[3].to_string().parse::<f64>();

                if !amt.is_err() {
                    tot_amt_ip += &row[3].to_string().parse().unwrap();
                    tot_amt_op += &row[3].to_string().parse().unwrap();
                } else {
                    error!(log, "Amt could not be parsed in f64");
                }

                for i in 4..row.len() {
                    tot_acc_encntrd += 1;

                    moc_con_op.push_str(&format!(
                        "{}|{}|{}|{}|{}\n",
                        &row[1].to_string(),
                        &row[2].to_string(),
                        &row[i].to_string(),
                        '0',
                        &moc_buc_mat_dt
                            .get(&(i - 3).to_string())
                            .unwrap()
                            .format("%d-%m-%Y")
                            .to_string()
                    ));

                    let mut vec_op: Vec<String> = Vec::new();

                    vec_op.push(row[1].to_string());
                    vec_op.push(row[2].to_string());
                    vec_op.push(row[i].to_string());
                    vec_op.push("0".to_string());
                    vec_op.push(
                        moc_buc_mat_dt
                            .get(&(i - 3).to_string())
                            .unwrap()
                            .format("%d-%m-%Y")
                            .to_string(),
                    );
                    vec_moc_con_op.push(vec_op);

                    let amt = &row[3].to_string().parse::<f64>();

                    if !amt.is_err() {
                        tot_amt_ip += &row[3].to_string().parse().unwrap();
                        tot_amt_op += &row[3].to_string().parse().unwrap();
                    } else {
                        error!(log, "Amt could not be parsed in f64");
                    }
                }
                itr += 1;
            }
        }
    }

    for vec in &vec_moc_con_op {
        let mut col_num = 0;
        for op_val in vec {
            op_sheet.write_string(row_num, col_num, op_val, None);
            col_num += 1;
        }
        row_num += 1;
    }

    match ovrd_con_writer.write_all(ovrd_con_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully written overdue CON file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.ovrd_con_out(),
            error
        ),
    }

    match moc_buc_con_writer.write_all(moc_con_op.as_bytes()) {
        Ok(_) => info!(log, "Successfully written moc bucket CON file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.buc_moc_con_out(),
            error
        ),
    }

    let mut op_line_final = String::new();

    op_line_final.push_str("LLGID|CCY|Amt|INT|CFDate\n");
    op_line_final.push_str(&ovrd_kwd_op);
    op_line_final.push_str(&moc_kwd_op);
    op_line_final.push_str(&ovrd_fcy_op);
    op_line_final.push_str(&moc_fcy_op);
    op_line_final.push_str(&ovrd_con_op);
    op_line_final.push_str(&moc_con_op);

    let mut op_final = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create final output file: `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };

    match op_final.write_all(op_line_final.as_bytes()) {
        Ok(_) => info!(log, "Successfully written output file."),
        Err(error) => panic!(
            "Unable to write processed lines to file `{}`: {}.",
            config_param.output_file_path(),
            error
        ),
    }

    let end_read_timer = SystemTime::now();
    let duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration read timer.");
    debug!(
        diag_log,
        "Reading Reference Files, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skip_rec,
        skip_rec,
        tot_amt_ip,
        tot_amt_op,
        0,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());
}
