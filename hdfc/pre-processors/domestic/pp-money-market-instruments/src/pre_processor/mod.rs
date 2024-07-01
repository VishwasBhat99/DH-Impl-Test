use self::alm_concat::Concat;
use self::alm_concat::{get_concat, get_concat_line};
use self::reconcilation::ReconKey;
use self::structs::CurrencyConverter;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use csv::ReaderBuilder;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::time::SystemTime;
use writer::Writers;
use writer::{write_air_aip, write_cf};

pub mod alm_concat;
mod reconcilation;
mod structs;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let file_read =
        fs::read_to_string(config_param.input_file_path()).expect("Unable to read file");
    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.ref_file_path_1())
        .expect("Unable to open `Murex_MM_Master.xlsx`.");
    let mut o_sys_gl: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            o_sys_gl.insert(row[4].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> =
        open_workbook(config_param.ref_file_path_2()).expect("Unable to open `ORA_GL.xlsx`.");
    let mut ora_gl: HashMap<String, Concat> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            let alm_concat_fields: Concat = get_concat(
                row[2].to_string(),
                row[4].to_string(),
                row[1].to_string(),
                row[5].to_string(),
            );
            ora_gl.insert(row[0].to_string(), alm_concat_fields);
        }
    }
    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.ref_file_path_3()).expect("Unable to open `MIS1_Desc.xlsx`.");
    let mut mis_desc: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            mis_desc.insert(row[1].to_string(), row[2].to_string());
        }
    }
    let mut ref_excel4: Xlsx<_> =
        open_workbook(config_param.ref_file_path_4()).expect("Unable to open `Master_LLG.xlsx`.");
    let mut master_llg: HashMap<String, String> = HashMap::new();
    let mut ia_line_map: HashMap<String, String> = HashMap::new();
    let mut duplicate_gl: Vec<String> = Vec::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(config_param.sheet_name()) {
        for row in reader.rows() {
            if master_llg.contains_key(&row[2].to_string()) {
                if duplicate_gl.contains(&row[2].to_string()) {
                } else {
                    log_info!(log, "Duplicate GL Code : {}", row[2]);
                    duplicate_gl.push(row[2].to_string());
                }
            }
            master_llg.insert(row[0].to_string(), row[6].to_string());
            ia_line_map.insert(row[0].to_string(), row[7].to_string());
        }
    }

    let mut reader = match ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(config_param.ex_rt_file_path())
    {
        Ok(read) => read,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ex_rt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let mut ccy_conv: HashMap<String, f64> = HashMap::new();
    for (line_num, lines) in reader.deserialize().enumerate() {
        let ccy: CurrencyConverter = match lines {
            Ok(line) => line,
            Err(error) => {
                log_error!(
                    log,
                    "Unable to read file `{}` at line number: `{}` : {}",
                    config_param.input_file_path(),
                    line_num + 1,
                    error
                );
                Default::default()
            }
        };
        if ccy.target == config_param.lcy() {
            ccy_conv.insert(ccy.source, ccy.ex_rt);
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
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_bor_acc: i64 = 0;
    let mut tot_len_acc: i64 = 0;
    let mut skp_rec: i64 = 0;
    let mut tot_amt: f64 = 0.0;
    let mut acc_vec: Vec<String> = Vec::new();
    let mut concat = String::new();
    let mut writers = Writers::create_writer(
        config_param.output_file_path_borrowings(),
        config_param.output_file_path_lendings(),
        config_param.concat_file(),
    );
    let mut prev_deal_id = "";
    for row in file_read.lines() {
        let mut fields: Vec<&str> = row.split('~').collect();
        if fields.len() != 42 {
            log_debug!(log, "Skipped Record: `{:?}`", fields);
            skp_rec += 1;
            continue;
        }
        if fields[1] != config_param.entity() {
            skp_rec += 1;
            continue;
        }
        let oper_type = fields[3].to_string();
        let product_concat = fields[1].to_string().as_str().replace('\u{a0}', " ")
            + "_"
            + &fields[5].to_string().as_str().replace('\u{a0}', " ")
            + "_"
            + &oper_type.as_str().replace('\u{a0}', " ")
            + "_"
            + &fields[6].to_string().as_str().replace('\u{a0}', " ")
            + "_"
            + &fields[40].to_string().as_str().replace('\u{a0}', " ")
            + "_"
            + &fields[41].to_string().as_str().replace('\u{a0}', " ");
        let gl = match o_sys_gl.get(&product_concat) {
            Some(value) => value.to_string(),
            None => "NONE".to_string(),
        };
        let alm_concat_write = match ora_gl.get(&gl) {
            Some(value) => value.to_owned(),
            None => Concat {
                ..Default::default()
            },
        };
        let alm_concat = alm_concat_write.ora_mis1.to_string()
            + "_"
            + &alm_concat_write.ora_prod.to_string()
            + "_"
            + &alm_concat_write.ora_gl.to_string()
            + "_"
            + &alm_concat_write.ora_catogery.to_string();
        let division = match mis_desc.get(&alm_concat_write.ora_mis1.to_string()) {
            Some(value) => value.to_owned(),
            None => "NONE".to_string(),
        };
        let alm_line = match master_llg.get(&alm_concat) {
            Some(value) => value.to_owned(),
            None => "NONE".to_string(),
        };
        if alm_line == "NONE" {
            concat.push_str(&get_concat_line(fields[0], &gl, &alm_concat));
            concat.push('\n');
        }
        let ia_line = match ia_line_map.get(&alm_concat) {
            Some(value) => value.to_owned(),
            None => "NONE".to_string(),
        };
        let mut amt = 0.0;
        let ccy = fields[11];
        if fields[20].to_uppercase() == "CAP" {
            fields[19] = "PRINCIPAL";
            amt = if prev_deal_id != fields[0] {
                prev_deal_id = fields[0];
                fields[12].parse::<f64>().unwrap_or(0.0).abs()
            } else {
                0.0
            };
            if !config_param.is_consolidated() {
                let ex_rt: f64 = match ccy_conv.get(&ccy.to_string()) {
                    Some(val) => *val,
                    None => panic!("Exchange rate value not found for currency: `{}`.", ccy),
                };
                amt *= ex_rt;
            }
        } else if fields[20].to_uppercase() == "REV" {
            fields[19] = "INTEREST";
        }
        if oper_type.to_uppercase() == "BORROW" {
            tot_bor_acc += 1;
            write_cf(
                &fields,
                &mut writers.borr_cf_writer,
                &gl,
                &alm_concat,
                &product_concat,
                &division,
                &alm_line,
                &ia_line,
                "BORROW",
                &sma_map,
            );
            if !acc_vec.contains(&fields[0].to_string()) {
                write_air_aip(
                    &fields,
                    &mut writers.borr_aip_writer,
                    &gl,
                    &alm_concat,
                    &product_concat,
                    &division,
                    &alm_line,
                    &ia_line,
                    &sma_map,
                );
                acc_vec.push(fields[0].to_string());
            }
            let recon_key = ReconKey::new(
                ccy.to_string(),
                "BORR-LEND".to_string(),
                (*o_sys_gl
                    .entry(product_concat)
                    .or_insert_with(|| "".to_string()))
                .to_string(),
            );
            tot_amt += amt;
            recon
                .entry(recon_key)
                .and_modify(|val| *val += amt)
                .or_insert(amt);
        } else if oper_type.to_uppercase() == "LEND" {
            tot_len_acc += 1;
            write_cf(
                &fields,
                &mut writers.lend_cf_writer,
                &gl,
                &alm_concat,
                &product_concat,
                &division,
                &alm_line,
                &ia_line,
                "LEND",
                &sma_map,
            );
            if !acc_vec.contains(&fields[0].to_string()) {
                write_air_aip(
                    &fields,
                    &mut writers.lend_air_writer,
                    &gl,
                    &alm_concat,
                    &product_concat,
                    &division,
                    &alm_line,
                    &ia_line,
                    &sma_map,
                );
                acc_vec.push(fields[0].to_string());
            }
            let recon_key = ReconKey::new(
                ccy.to_string(),
                "BORR-LEND".to_string(),
                (*o_sys_gl
                    .entry(product_concat)
                    .or_insert_with(|| "".to_string()))
                .to_string(),
            );
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

    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);

    let start_write_timer = SystemTime::now();
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
            "MoneyMarket",
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
    match writers.concat_writer.write_all(concat.as_bytes()) {
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

    let health_report = HealthReport::new(
        tot_bor_acc + tot_len_acc + skp_rec,
        tot_bor_acc + tot_len_acc,
        skp_rec,
        tot_amt,
        tot_amt,
        0,
    );
    log_info!(log, "{}", health_report.display());
    println!("{}", health_report.display());
    health_report.gen_health_rpt(
        &config_param
            .rec_output_file_path()
            .replace("BORR-LEND-ReconRpt.txt", "BORR-LEND"),
    );
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
