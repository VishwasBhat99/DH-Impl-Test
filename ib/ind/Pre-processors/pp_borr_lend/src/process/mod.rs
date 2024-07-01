use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use calamine::{open_workbook_auto, Reader};
use core::str;
use hashbrown::HashMap;
use health_report::HealthReport;
use rbdate::NaiveDate;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
pub struct MasterData {
    group: String,
    llg: String,
}
pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut succ_rec = 0;
    //Mapping master File reading started
    log_debug!(log, "Mapping master File reading started");
    let mut map_master_map: HashMap<String, MasterData> = HashMap::new();
    let mut map_master_excel = open_workbook_auto(config_params.map_master_file_path())
        .expect("Unable to open Mapping Master File.");
    if let Some(Ok(reader)) = map_master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(0) {
            let gl_acc_no = row[0].to_string();
            let data = MasterData {
                group: row[3].to_string(),
                llg: row[4].to_string(),
            };
            map_master_map.insert(gl_acc_no, data);
        }
    }
    log_debug!(log, "Mapping Master File Reading Completed");
    //Mapping master File reading is Completed
    //BGL CGL File reading is started
    log_debug!(log, "BGL CGL File reading started");
    let bgl_cgl_file = match new_buf_rdr(config_params.bgl_cgl_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found bgl_cgl_file: `{}` on location `{}` : {}.",
            config_params.bgl_cgl_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut bgl_cgl_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in bgl_cgl_file.lines().enumerate() {
        let bgl_cgl_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bgl_cgl_file_path(),
                line_num + 1,
                error
            ),
        };
        let bgl_cgl_fields = bgl_cgl_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        bgl_cgl_map.insert(bgl_cgl_fields[0].to_string(), bgl_cgl_fields[1].to_string());
    }
    log_debug!(log, "BGL CGL File Reading is Completed");
    //BGL CGL File reading is Completed
    // Read the BG CF master file
    log_debug!(log, "BG CF File Reading has started");
    let bl_cf_file = match new_buf_rdr(config_params.bl_cf_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found bl_cf_file: `{}` on location `{}` : {}.",
            config_params.bl_cf_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut bl_cf_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for (line_num, lines) in bl_cf_file.lines().enumerate() {
        let bl_cf_input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bl_cf_file_path(),
                line_num + 1,
                error
            ),
        };
        let mut bl_cf_inp_fields: Vec<String> = bl_cf_input_line
            .split(config_params.delimeter_type())
            .map(str::to_string)
            .collect();
        let deal_ref = bl_cf_inp_fields[1].to_string();
        bl_cf_inp_fields.drain(0..2);
        for index in 0..bl_cf_inp_fields.len() {
            //Get the absolute value of cashflow amount
            if index == 2 {
                let abs_cf_amount: f64 = bl_cf_inp_fields[index].parse().unwrap_or(0.0);
                bl_cf_inp_fields[index] = abs_cf_amount.abs().to_string();
            } else if index == 3 || index == 4 {
                bl_cf_inp_fields[index] =
                    NaiveDate::parse_from_str(bl_cf_inp_fields[index].trim(), "%d/%m/%Y")
                        .unwrap_or(*config_params.as_on_date())
                        .format("%d-%m-%Y")
                        .to_string();
            } else {
                bl_cf_inp_fields[index] = bl_cf_inp_fields[index].trim().to_string();
            }
        }
        bl_cf_map
            .entry(deal_ref)
            .or_insert(Vec::new())
            .push(bl_cf_inp_fields);
    }

    log_debug!(log, "BG CF File Reading has Completed");
    // BG CF Master file reading is Completed
    let header_rows = config_params
        .header_rows()
        .split(config_params.delimeter_type())
        .collect::<Vec<&str>>();
    let date_fields = config_params
        .date_fields()
        .split(config_params.delimeter_type())
        .collect::<Vec<&str>>();
    //Initialize the writer for BL output
    let mut bl_writer = match buf_file_wrtr(config_params.bl_output_file_path(), None) {
        Ok(bl_output_file) => bl_output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.bl_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    //Read the Base Input file
    log_debug!(log, "BL Master file File reading started");
    let bl_master_file = match new_buf_rdr(config_params.bl_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found bl_master_file: `{}` on location `{}` : {}.",
            config_params.bl_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in bl_master_file.lines().enumerate() {
        tot_rec += 1;
        if header_rows.contains(&((line_num + 1).to_string().as_str())) {
            log_debug!(log, "skipped record for header row_no:{}", line_num + 1);
            continue;
        }
        let bl_master_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.bl_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let bl_master_fields = bl_master_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        succ_rec += 1;
        let mut output_line = String::new();
        for (index, bl_master_val) in bl_master_fields.iter().enumerate() {
            if date_fields.contains(&((index + 1).to_string().as_str())) {
                let current_date = bl_master_val;
                let curr_date_field = NaiveDate::parse_from_str(current_date, "%d-%b-%Y")
                    .unwrap_or(*config_params.as_on_date())
                    .format("%d-%m-%Y");
                output_line.push_str(&curr_date_field.to_string());
            } else {
                output_line.push_str(bl_master_val.trim());
            }
            output_line.push_str(config_params.delimeter_type());
        }
        let cgl = bgl_cgl_map
            .get(&bl_master_fields[20][0..10].to_string())
            .unwrap_or(&"CGL".to_string())
            .to_string();
        output_line.push_str(&cgl);
        output_line.push_str(config_params.delimeter_type());
        let default_master_val = MasterData {
            group: "NA".to_string(),
            llg: "NA".to_string(),
        };
        let map_master_val = map_master_map.get(&cgl).unwrap_or(&default_master_val);
        output_line.push_str(&map_master_val.group);
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&map_master_val.llg);
        output_line.push_str(config_params.delimeter_type());
        let deal_amount_diff: f64 =
            bl_master_fields[7].parse().unwrap_or(0.0) - bl_master_fields[6].parse().unwrap_or(0.0);
        let default_cf_date = NaiveDate::parse_from_str(bl_master_fields[5], "%d-%b-%Y")
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y");
        // Get the absolute amount of cashflow amount(deal_amount_diff)
        let cf_default_vec: Vec<Vec<String>> = vec![vec![
            "Principal".to_string(),
            bl_master_fields[19].to_string(),
            deal_amount_diff.abs().to_string(),
            default_cf_date.to_string(),
            default_cf_date.to_string(),
        ]];
        let bl_cf_vec = bl_cf_map
            .get(&bl_master_fields[0][3..].to_string())
            .unwrap_or(&cf_default_vec);

        for (_index, val) in bl_cf_vec.iter().enumerate() {
            let mut curr_output_line = output_line.clone();
            curr_output_line.push_str(&val.to_vec().join(config_params.delimeter_type()));
            writeln!(bl_writer, "{}", curr_output_line)
                .expect("pp borrow lending Output Line can not be written");
        }
    }
    log_debug!(log, "BL Master File Reading Completed");
    //Base Input file reading is completed.
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    log_debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.",
        duration
    );
    let health_report = HealthReport::new(tot_rec, succ_rec, tot_rec - succ_rec, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_params.bl_output_file_path());
}
