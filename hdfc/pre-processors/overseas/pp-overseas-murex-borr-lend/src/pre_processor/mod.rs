use self::alm_concat::*;
use self::alm_concat::{get_concat, get_concat_line};
use self::reconcilation::ReconKey;
use self::structs::CurrencyConverter;
use calamine::{open_workbook, Reader, Xlsx};
use chrono::NaiveDate;
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
    let file_read = fs::read_to_string(config_param.input_file_path().to_string())
        .expect("Unable to read file");
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
    let mut funding_source_file: Xlsx<_> = open_workbook(config_param.funding_source_file_path())
        .expect("Error while opening `Funding source` file.");
    let mut funding_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        funding_source_file.worksheet_range(config_param.funding_source_sheet_name())
    {
        for row in reader.rows() {
            funding_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> =
        open_workbook(config_param.ref_file_path_4()).expect("Unable to open `Master_LLG.xlsx`.");
    let mut master_llg: HashMap<String, ALMMasterFields> = HashMap::new();
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
            master_llg.insert(
                row[1].to_string(),
                ALMMasterFields::new(row[6].to_string(), row[7].to_string(), row[9].to_string()),
            );
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

    let start_derive_timer = SystemTime::now();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_bor_acc: i64 = 0;
    let mut tot_len_acc: i64 = 0;
    let mut skp_rec: i64 = 0;
    let mut tot_amt: f64 = 0.0;
    let mut acc_vec: Vec<String> = Vec::new();
    let mut concat = String::new();
    //adding header to output
    let header = "deal_id|branch|inst_name|lend_borr_typ|typology|usage|sub_typ_borr_lend|cntrprty|crtn_dt|val_date|deal_date|ccy|crnt_deal_amt|crnt_conv_rt_lcy|crnt_deal_amt_lcy|roi|tenor_days|mat_dt|prin_amt|int_amt|cf_typ|flow_typ|mat_amt|dealer_name|nds_ref_no|nxt_fix_dt|residual_tenor|nxt_put_dt|nxt_call_dt|nxt_int_pay_dt|int_pay_tenor|aip_air|downgrade_clause|avg_monthly_bal|glcode|cntrprty_ctgry_1|cntrprty_ctgry_2|cntrprty_ctgry_3|cntrprty_ctgry_4|int_pay_rec|bckt_days|country|system_gl|prod_concat|alm_concat|div|alm_line|ia_line|balm_l2|funding_source\n";
    concat.push_str(&header);
    let mut writers = Writers::create_writer(
        config_param.output_file_path_borrowings(),
        config_param.output_file_path_lendings(),
        config_param.concat_file(),
    );
    match writers.borr_cf_writer.write_all(header.as_bytes()) {
        Ok(_) => {}
        Err(error) => panic!(
            "Unable to write header lines to file `{}`: {}.",
            config_param.output_file_path_borrowings(),
            error,
        ),
    };
    match writers.lend_cf_writer.write_all(header.as_bytes()) {
        Ok(_) => {}
        Err(error) => panic!(
            "Unable to write header lines to file `{}`: {}.",
            config_param.output_file_path_lendings(),
            error,
        ),
    };

    for row in file_read.lines() {
        let mut fields: Vec<&str> = row.split('~').collect();
        if fields.len() != 41 {
            log_debug!(log, "Skipped Record: `{:?}`", fields);
            skp_rec += 1;
            continue;
        }
        if fields[1] != config_param.entity() {
            continue;
        }
        let val_date = NaiveDate::parse_from_str(fields[9], "%d-%m-%Y").expect("Val date error");

        if val_date > *config_param.as_on_date() {
            continue;
        }
        let oper_type = fields[3].to_string();
        let product_concat = fields[1].to_string().as_str().replace("\u{a0}", " ")
            + "_"
            + &fields[5].to_string().as_str().replace("\u{a0}", " ")
            + "_"
            + &oper_type.as_str().replace("\u{a0}", " ")
            + "_"
            + &fields[6].to_string().as_str().replace("\u{a0}", " ")
            + "_"
            + &fields[34].to_string().replace("\u{a0}", " ")
            + "_"
            + &fields[40].to_string().as_str().replace("\u{a0}", " ");
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
            + &alm_concat_write.ora_gl.to_string();
        let division = match mis_desc.get(&alm_concat_write.ora_mis1.to_string()) {
            Some(value) => value.to_owned(),
            None => "NONE".to_string(),
        };
        let alm_fields = match master_llg.get(&alm_concat) {
            Some(value) => value.to_owned(),
            None => ALMMasterFields::default(),
        };
        if alm_fields.alm_line == "NONE" {
            concat.push_str(&get_concat_line(fields[0], &gl, &alm_concat));
            concat.push_str("\n");
        }
        let funding_source = match funding_map.get(fields[0]) {
            Some(value) => value.to_owned(),
            None => "NONE".to_string(),
        };
        let mut amt = 0.0;
        let ccy = fields[11];
        if fields[20].to_uppercase() == "CAP" {
            fields[19] = "PRINCIPAL";
            amt = fields[18].parse::<f64>().unwrap_or(0.0).abs();
            if !config_param.is_consolidated() {
                let ex_rt: f64 = match ccy_conv.get(&ccy.to_string()) {
                    Some(val) => *val,
                    None => panic!("Exchange rate value not found for currency: `{}`.", ccy),
                };
                amt = amt * ex_rt;
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
                &alm_fields,
                &funding_source,
            );
            if !acc_vec.contains(&fields[0].to_string()) {
                write_air_aip(
                    &fields,
                    &mut writers.borr_aip_writer,
                    &gl,
                    &alm_concat,
                    &product_concat,
                    &division,
                    &alm_fields,
                    &funding_source,
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
                &alm_fields,
                &funding_source,
            );
            if !acc_vec.contains(&fields[0].to_string()) {
                write_air_aip(
                    &fields,
                    &mut writers.lend_air_writer,
                    &gl,
                    &alm_concat,
                    &product_concat,
                    &division,
                    &alm_fields,
                    &funding_source,
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
