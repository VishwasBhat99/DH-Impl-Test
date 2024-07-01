use self::derive_fields::append_alm_ia_balm_line;
use self::derive_fields::append_as_on_date;
use self::derive_fields::append_last_rep_date;
use self::derive_fields::append_next_rep_date;
use self::derive_fields::append_next_rep_dt;
use self::derive_fields::append_rep_freq;
use self::derive_fields::RateCodeMaster;
use super::recon::ReconKey;
use crate::pre_processor::cust_struct::get_str;
use crate::pre_processor::cust_struct::CustData;
use calamine::{open_workbook, open_workbook_auto, Reader, Xlsx};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::date_from_timestamp;
use rbdate::datevalue_to_naive_date;
use rbdate::DateParser;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

mod cust_struct;
mod derive_fields;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let as_on_date = NaiveDate::parse_from_str(config_param.as_on_date(), "%d-%m-%Y")
        .expect("Cannot parse as_on_date to a valid NaiveDate type.");

    let mut sma_file_hashmap: HashMap<String, String> = HashMap::new();
    
    let input_file = match File::open(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
   
    //Read the cust type file path
    let sma_file = match new_buf_rdr(config_param.sma_file_path()) {
        Ok(sma_file) => sma_file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.sma_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in sma_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.sma_file_path(),
                line_num + 1,
                error
            ),
        };
        let mut sma_file_vec: Vec<&str> = line.split(',').collect();
        let mut data_source_name_to_compare: String = sma_file_vec[1].to_string();
        if data_source_name_to_compare.to_uppercase().trim()
            == config_param.data_source_name.to_uppercase().trim()
        {
            sma_file_hashmap.insert(sma_file_vec[2].to_string(), sma_file_vec[14].to_string());
        }
    }

    //Read customer master file
    let mut cust_data_vec: Vec<CustData> = Vec::new();
    let mut cust_matchcase_vec: Vec<String> = Vec::new();
    let cust_types_reader = fs::read_to_string(config_param.cust_type_file_path())
        .expect("Could not read customer file path");
    for (line_no, line) in cust_types_reader.lines().enumerate() {
        let cust_vec: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let cust_data: CustData = CustData::new(
            &config_param,
            config_param.cust_type_file_path(),
            &cust_vec,
            line_no + 1,
        );
        cust_data_vec.push(cust_data.clone());
        if cust_data.condition.to_string().to_uppercase() == "MATCHCASE"{
            cust_matchcase_vec.push(cust_data.flag_value.to_string());
        }
    }
  
    let mut ref_excel1: Xlsx<_> =
        open_workbook(config_param.ref_file_path_1()).expect("Error opening MIS Desc File.");
    let mut ref_map1: HashMap<String, (String, String)> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            ref_map1.insert(row[0].to_string(), (row[2].to_string(), row[3].to_string()));
        }
    }
    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error opening UBS Rate Code Master File.");
    let mut rate_code_master: HashMap<String, RateCodeMaster> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let rate_code = RateCodeMaster {
                interpretation: row[1].to_string().trim_matches('"').to_uppercase(),
                rate_type: row[2].to_string(),
                rate_flag: row[3].to_string(),
                days_added_to_bus_dt: row[4].to_string(),
                reset_freq: row[5].to_string(),
                reset_month: row[6].to_string(),
                reset_day: row[7].to_string(),
                override_sys_reset_dt: if row[8].to_string().is_empty() {
                    String::from("NA")
                } else {
                    row[8].to_string()
                },
            };
            let key = row[0].to_string();
            rate_code_master.insert(key.trim_matches('"').to_uppercase(), rate_code);
        }
    }
    let mut ref_excel3: Xlsx<_> =
        open_workbook(config_param.ref_file_path_3()).expect("Error opening Ora GL File.");
    let mut ref_map3: HashMap<String, String> = HashMap::new();
    let mut ora_gl_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let mut temp_concat: String = String::new();
            temp_concat.push_str(&row[1].to_string());
            temp_concat.push_str("_");
            temp_concat.push_str(&row[5].to_string());
            ref_map3.insert(row[0].to_string(), temp_concat);

            ora_gl_map.insert(row[0].to_string(), row[1].to_string());
        }
    }
    let mut ref_excel4: Xlsx<_> =
        open_workbook(config_param.ref_file_path_4()).expect("Error opening ALM Master LLG File.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range(&config_param.alm_sheet_name) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
            ia_llg.insert(row[0].to_string(), row[7].to_string());
            balm_llg.insert(row[0].to_string(), row[9].to_string());
        }
    }
    let mut ref_map5: HashMap<String, String> = HashMap::new();
    let mut ref_excel5: Xlsx<_> =
        open_workbook(config_param.ref_file_path_5()).expect("Error opening Ora Prod File.");
    if let Some(Ok(reader)) = ref_excel5.worksheet_range("Sheet1") {
        for row in reader.rows() {
            ref_map5.insert(row[1].to_string(), row[0].to_string());
        }
    }

    let mut ref_excel6 = open_workbook_auto(config_param.ref_file_path_6()).unwrap();
    let mut ref_map6: HashMap<String, Vec<HashMap<NaiveDate, f32>>> = HashMap::new();
    let mut skp_header = 1;
    let mut hdr_date_vec: Vec<NaiveDate> = Vec::new();
    if let Some(Ok(reader)) = ref_excel6.worksheet_range("BM Rates") {
        for row in reader.rows() {
            if skp_header == 1 {
                for row_val in row {
                    if skp_header == 1 {
                        skp_header += 1;
                        continue;
                    }
                    let hdr_date = datevalue_to_naive_date(&row_val.to_string())
                        .expect("Could not convert to NaiveDate");
                    hdr_date_vec.push(hdr_date);
                }
            }
            let mut sprd_vec: Vec<HashMap<NaiveDate, f32>> = Vec::new();
            for i in 1..row.len() {
                let mut sprd_hash: HashMap<NaiveDate, f32> = HashMap::new();
                sprd_hash.insert(
                    hdr_date_vec[i - 1],
                    row[i]
                        .to_string()
                        .parse::<f32>()
                        .expect("Could not parse as f32"),
                );
                sprd_vec.push(sprd_hash.to_owned());
            }
            ref_map6.insert(row[0].to_string(), sprd_vec);
        }
    }

    let mut ref_map7: HashMap<String, String> = HashMap::new();
    let ref_txt7 = match new_buf_rdr(config_param.ref_file_path_7()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_7(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ref_txt7.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref_file_path_7(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(' ').collect();
        ref_map7.insert(fields[0].to_string(), "NA".to_string());
    }

    //read ref8-----------------------------------------------------------------------------------
    let mut mis2_map: HashMap<String, String> = HashMap::new();
    let mut ref_excel8: Xlsx<_> =
        open_workbook(config_param.ref_file_path_8()).expect("Error opening MIS 2 master File.");
    if let Some(Ok(reader)) = ref_excel8.worksheet_range(config_param.mis2_master_sheet_name()) {
        for row in reader.rows().skip(1) {
            mis2_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    //read ref9-----------------------------------------------------------------------------------
    let mut concat_yield_map: HashMap<String, (String, String)> = HashMap::new();
    let mut ref_excel9: Xlsx<_> =
        open_workbook(config_param.ref_file_path_9()).expect("Error opening concat yieldgrp file.");
    if let Some(Ok(reader)) = ref_excel9.worksheet_range(config_param.concat_yieldgrp_sheet_name())
    {
        for row in reader.rows().skip(1) {
            concat_yield_map.insert(row[0].to_string(), (row[1].to_string(), row[2].to_string()));
        }
    }

    //read ref10-------------------------------------------------------------------------------------
    let mut ubs_coa_map: HashMap<String, String> = HashMap::new();
    let mut ref_excel10: Xlsx<_> =
        open_workbook(config_param.ref_file_path_10()).expect("Error opening UBA COA SHEET NAME.");
    if let Some(Ok(reader)) = ref_excel10.worksheet_range(config_param.master_ubs_coa_sheet_name())
    {
        for row in reader.rows().skip(1) {
            ubs_coa_map.insert(row[0].to_string(), row[3].to_string());
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
    let output_file = match File::create(config_param.output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    let rec_output_file = match File::create(config_param.rec_output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let mut recon_writer = BufWriter::new(rec_output_file);
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    let mut writer = BufWriter::new(output_file);
    let mut output_line = String::new();
    let mut weaker_sec_master_file = open_workbook_auto(config_param.weaker_sec_master_path())
        .expect("Unable to open `Weaker_section_master.xlsx`.");
    let mut weaker_master: HashSet<String> = HashSet::new();
    if let Some(Ok(reader)) =
        weaker_sec_master_file.worksheet_range(&config_param.weaker_sec_sheet_name())
    {
        for row in reader.rows() {
            weaker_master.insert(row[0].to_string());
        }
    }

    let mut ews_weaker_master_file = open_workbook_auto(config_param.ews_weaker_master_path())
        .expect("Unable to open `EWS_Weaker_master.xlsx`.");
    let mut ews_weaker_map: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) =
        ews_weaker_master_file.worksheet_range(&config_param.ews_master_sheet_name())
    {
        for row in reader.rows() {
            ews_weaker_map.insert(row[0].to_string(), row[7].to_string());
        }
    }
    //adding header to output
    let header = "cust_no|reference|cust_name|branch_cd|norm_int_rt|acurl_freq|book_dt|val_dt|mat_dt|due_dt|user_def_stats|prod_cd|gl|curr|prin_ost_bal|component|amt_due|amt_setld|cf_amt|spread|compmis1|compmis2|compmis3|old_rt_cd|old_rt_typ|old_benchmark|nxt_reset_dt|last_reset_dt|rt_flag_new|rt_cd_new|division|concat|alm_line|ia_llg|balm_llg|repricing_freq|nxt_repricing_dt|lst_repricing_dt|as_on_dt|int_basis|int_calc_typ|cust_typ|npa_typ|bmid|cntr_party|lcy_amount|raw_benchmark|derived_int_rt|benchmark_rt|spread_val|ff_flag|call_option_date|put_option_date|is_acc_weaker_section|sek_weaker|Frequency|GL_Description|Ratecode|Ratespread|BDP Division|BDP COA|RETAIL/WHOLESALE|PROD_DESC|YLDGRP_AL|Concat2_Point|PSL_Category|LRD_UDF|NRD_UDF|SMA_FLAG\n";
    output_line.push_str(&header);
    let mut ttl_amt = 0.0;
    let mut ttl_amt_due = 0.0;
    let mut ttl_amt_settled = 0.0;
    let mut tot_acc_encntrd: i64 = 0;
    let mut skp_acc: i64 = 0;
    let mut concats: Vec<String> = Vec::new();
    let mut prev_acc_no: String = String::new();
    for (line_no, line) in reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        tot_acc_encntrd += 1;
        let last_reset_dt = fields[27].to_string();
        let frequency = fields[38].to_string();
        let gl_desc = fields[39].to_string();
        let ratecode = fields[23].to_string();
        let ratespread = fields[19].to_string();

        if fields.len() != 41 {
            skp_acc += 1;
            log_error!(
                diag_log,
                "Cannot Process line no: {}. Invalid column count.",
                line_no + 1
            );
            continue;
        }
        for (index, field) in fields.iter().enumerate() {
            output_line.push_str(field.trim_matches('"'));
            output_line.push_str("|");
            if index == 22 {
                break;
            }
        }
        let cf_amt: f64 = fields[18].parse().unwrap_or(0.0);
        let due_amt: f64 = fields[16].parse().unwrap_or(0.0);
        let settled_amt: f64 = fields[17]
            .parse()
            .expect("Cannot parse settled amt as f64.");
        let acc_num = fields[1];
        let compmis1 = fields[20];
        let rate_code = &fields[23].to_uppercase();
        let rate_ip = &fields[25].to_uppercase();
        let next_reset_date = fields[26];
        let mat_date = fields[8];
        let gl = fields[12];
        let val_date =
            NaiveDate::parse_from_str(&fields[7], "%d-%m-%Y").expect("cannot get val date");
        let mut new_rate_flag: &str = "";
        let mut rep_freq: String = String::new();
        let mut new_lrd: String = fields[34].to_string();
        let mut new_nrd: String = fields[35].to_string();
        if new_lrd != "" && new_lrd != "20" {
            new_lrd = NaiveDate::parse_from_str(&new_lrd, "%d-%b-%y")
                .expect("cannot parse lrd")
                .format("%d-%m-%Y")
                .to_string();
        }
        if new_nrd != "" && new_nrd != "20" {
            new_nrd = NaiveDate::parse_from_str(&new_nrd, "%d-%b-%y")
                .expect("cannot parse nrd")
                .format("%d-%m-%Y")
                .to_string();
        }
        let rate_flag: String = fields[24].to_string();
        let mut bmid: String = String::new();

        if rate_flag == "F" && !rate_code.is_empty() {
            rep_freq = append_rep_freq(&rate_code_master, rate_code).to_string();
            if rate_code_master.contains_key(rate_code) {
                bmid = rate_code_master
                    .get(rate_code)
                    .unwrap()
                    .interpretation
                    .to_uppercase()
                    .to_string();
            } else {
                bmid = "FIXED".to_string();
            }
            if bmid != "FIXED" {
                if NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").is_err()
                    && NaiveDate::parse_from_str(&new_lrd, "%d-%m-%Y").is_ok()
                {
                    new_nrd = mat_date.to_string();
                    new_rate_flag = "A";
                } else if NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").is_ok()
                    && NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").unwrap() > as_on_date
                    && NaiveDate::parse_from_str(&new_lrd, "%d-%m-%Y").is_ok()
                {
                    new_rate_flag = "A";
                } else if NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").is_ok()
                    && NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").unwrap() > as_on_date
                    && NaiveDate::parse_from_str(&new_lrd, "%d-%m-%Y").is_err()
                {
                    new_rate_flag = "A";
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                } else if (NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").is_err()
                    && NaiveDate::parse_from_str(&new_lrd, "%d-%m-%Y").is_err())
                    || NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y").unwrap() <= as_on_date
                {
                    new_rate_flag = "V";
                    new_nrd = append_next_rep_dt(
                        &rate_code_master,
                        &rep_freq.to_uppercase(),
                        rate_code,
                        as_on_date,
                        mat_date,
                        new_rate_flag,
                        log,
                    )
                    .format("%d-%m-%Y")
                    .to_string();
                    new_lrd = append_last_rep_date(
                        &rep_freq.to_uppercase(),
                        NaiveDate::parse_from_str(&new_nrd, "%d-%m-%Y")
                            .expect("connot convert new next rep date"),
                        val_date,
                        log,
                    )
                    .to_string();
                } else {
                    new_nrd = mat_date.to_string();
                    new_rate_flag = "F";
                    rep_freq = "".to_string();
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                }
            } else {
                new_nrd = mat_date.to_string();
                new_rate_flag = "F";
                rep_freq = "".to_string();
                new_lrd = val_date.format("%d-%m-%Y").to_string();
            }
        } else if rate_flag == "F" && rate_code.is_empty() {
            bmid = "FIXED".to_string();
            new_nrd = mat_date.to_string();
            new_rate_flag = "F";
            rep_freq = "".to_string();
            new_lrd = val_date.format("%d-%m-%Y").to_string();
        } else if rate_flag == "X" && !rate_ip.is_empty() {
            rep_freq = append_rep_freq(&rate_code_master, rate_ip).to_string();
            if rate_code_master.contains_key(rate_ip) {
                bmid = rate_code_master
                    .get(rate_ip)
                    .unwrap()
                    .interpretation
                    .to_uppercase()
                    .to_string();
            } else {
                bmid = "FIXED".to_string();
            }
            if bmid != "FIXED" {
                if NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").is_err()
                    && NaiveDate::parse_from_str(&last_reset_dt, "%d-%m-%Y").is_ok()
                {
                    new_nrd = mat_date.to_string();
                    new_rate_flag = "F";
                    rep_freq = "".to_string();
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                } else if NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").is_ok()
                    && NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").unwrap() > as_on_date
                    && NaiveDate::parse_from_str(&last_reset_dt, "%d-%m-%Y").is_ok()
                {
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                    new_nrd = next_reset_date.to_string();
                    new_rate_flag = "A";
                } else if NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").is_ok()
                    && NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").unwrap() > as_on_date
                    && NaiveDate::parse_from_str(&last_reset_dt, "%d-%m-%Y").is_err()
                {
                    new_rate_flag = "A";
                    new_nrd = next_reset_date.to_string();
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                } else if NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").is_err()
                    || NaiveDate::parse_from_str(&next_reset_date, "%d-%m-%Y").unwrap()
                        <= as_on_date
                {
                    bmid = "FIXED".to_string();
                    new_nrd = mat_date.to_string();
                    new_rate_flag = "F";
                    rep_freq = "".to_string();
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                } else {
                    new_nrd = mat_date.to_string();
                    new_rate_flag = "F";
                    rep_freq = "".to_string();
                    new_lrd = val_date.format("%d-%m-%Y").to_string();
                }
            } else {
                new_nrd = mat_date.to_string();
                new_rate_flag = "F";
                rep_freq = "".to_string();
                new_lrd = val_date.format("%d-%m-%Y").to_string();
            }
        } else if rate_flag == "X" && rate_ip.is_empty() {
            bmid = "FIXED".to_string();
            new_nrd = mat_date.to_string();
            new_rate_flag = "F";
            rep_freq = "".to_string();
            new_lrd = val_date.format("%d-%m-%Y").to_string();
        }
        if new_rate_flag == "F" {
            bmid = "FIXED".to_owned();
        }
        output_line.push_str(rate_code);
        output_line.push_str("|");
        output_line.push_str(fields[24]);
        output_line.push_str("|");
        output_line.push_str(rate_ip);
        output_line.push_str("|");
        output_line.push_str(next_reset_date);
        output_line.push_str("|");
        output_line.push_str(fields[27]);
        output_line.push_str("|");
        output_line.push_str(new_rate_flag);
        output_line.push_str("|");
        output_line.push_str(rate_code);
        output_line.push_str("|");

        let default_tuple = ("NONE".to_string(), "NA".to_string());
        let (division, bdp_div) = match ref_map1.get(compmis1) {
            Some(val) => val,
            None => &default_tuple,
        };
        output_line.push_str(division);
        output_line.push_str("|");

        let mut ora_mis1 = String::new();
        ora_mis1.push('1');
        ora_mis1.push_str(fields[20]);

        let ora_prod = ref_map5
            .entry(fields[11].to_string())
            .or_insert_with(|| "".to_string());

        let mut concat4_point = "".to_string();
        concat4_point.push_str(&ora_mis1);
        concat4_point.push('_');
        concat4_point.push_str(ora_prod);
        concat4_point.push('_');
        match ref_map3.get(gl) {
            Some(val) => concat4_point.push_str(val),
            None => concat4_point.push_str(""),
        };
        concats.push(append_alm_ia_balm_line(
            &mut output_line,
            &ora_mis1,
            &ref_map3,
            &ora_prod,
            &alm_line,
            &ia_llg,
            &balm_llg,
            gl,
            log,
            acc_num,
        ));
        output_line.push_str(&rep_freq[..]);
        output_line.push_str("|");
        if new_nrd == NaiveDate::from_ymd(1900, 1, 1).to_string() {
            output_line.push_str("|");
        } else {
            output_line.push_str(&new_nrd);
            output_line.push_str("|");
        }
        output_line.push_str(&new_lrd);
        output_line.push_str("|");
        append_as_on_date(&mut output_line, as_on_date);
        let customer_name1 = get_str(config_param.input_file_path(), &fields, 2, line_no);
        let customer_type = get_customer_type(customer_name1, cust_data_vec.clone(), cust_matchcase_vec.clone());
        output_line.push_str(&customer_type);
        output_line.push_str("||");
        output_line.push_str(&bmid.trim());
        output_line.push_str("|");
        output_line.push_str(fields[36]);
        output_line.push_str("|");
        output_line.push_str(fields[37]);
        output_line.push_str("|");
        let raw_benchmark = if fields[23].trim() != "" {
            fields[23]
        } else if fields[25].trim() != "" {
            fields[25]
        } else {
            ""
        };
        output_line.push_str(raw_benchmark);
        output_line.push_str("|");
        //Passthrough Derived Interest Rate
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let mut der_ir: f32 = 0.0;
        if !ref_map7.contains_key(&fields[10].to_string()) {
            if fields[4] != "" {
                der_ir = fields[4]
                    .parse::<f32>()
                    .expect("Could not parse derived int_rate");
            }
        }
        output_line.push_str(&der_ir.to_string());
        output_line.push_str("|");
        //Passthrough Benchmark Rates
        let mut bnchmrk_date = date_from_timestamp(0);
        if new_lrd != "" {
            bnchmrk_date = date_parser.parse(&new_lrd.to_string());
        }
        let sprd_bnchmrk = bmid.to_string();
        let mut bnchmrk_rate = 0.0;
        let mut prev_dt = date_from_timestamp(0);
        if ref_map6.contains_key(&sprd_bnchmrk) {
            for bnchmrk_val in ref_map6
                .get(&sprd_bnchmrk)
                .expect("Could not find spread benchmark")
            {
                for (key, val) in bnchmrk_val {
                    if key <= &bnchmrk_date && prev_dt <= *key {
                        prev_dt = *key;
                        bnchmrk_rate = *val;
                    }
                }
            }
        }
        output_line.push_str(&bnchmrk_rate.to_string());
        output_line.push_str("|");

        //Passthrough Spread
        output_line.push_str(&(&der_ir - &bnchmrk_rate).to_string());
        output_line.push_str("|");

        //Passthrough Fully Floating flag
        let mut ff_flag: String = String::new();
        let mut new_nrd_inp = "".to_string();
        if fields[35].to_string() != "" {
            new_nrd_inp = NaiveDate::parse_from_str(&fields[35].to_string(), "%d-%b-%y")
                .expect("cannot parse next repricing date")
                .format("%d-%m-%Y")
                .to_string();
        }
        if bmid.to_uppercase() == "FIXED" {
            ff_flag = "NA".to_string();
        } else {
            if new_lrd == "" && new_nrd == "" {
                ff_flag = "YES".to_string();
            } else {
                if new_lrd != "" {
                    new_nrd = append_next_rep_date(
                        &rep_freq.to_uppercase(),
                        NaiveDate::parse_from_str(&new_lrd, "%d-%m-%Y")
                            .expect("cannot convert new next rep date"),
                        as_on_date,
                        log,
                    )
                    .to_string();
                    if new_nrd != new_nrd_inp {
                        ff_flag = "YES".to_string();
                    } else {
                        ff_flag = "NO".to_string();
                    }
                } else {
                    ff_flag = "NA".to_string();
                }
            }
        }
        output_line.push_str(&ff_flag);
        output_line.push('|');
        //Validation for call option date having details along with date value: "DD-MMM-YYYY".
        let call_dt_raw_val = &fields[31].chars().take(11).collect::<String>();
        let call_option_date = match NaiveDate::parse_from_str(call_dt_raw_val, "%d-%b-%Y") {
            Ok(date) => date.format("%d-%m-%Y").to_string(),
            Err(_e) => NaiveDate::parse_from_str(&fields[31], "%d-%m-%Y")
                .unwrap_or(NaiveDate::from_ymd(2999, 1, 31))
                .format("%d-%m-%Y")
                .to_string(),
        };
        output_line.push_str(&call_option_date);
        output_line.push('|');
        //Validation for put option date having details along with date value: "DD-MMM-YYYY".
        let put_dt_raw_val = &fields[33].chars().take(11).collect::<String>();
        let put_option_date = match NaiveDate::parse_from_str(put_dt_raw_val, "%d-%b-%Y") {
            Ok(date) => date.format("%d-%m-%Y").to_string(),
            Err(_e) => NaiveDate::parse_from_str(&fields[33], "%d-%m-%Y")
                .unwrap_or(NaiveDate::from_ymd(2999, 1, 31))
                .format("%d-%m-%Y")
                .to_string(),
        };
        output_line.push_str(&put_option_date);
        let is_acc_weaker = if weaker_master.contains(&fields[1].to_string()) {
            "Y"
        } else {
            "N"
        };
        let ews_weaker_value = if (match ews_weaker_map.get(&fields[1].to_string()) {
            Some(val) => val,
            None => "Others",
        }) == "SEK_WK"
        {
            "SEK-WEAKER"
        } else {
            "Others"
        };
        output_line.push('|');
        output_line.push_str(is_acc_weaker);
        output_line.push('|');
        output_line.push_str(ews_weaker_value);
        ttl_amt += cf_amt;
        ttl_amt_due += due_amt;
        ttl_amt_settled += settled_amt;
        if prev_acc_no != fields[1] {
            let recon_key = ReconKey::new(
                // currency
                fields[13].to_string(),
                "GL".to_string(),
                //gl code
                gl.to_string(),
            );
            let lcy_amt: f64 = fields[14].parse().unwrap_or(0.0);
            recon_map
                .entry(recon_key)
                .and_modify(|amt| *amt += lcy_amt)
                .or_insert(lcy_amt);
            prev_acc_no = fields[1].to_string();
            log_debug!(
                log,
                "Account: `{}`, gl_code: `{}`, amount: `{}`.",
                fields[1],
                gl,
                lcy_amt
            );
        }
        //new output:
        output_line.push('|');
        output_line.push_str(&frequency);
        output_line.push('|');
        output_line.push_str(&gl_desc);
        output_line.push('|');
        output_line.push_str(&ratecode);
        output_line.push('|');
        output_line.push_str(&ratespread);
        output_line.push('|');
        output_line.push_str(&bdp_div);
        output_line.push('|');

        let new_nature_acc = match ora_gl_map.get(gl) {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };
        let concat2 = format!("{}_{}", ora_prod, new_nature_acc);

        let bdp_coa = match ubs_coa_map.get(&concat2) {
            Some(val) => val,
            None => "NA",
        };
        output_line.push_str(bdp_coa);
        output_line.push('|');

        let (retail, yieldgrp) = match concat_yield_map.get(&concat4_point) {
            Some(val) => val,
            None => &default_tuple,
        };
        output_line.push_str(&retail);
        output_line.push('|');
        output_line.push_str(fields[40]);
        output_line.push('|');
        output_line.push_str(&yieldgrp);
        output_line.push('|');
        output_line.push_str(&concat2);
        output_line.push('|');

        let psl_category = match mis2_map.get(fields[21]) {
            Some(val) => val,
            None => "NA",
        };
        let mut sma_flag: String = "P".to_string();
        if sma_file_hashmap.contains_key(&fields[1].to_string()) {
            sma_flag = sma_file_hashmap
                .get(&fields[1].to_string())
                .expect("failed to get the required contents")
                .to_string()
        }
        output_line.push_str(&psl_category);
        output_line.push('|');
        output_line.push_str(fields[27]);
        output_line.push('|');
        output_line.push_str(fields[26]);
        output_line.push('|');
        output_line.push_str(&sma_flag);
        output_line.push_str("\n");

        log_debug!(diag_log, "Processed line no: {}", line_no + 1);
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    let start_write_timer = SystemTime::now();
    log_debug!(diag_log, "Total cf amount: {}", ttl_amt);
    log_debug!(diag_log, "Total due amount: {}", ttl_amt_due);
    log_debug!(diag_log, "Total settled amount: {}", ttl_amt_settled);
    match writer.write_all(output_line.as_bytes()) {
        Ok(_val) => println!("Successfully processed all accounts"),
        Err(error) => {
            panic!("Cannot pre process the input file: {:?}", error);
        }
    }
    let mut recon_output_line = String::new();
    for (key, value) in recon_map {
        let op = format!(
            "{}|{}|{}|{}|{}|{}",
            config_param.as_on_date(),
            "INP001_LOANS_FC/INP003_FC_LOANS04",
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
            panic!("Cannot pre process the input file: {:?}", error);
        }
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
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing pre-processed output and reconcilation files.");
    debug!(
        diag_log,
        "Writing Records and Reconcilation File, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skp_acc,
        skp_acc,
        ttl_amt,
        ttl_amt,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());
}

fn get_customer_type(cust_desc: String, text_desc_data1: Vec<CustData>, cust_matchcase_vec: Vec<String>) -> String {
    let mut text_desc_flag = "NON-FI".to_string();
    for text_desc_data in text_desc_data1 {
        text_desc_flag = match text_desc_data.condition.to_uppercase().as_str() {
            "MATCHCASE" => {
                if cust_desc
                    .to_uppercase()
                    .to_string()
                    .trim_end()
                    .trim_start()
                    .eq(&text_desc_data.flag_value.to_uppercase())
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "NON-FI".to_string()
                }
            }
         
            "START" => {
                if cust_desc
                    .to_uppercase()
                    .starts_with(&text_desc_data.flag_value.to_uppercase()) && !cust_matchcase_vec.contains(&cust_desc)
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "NON-FI".to_string()
                }
            }
            "END" => {
                if cust_desc
                    .to_uppercase()
                    .ends_with(&text_desc_data.flag_value.to_uppercase()) && !cust_matchcase_vec.contains(&cust_desc)
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "NON-FI".to_string()
                }
            }
            "BETWEEN" => {
                if cust_desc
                    .to_uppercase()
                    .contains(&text_desc_data.flag_value.to_uppercase()) && !cust_matchcase_vec.contains(&cust_desc)
                {
                    text_desc_data.txt_desc_flag
                } else {
                    "NON-FI".to_string()
                }
            }
           
            _ => "NON-FI".to_string(),
        };
        if text_desc_flag != "NON-FI" {
            break;
        }
    }
    text_desc_flag
}
