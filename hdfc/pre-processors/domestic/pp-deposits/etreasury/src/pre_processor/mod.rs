use self::derive_fields::append_alm_line;
use self::derive_fields::append_as_on_date;
use self::derive_fields::append_currency;
use self::derive_fields::append_current_book_balance;
use self::derive_fields::append_gl_acc;
use self::derive_fields::append_int_rate;
use self::derive_fields::cost_center;
use self::recon::ReconKey;
use self::split_pos::get_split_pos;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

mod derive_fields;
mod recon;
mod split_pos;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_ref_time = SystemTime::now();
    let mut ref_mis1: String = "".to_string();
    let input_file = match File::open(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };

    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.ref_file_path_1()).unwrap();
    let mut ref_map1: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("R1") {
        for row in reader.rows() {
            let mut alm_concat = String::new();
            alm_concat.push_str(&row[4].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[1].to_string());
            alm_concat.push_str("_");
            alm_concat.push_str(&row[5].to_string());
            ref_map1.insert(row[0].to_string(), alm_concat);
            ref_mis1 = row[2].to_string();
        }
    }
    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2()).unwrap();
    let mut ref_map2: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            let alm_line = row[6].to_string();
            ref_map2.insert(row[0].to_string(), alm_line);
        }
    }
    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3()).unwrap();
    let mut ref_map3: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let cost_center = row[1].to_string();
            ref_map3.insert(row[0].to_string(), cost_center);
        }
    }
    let end_read_ref_time = SystemTime::now();
    let total_duration = end_read_ref_time
        .duration_since(start_read_ref_time)
        .expect("Could not calculate total read reference duration.");
    info!(
        diag_log,
        "Reading Reference Total Duration: {:?}", total_duration
    );
    let start_process_time = SystemTime::now();
    let output_file = match File::create(config_param.output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let rec_output_file = match File::create(config_param.rec_output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut recon_writer = BufWriter::new(rec_output_file);
    let mut output_line = String::new();
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    for line in reader.lines() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let split_val = get_split_pos();
        let mut field_num = 1;
        let mut bal_principal_lcy = "";
        let mut rat_acct_int = "";
        let mut rat_acct_int_var = "";
        let mut bal_int_comp_lcy = "";
        let mut cod_gl_regular_dep = "";
        let mut mis1 = "";
        let mut currency = "";
        let mut cost_cen_ftp = "";
        let mut comp_freq = "";
        for val in split_val {
            let st_pos = val.st_pos;
            let end_pos = val.st_pos + val.length;
            let sub_string = acc_info[st_pos..end_pos].trim();
            if field_num == 4 {
                bal_principal_lcy = sub_string;
                field_num += 1;
                continue;
            }
            if field_num == 6 {
                rat_acct_int = sub_string;
            }
            if field_num == 7 {
                rat_acct_int_var = sub_string;
            }
            if field_num == 11 {
                currency = sub_string;
            }
            if field_num == 17 {
                cod_gl_regular_dep = sub_string;
                field_num += 1;
                continue;
            }
            if field_num == 19 {
                output_line.push_str("FWTD");
                output_line.push_str("|");
                append_as_on_date(&mut output_line, config_param.as_on_date);
                output_line.push_str("000");
                output_line.push_str("|");
            }
            if field_num == 20 {
                output_line.push_str("F");
                output_line.push_str("|");
            }
            if field_num == 21 {
                bal_int_comp_lcy = sub_string;
                field_num += 1;
                continue;
            }
            if field_num == 22 {
                cost_cen_ftp = sub_string;
                output_line.push_str(cost_cen_ftp);
                output_line.push_str("|");
                field_num += 1;
                continue;
            }
            if field_num == 23 {
                comp_freq = sub_string;
                field_num += 1;
                continue;
            }
            if field_num == 24 {
                mis1 = sub_string;
                field_num += 1;
                continue;
            }
            if field_num == 5
                || field_num == 8
                || field_num == 9
                || field_num == 10
                || field_num == 14
                || field_num == 15
            {
                let dt = rbdate::NaiveDate::parse_from_str(sub_string, "%d-%b-%Y")
                    .expect("Cannot get NaiveDate from str.");
                output_line.push_str(&dt.format("%d-%m-%Y").to_string());
                output_line.push_str("|");
                field_num += 1;
                continue;
            }
            output_line.push_str(sub_string);
            output_line.push_str("|");
            field_num += 1;
        }
        let currency = append_currency(&mut output_line, currency);
        let cip_gl = acc_info[393..402].trim();
        let cod_gl = acc_info[328..337].trim();
        let cip_recon_key =
            ReconKey::new(currency.clone(), "CIPGL".to_string(), cip_gl.to_string());
        let cod_recon_key = ReconKey::new(currency, "PRDGL".to_string(), cod_gl.to_string());
        append_gl_acc(&mut output_line, cod_gl_regular_dep);
        append_int_rate(&mut output_line, rat_acct_int, rat_acct_int_var);
        let cost_center = cost_center(&ref_map3, cod_gl_regular_dep, cost_cen_ftp);
        append_alm_line(
            &mut output_line,
            &ref_map1,
            &ref_map2,
            cod_gl_regular_dep,
            &cost_center,
            mis1,
            &ref_mis1,
            log,
            acc_info[0..20].trim(),
        );
        append_current_book_balance(
            &mut output_line,
            bal_principal_lcy,
            bal_int_comp_lcy,
            cip_recon_key,
            cod_recon_key,
            &mut recon_map,
        );
        output_line.push_str(&cost_center);
        output_line.push_str("|");
        output_line.push_str(&comp_freq);
        output_line.push_str("|");
        output_line.push_str("\n");
    }
    let end_process_time = SystemTime::now();
    let duration = end_process_time
        .duration_since(start_process_time)
        .expect("Could not calculate total process duration.");
    info!(diag_log, "Process Total Duration: {:?}.", duration);
    let start_writer_time = SystemTime::now();
    let mut recon_output_line = String::new();
    for (key, value) in recon_map {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date().format("%d-%m-%Y"),
            "TD353",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_output_line.push_str(&op[..]);
        recon_output_line.push_str("\n");
    }
    match recon_writer.write_all(recon_output_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing reconciliation report: {:?}", error);
        }
    }
    match writer.write_all(output_line.as_bytes()) {
        Ok(val) => val,
        Err(error) => {
            panic!("Error writing processed data: {:?}", error);
        }
    }
    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
}
