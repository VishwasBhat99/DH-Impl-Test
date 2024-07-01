use self::benchmark::Benchmark;
use self::derive_fields::get_op_line;
use self::derive_fields::RateCodeMaster;
use self::reconcilation::ReconKey;
use calamine::{open_workbook, open_workbook_auto, Reader, Xlsx};
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::datevalue_to_naive_date;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};

mod benchmark;
mod derive_fields;
mod output_lines;
mod reconcilation;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();

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

    let mut ref_excel1: Xlsx<_> = open_workbook(config_param.ref_file_path_1())
        .expect("Errow while opening `MIS1_Desc.xlsx` file.");
    let mut div: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel1.worksheet_range("Sheet1") {
        for row in reader.rows() {
            div.insert(row[1].to_string(), row[2].to_string());
        }
    }

    let mut ref_excel2: Xlsx<_> = open_workbook(config_param.ref_file_path_2())
        .expect("Error while opening `Ora_GL.xlsx` file.");
    let mut t_ora_prod: HashMap<String, String> = HashMap::new();
    let mut t_ora_gl: HashMap<String, String> = HashMap::new();
    let mut t_ora_cat: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel2.worksheet_range("Sheet1") {
        for row in reader.rows() {
            t_ora_prod.insert(row[0].to_string(), row[4].to_string());
            t_ora_gl.insert(row[0].to_string(), row[1].to_string());
            t_ora_cat.insert(row[0].to_string(), row[5].to_string());
        }
    }

    let mut ref_excel3: Xlsx<_> = open_workbook(config_param.ref_file_path_3())
        .expect("Error while opening `ALM_Line_Master.xlsx` file.");
    let mut alm_line: HashMap<String, String> = HashMap::new();
    let mut ia_llg: HashMap<String, String> = HashMap::new();
    let mut balm_llg: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
            ia_llg.insert(row[0].to_string(), row[7].to_string());
            balm_llg.insert(row[0].to_string(), row[9].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `FWCostCenter_OD.xlsx` file.");
    let mut cost_center: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("Sheet1") {
        for row in reader.rows() {
            cost_center.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut ref_excel6: Xlsx<_> = open_workbook(config_param.ref_file_path_6())
        .expect("Error while opening Rate Code Master File.");
    let mut rt_cd: HashMap<String, RateCodeMaster> = HashMap::new();
    let mut override_sys_reset_dt: String;
    if let Some(Ok(reader)) = ref_excel6.worksheet_range("Sheet1") {
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
            rt_cd.insert(key.trim_matches('"').to_uppercase(), rate_code);
        }
    }

    let mut ref_excel7: Xlsx<_> = open_workbook(config_param.ref_file_path_7())
        .expect("Error while opening `Master Currency` file.");
    let mut currency: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel7.worksheet_range("Sheet1") {
        for row in reader.rows() {
            currency.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut bm_id_map: HashMap<String, Benchmark> = HashMap::new();
    let bm_reader = match new_buf_rdr(config_param.ref_file_path_8()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_8(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in bm_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref_file_path_8(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('~').collect();
        benchmark::get_benchmark(&mut bm_id_map, fields[3], fields[19], fields[21]);
    }

    let mut asset_class: HashMap<String, String> = HashMap::new();
    let ref_txt1 = match new_buf_rdr(config_param.ref_file_path_5()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_5(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ref_txt1.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref_file_path_5(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(',').collect();
        asset_class.insert(fields[1].to_string(), fields[6].to_string());
    }
    let end_read_timer = SystemTime::now();
    let total_duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration.");
    debug!(
        diag_log,
        "Reading Reference Files Total Duration: {:?}", total_duration
    );
    let mut ref_excel9 = open_workbook_auto(config_param.ref_file_path_9()).unwrap();
    let mut sprd_map: HashMap<String, Vec<HashMap<NaiveDate, f32>>> = HashMap::new();
    let mut skp_header = 1;
    let mut hdr_date_vec: Vec<NaiveDate> = Vec::new();
    if let Some(Ok(reader)) = ref_excel9.worksheet_range("BM Rates") {
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
            sprd_map.insert(row[0].to_string(), sprd_vec);
        }
    }

    let mut npa_map: HashMap<String, String> = HashMap::new();
    let ref_txt10 = match new_buf_rdr(config_param.ref_file_path_10()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ref_file_path_10(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in ref_txt10.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ref_file_path_10(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split(' ').collect();
        npa_map.insert(fields[0].to_string(), "NA".to_string());
    }
    let mut od_study_map: HashMap<String, Vec<f64>> = HashMap::new();
    let mut od_study_master: Xlsx<_> = open_workbook(config_param.od_study_master())
        .expect("Error while opening `od_study_master` file.");
    if let Some(Ok(reader)) =
        od_study_master.worksheet_range(&config_param.od_study_master_sheet_name())
    {
        for row in reader.rows() {
            od_study_map.insert(
                row[0].to_string(),
                vec![
                    row[1].to_string().parse().unwrap_or(0.0000),
                    row[2].to_string().parse().unwrap_or(0.0000),
                    row[3].to_string().parse().unwrap_or(0.0000),
                ],
            );
        }
    }
    let start_derive_timer = SystemTime::now();
    let total_field_derivation_duration: Duration = Duration::new(0, 0);
    let reader = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let master_reader = match new_buf_rdr(config_param.master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut master_fields: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in master_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let master_record: Vec<&str> = line.split("~#~").collect();
        if master_record.len() != 8 {
            continue;
        }
        master_fields.insert(master_record[0].to_string(), master_record[7].to_string());
    }

    let mut output_line = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd = DEFAULT_INT;
    let mut lines_fld = DEFAULT_INT;
    let mut tot_amt = DEFAULT_FLOAT;
    let mut concats: Vec<String> = Vec::new();
    //adding header to output
    let header = "cod_acc_no|cod_cc_brn|cod_prod|bal_book|bal_book_lcy|amt_od_lmt|amt_od_lmt_lcy|cod_cust|cod_acc_title|dt_open_acc|cod_int_accr_bas|freq_int_accr|dt_acc_close|cod_collat_id|collat_desc|as_of_dt|cost_cntr|gl_acc_no|rt_flg|inst|crnt_book_bal|acrl_basis|int_rt|div|alm_line|ia_llg|balm_llg|mis1|npa_flg|benchmark|rep_freq|nxt_rep_dt|lst_rep_dt|cust_typ|country|bm_id_lookup|alm_concat|mis2_code|der_int_rate|bnchmrk_rate|spread|fully_floating_flg|B1|B2|B3|flg_frequency|dat_start_frq|dat_frq_last_reset|dat_frq_next_reset|rat_var_penality|sma_flag\n";
    output_line.push_str(&header);
    for (line_num, lines) in reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };
        let mut fields: Vec<&str> = line.split("~#~").collect();
        let acc_id = get_str(config_param.input_file_path(), &fields, 0, line_num);
        let sma_flag = sma_map.get(&acc_id).unwrap_or(&"P".to_string()).to_string();
        if line_num == 0 && fields[1].parse::<i64>().is_err() {
            lines_fld += 1;
            continue;
        }
        fields[0] = fields[0].trim();
        let input_acc_bmid = fields[31].trim().to_string();
        let bm_struct = match bm_id_map.get(&fields[0].to_string()) {
            Some(value) => value.to_owned(),
            None => Benchmark {
                amt: DEFAULT_FLOAT,
                bm_id: input_acc_bmid,
            },
        };

        let mut mis2_code = "".to_string();
        if master_fields.contains_key(fields[0]) {
            mis2_code = master_fields
                .get(fields[0])
                .unwrap_or(&"".to_string())
                .to_string();
        }
        let bm_id = bm_struct.bm_id.to_string();
        log_debug!(log, "`Benchmark for: `{}` is `{}`.", fields[0], bm_id);

        let bm_id_lookup = match rt_cd.get(&bm_id.trim_matches('"').to_uppercase()) {
            Some(rt_cd) => rt_cd
                .interpretation
                .to_string()
                .trim_matches('"')
                .to_uppercase(),
            None => {
                log_debug!(
                    log,
                    "`Benchmark for: `{}` not found in Rate Code Master.",
                    bm_id
                );
                "FIXED".to_string()
            }
        };
        let val_date = NaiveDate::parse_from_str(&fields[10].to_string(), "%d-%b-%Y")
            .expect("cannot get val date");
        let ccy: &str = currency
            .entry(fields[3].to_string())
            .or_insert_with(|| "OTH".to_string());
        if rt_cd.contains_key(&bm_id.trim_matches('"').to_uppercase()) {
            override_sys_reset_dt = rt_cd
                .get(&bm_id.trim_matches('"').to_uppercase())
                .unwrap()
                .override_sys_reset_dt
                .to_string();
        } else {
            override_sys_reset_dt = "N".to_string();
        }
        let mut output = get_op_line(
            &mut fields,
            &mut div,
            &mut alm_line,
            &mut ia_llg,
            &mut balm_llg,
            &mut t_ora_prod,
            &mut t_ora_gl,
            &mut t_ora_cat,
            &mut cost_center,
            &mut asset_class,
            &mut rt_cd,
            ccy,
            override_sys_reset_dt.to_string(),
            *config_param.as_on_date(),
            &bm_id,
            &log,
            &bm_id_lookup,
            val_date,
            mis2_code,
            &mut sprd_map,
            &mut npa_map,
            &mut od_study_map,
        );
        output_line.push_str(&output.processed_lines);
        output_line.push_str("|");
        output_line.push_str(&sma_flag);
        output_line.push_str("\n");
        if let Some(concat) = output.concat_lines.pop() {
            concats.push(concat);
        }

        let recon_key = ReconKey::new(ccy.to_string(), "FWOD".to_string(), fields[16].to_string());
        let amt: f64 = fields[5].parse().unwrap_or(DEFAULT_FLOAT);
        recon
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);

        tot_amt += amt;
        tot_acc_encntrd += 1;
        output.clear();
    }
    let end_derive_timer = SystemTime::now();
    let duration = end_derive_timer
        .duration_since(start_derive_timer)
        .expect("Could not calculate total derive process duration.");
    debug!(diag_log, "Derive Process Total Duration: {:?}.", duration);
    debug!(
        diag_log,
        "Field Derivation Total Duration: {:?}", total_field_derivation_duration
    );

    let start_write_timer = SystemTime::now();
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}`: {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    match writer.write_all(output_line.as_bytes()) {
        Ok(_) => println!("Successfully processed all accounts"),
        Err(error) => {
            panic!("Cannot pre process the input file: {:?}", error);
        }
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
            "CH378_NCB",
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
            "Unable to write reconcilation lines on file `{}`: {}.",
            config_param.rec_output_file_path(),
            error
        ),
    };

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
        .expect("Could not calculate total write process duration.");
    debug!(diag_log, "Write Process Total Duration: {:?}.", duration);

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        tot_acc_encntrd - lines_fld,
        lines_fld,
        tot_amt,
        tot_amt,
        DEFAULT_INT,
    );
    health_report.gen_health_rpt(&config_param.output_file_path());

    let report_string = format!(
        "Accounts encountered: {}\n\
         Accounts proccessed suceessfully: {}\n\
         Accounts failed to process: {}",
        tot_acc_encntrd,
        tot_acc_encntrd - lines_fld,
        lines_fld
    );
    info!(log, "{}", report_string);
    println!("{}", report_string);
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
