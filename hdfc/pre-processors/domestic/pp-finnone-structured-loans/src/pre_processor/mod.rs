use self::derive_fields::*;
use self::recon::ReconKey;
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
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::SystemTime;

static DEFAULT_FLOAT: f64 = 0.0;
mod derive_fields;
mod recon;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
    let as_on_date = NaiveDate::parse_from_str(config_param.as_on_date(), "%d-%m-%Y")
        .expect("Cannot parse as_on_date to a valid NaiveDate type.");
    let input_file = match File::open(config_param.input_file_path()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let rec_output_file = match File::create(config_param.rec_output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };

    //Reading sma file
    let data_src_name = config_param.data_src_name();
    let mut sma_map: HashMap<String, String> = HashMap::new();
    let sma_file_reader =
        fs::read_to_string(config_param.sma_file_path()).expect("Could not read sma file");
    for (line_no, line) in sma_file_reader.lines().enumerate() {
        let sma_data_vec: Vec<&str> = line.split(',').collect::<Vec<&str>>();

        let data_src_name = get_str(config_param.sma_file_path(), &sma_data_vec, 1, line_no);
        let acc_id = get_str(config_param.sma_file_path(), &sma_data_vec, 2, line_no);
        let sma_stamping = get_str(config_param.input_file_path(), &sma_data_vec, 14, line_no);
        if data_src_name.to_uppercase() == data_src_name.trim().to_uppercase() {
            sma_map.insert(acc_id, sma_stamping);
        }
    }
    let mut recon_writer = BufWriter::new(rec_output_file);
    let mut recon_map: HashMap<ReconKey, f64> = HashMap::new();
    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.ref_file_path_1()).unwrap();
    let mut ref_map1: HashMap<String, AlmMaster> = HashMap::new();
    let mut ia_line_map: HashMap<String, String> = HashMap::new();
    let mut division: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range(&config_param.alm_master_sheet_name) {
        for row in reader.rows() {
            let alm_master = AlmMaster {
                alm: row[2].to_string(),
                coa: row[3].to_string(),
                al_line: row[5].to_string(),
                balm_l2: if let Some(val) = row.get(6) {
                    val.to_string()
                } else {
                    String::from("NONE")
                },
            };
            let scheme_id = match row.get(7) {
                Some(val) => val.to_string(),
                None => "NA".to_string(),
            };
            ref_map1.insert(scheme_id.to_string(), alm_master);
            ia_line_map.insert(scheme_id.to_string(), row[5].to_string());
            division.insert(row[0].to_string(), row[4].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2()).unwrap();
    let mut ref_map2: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            let costcenter = row[1].to_string();
            ref_map2.insert(row[0].to_string(), costcenter);
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3()).unwrap();
    let mut ref_map3: HashMap<String, RateCodeMaster> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range("Finone_Rate_Code_Master") {
        for row in reader.rows().skip(2) {
            let rate_code = RateCodeMaster {
                interpretation: row[1].to_string().trim_matches('"').to_uppercase(),
                rate_flag: row[2].to_string(),
                days_added_to_bus_dt: row[3].to_string(),
                reset_freq: row[4].to_string(),
                reset_month: row[5].to_string(),
                reset_day: row[6].to_string(),
                override_sys_reset_dt: row[7].to_string(),
            };
            ref_map3.insert(
                row[0].to_string().trim_matches('"').to_uppercase(),
                rate_code,
            );
        }
    }

    let mut ref_excel5 = open_workbook_auto(config_param.ref_file_path_5()).unwrap();
    let mut ref_map5: HashMap<String, Vec<HashMap<NaiveDate, f32>>> = HashMap::new();
    let mut skp_header = 1;
    let mut hdr_date_vec: Vec<NaiveDate> = Vec::new();
    if let Some(Ok(reader)) = ref_excel5.worksheet_range("BM Rates") {
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
            ref_map5.insert(row[0].to_string(), sprd_vec);
        }
    }

    let mut asset_class: HashMap<String, String> = HashMap::new();
    let ref_txt1 = match new_buf_rdr(config_param.ref_file_path_4()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_4(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ref_txt1.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                error!(
                    log,
                    "Unable to read file {} at line number: {} : {}",
                    config_param.ref_file_path_4(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = line.split(',').collect();
        asset_class.insert(fields[1].to_string(), fields[6].to_string());
    }

    let mut ref_map6: HashMap<String, String> = HashMap::new();
    let ref_txt6 = match new_buf_rdr(config_param.ref_file_path_6()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_6(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in ref_txt6.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => {
                error!(
                    log,
                    "Unable to read file {} at line number: {} : {}",
                    config_param.ref_file_path_6(),
                    line_num + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = line.split(' ').collect();
        ref_map6.insert(fields[0].to_string(), "NA".to_string());
    }

    let output_file = match File::create(config_param.output_file_path()) {
        Ok(output_file) => output_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut output_line = String::new();
    let mut total_amt: f64 = 0.0;

    //adding header to output
    let header = "account_number|accrual_basis|accrued_interest|branch|curr_code|current_bal|due_date|interest_pay_freq|intt_rate|product_code|mat_date|original_balance|orig_term|org_date|emi|payment_freq|payment_type|rate_flag|repricing_index|dpd|customer_name|scheme_id|psl|npa|inst_st_dt|weaker|current_book_balance|first_inst_date|inst_num|num_inst_paid|last_inst_date|indv_corp_flag|customer_type|gr_dr|gr_cr|re_dr|re_cr|is_dr|is_cr|ui_dr|ui_cr|asset_class_id|customer_id|prod_type|is_ofs_gl|gr_ofs_gl|re_ofs_gl|ui_ofs_gl|int_amt|prin_amt|cf_dt|as_on_date|final_int_rate|cost_centre|alm_line|coa|division|rep_freq|next_repricing_date|last_repricing_date|asset_class|al_line|balm_l2|bmid|ia_line|weaker_code|derived_int_rate|bnchmrk_rate|spread|fully_floating_flag|sma_flag\n";
    output_line.push_str(&header);

    let end_read_timer = SystemTime::now();
    let total_duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total read duration.");
    info!(
        diag_log,
        "Total Input Read Duration(incl. reference read): {:?}", total_duration
    );
    let start_process_time = SystemTime::now();
    let mut tot_acc_encntrd = 0;
    let mut skipp_acc = 0;
    let mut concats: Vec<String> = Vec::new();
    let mut prev_acc_no: String = String::new();
    for (line_no, line) in reader.lines().enumerate() {
        tot_acc_encntrd += 1;
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                error!(
                    log,
                    "Unable to read file {} at line number: {} : {}",
                    config_param.input_file_path(),
                    line_no + 1,
                    error
                );
                continue;
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        if line_no == 0 && fields[2].parse::<f64>().is_err() {
            skipp_acc += 1;
            continue;
        }
        if fields.len() != 51 {
            log_error!(diag_log, "Cannot Process line no: {}", line_no + 1);
            skipp_acc += 1;
            continue;
        }
        for field in &fields {
            output_line.push_str(field);
            output_line.push_str("|");
        }
        let dpd_opt: Option<f64> = fields[19].parse().ok();
        let mut dpd: f64 = 0.0;
        if dpd_opt.is_none() {
            log_warn!(log, "Cannot parse DPD fields as double: {}", fields[0]);
        } else {
            dpd = dpd_opt.unwrap_or(0.0);
        }
        let int_rate = fields[8];
        let psl = fields[22];
        let prod_code = fields[9];
        let reprice_index = &fields[18].to_uppercase();
        let maturity_date = fields[10];
        let rate_flag = fields[17];
        let scheme_id = fields[21].trim_matches('"');
        let weaker_code = fields[25].trim_matches('"');
        let bmid = match ref_map3.get(reprice_index.trim_matches('"')) {
            Some(val) => {
                if val.interpretation == ""
                    || val.reset_freq == ""
                    || fields[17].to_uppercase().to_string().trim_matches('"') == "FIXED"
                {
                    "FIXED".to_string()
                } else {
                    val.interpretation.to_string()
                }
            }
            None => "FIXED".to_string(),
        };
        let ccy: String = fields[4].to_string().to_string();
        if prev_acc_no != fields[0] {
            prev_acc_no = fields[0].to_string();
            let is_recon_key = ReconKey::new(
                ccy.to_string(),
                "IS".to_string(),
                fields[44].trim().to_string(),
            );
            let gr_recon_key = ReconKey::new(
                ccy.to_string(),
                "GR".to_string(),
                fields[45].trim().to_string(),
            );
            let re_recon_key = ReconKey::new(
                ccy.to_string(),
                "RE".to_string(),
                fields[46].trim().to_string(),
            );
            let ui_recon_key = ReconKey::new(ccy, "UI".to_string(), fields[47].trim().to_string());
            let net_ui = fields[39].parse::<f64>().unwrap_or(DEFAULT_FLOAT)
                - fields[40].parse::<f64>().unwrap_or(DEFAULT_FLOAT);
            let net_gr = fields[33].parse::<f64>().unwrap_or(DEFAULT_FLOAT)
                - fields[34].parse::<f64>().unwrap_or(DEFAULT_FLOAT);
            let net_re = fields[35].parse::<f64>().unwrap_or(DEFAULT_FLOAT)
                - fields[36].parse::<f64>().unwrap_or(DEFAULT_FLOAT);
            let net_is = fields[37].parse::<f64>().unwrap_or(DEFAULT_FLOAT)
                - fields[38].parse::<f64>().unwrap_or(DEFAULT_FLOAT);

            recon_map
                .entry(is_recon_key)
                .and_modify(|amt| *amt += net_is)
                .or_insert(net_is);
            recon_map
                .entry(gr_recon_key)
                .and_modify(|amt| *amt += net_gr)
                .or_insert(net_gr);
            recon_map
                .entry(re_recon_key)
                .and_modify(|amt| *amt += net_re)
                .or_insert(net_re);
            recon_map
                .entry(ui_recon_key)
                .and_modify(|amt| *amt += net_ui)
                .or_insert(net_ui);
        }
        let gl = fields[44].trim().to_string()
            + "#"
            + &fields[45].trim().to_string()
            + "#"
            + &fields[46].trim().to_string()
            + "#"
            + &fields[47].trim().to_string();
        append_as_on_date(&mut output_line, as_on_date);
        append_final_interest_rate(&mut output_line, dpd, int_rate);
        append_cost_centre(&mut output_line, &ref_map2, psl);
        let mut rep_date = NaiveDate::from_ymd(1970, 01, 01);
        concats.push(append_alm_line(
            &mut output_line,
            &ref_map1,
            scheme_id,
            log,
            fields[0],
            &gl,
        ));
        append_coa(&mut output_line, &ref_map1, prod_code);
        apppend_division(&mut output_line, &division, prod_code);
        if rate_flag != "Fixed" {
            let rep_freq: &str = &append_rep_freq(&mut output_line, &ref_map3, reprice_index);
            let next_rep_date = append_next_rep_dt(
                &mut output_line,
                &ref_map3,
                rep_freq,
                reprice_index,
                as_on_date,
                maturity_date,
                rate_flag,
            );
            if rep_freq != "NONE" {
                rep_date = append_last_rep_date(&mut output_line, rep_freq, next_rep_date);
                output_line.push_str(&format!("{}", rep_date.format("%d-%m-%Y")));
                output_line.push_str("|");
            } else {
                output_line.push_str("|");
            }
        } else {
            output_line.push_str("|31-12-2099||");
        }
        let npa_flg = &asset_class
            .entry(fields[0].to_string())
            .or_insert_with(|| "P".to_string());
        output_line.push_str(&npa_flg);
        append_al_line(&mut output_line, &ref_map1, scheme_id);
        append_balm_l2(&mut output_line, &ref_map1, scheme_id);
        output_line.push_str(&bmid);

        let ia_line = match ia_line_map.get(scheme_id) {
            Some(val) => {
                if val.is_empty() {
                    "NONE".to_string()
                } else {
                    val.to_string()
                }
            }
            None => "NONE".to_string(),
        };
        output_line.push_str("|");
        output_line.push_str(&ia_line);
        output_line.push_str("|");
        output_line.push_str(&weaker_code);
        output_line.push_str("|");
        //Passthrough Derived Interest Rate
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let mut der_ir: f32 = 0.0;
        if !ref_map6.contains_key(&npa_flg.to_string()) {
            der_ir = fields[8]
                .parse::<f32>()
                .expect("Could not parse derived int_rate");
        }
        output_line.push_str(&der_ir.to_string());
        output_line.push_str("|");

        //Passthrough Benchmark Rates
        let rep_date = &format!("{}", rep_date.format("%d-%m-%Y"));
        let bnchmrk_date = date_parser.parse(&rep_date.to_string());
        let sprd_bnchmrk = bmid.to_string();
        let mut bnchmrk_rate = 0.0;
        let mut prev_dt = date_from_timestamp(-3456789000);
        if ref_map5.contains_key(&sprd_bnchmrk) {
            for bnchmrk_val in ref_map5
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
        if rate_flag.to_uppercase() == "FIXED" {
            ff_flag = "NA".to_string();
        } else {
            if ref_map3.contains_key(fields[18].to_string().to_uppercase().trim_matches('"')) {
                let ff_struct = ref_map3
                    .get(fields[18].to_string().to_uppercase().trim_matches('"'))
                    .unwrap();
                if ff_struct.days_added_to_bus_dt == "" && ff_struct.reset_month == "" {
                    ff_flag = "YES".to_string();
                } else {
                    ff_flag = "NO".to_string();
                }
            } else {
                ff_flag = "NA".to_string();
            }
        }
        output_line.push_str(&ff_flag);
        output_line.push_str("|");
        let data_src = config_param.data_src_name().trim().to_uppercase();
        let def_map: HashMap<String, String> = HashMap::new();
        let acc_id = get_str(config_param.input_file_path(), &fields, 0, line_no);
        let sma_flag = sma_map.get(&acc_id).unwrap_or(&"P".to_string()).to_string();
        output_line.push_str(&sma_flag);
        output_line.push_str("\n");
        log_debug!(diag_log, "Processed line no: {}", line_no + 1);
        let amt: f64 = fields[26].parse().unwrap_or(0.0);
        total_amt += amt;
    }
    log_info!(log, "Total Outstanding in input: {}", total_amt);
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
            config_param.as_on_date(),
            "INP005_ALM_ME",
            key.gl_type,
            key.gl_code,
            value,
            key.currency,
        );
        recon_output_line.push_str(&op[..]);
        recon_output_line.push_str("\n");
    }
    match recon_writer.write_all(recon_output_line.as_bytes()) {
        Ok(_val) => {}
        Err(error) => {
            panic!("Cannot generate reconciliation report file: {:?}", error);
        }
    }

    match writer.write_all(output_line.as_bytes()) {
        Ok(_val) => println!("Successfully processed all accounts"),
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

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - skipp_acc,
        skipp_acc,
        total_amt,
        total_amt,
        0,
    );
    println!("{}", health_report.display());
    health_report.gen_health_rpt(&config_param.output_file_path());

    let end_writer_time = SystemTime::now();
    let duration = end_writer_time
        .duration_since(start_writer_time)
        .expect("Could not calculate total write process duration.");
    info!(diag_log, "Write Process Total Duration: {:?}.", duration);
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
