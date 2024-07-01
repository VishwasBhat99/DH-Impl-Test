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

    let header_rows = config_params
        .header_rows()
        .split(config_params.delimeter_type())
        .collect::<Vec<&str>>();
    let date_fields = config_params
        .date_fields()
        .split(config_params.delimeter_type())
        .collect::<Vec<&str>>();
    //Initialize the writer for BL output
    let mut investment_writer =
        match buf_file_wrtr(config_params.investment_output_file_path(), None) {
            Ok(investment_output_file) => investment_output_file,
            Err(error) => panic!(
                "Unable to create output file: `{}` on location `{}` : {}",
                config_params.investment_output_file_path(),
                current_dir()
                    .expect("Unable to get current directory path.")
                    .display(),
                error,
            ),
        };
    //Read the Base Input file
    log_debug!(log, "investment file File reading started");
    let investment_file = match new_buf_rdr(config_params.investment_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found investment_file: `{}` on location `{}` : {}.",
            config_params.investment_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in investment_file.lines().enumerate() {
        tot_rec += 1;
        if header_rows.contains(&((line_num + 1).to_string().as_str())) {
            log_debug!(log, "skipped record for header row_no:{}", line_num + 1);
            continue;
        }
        let investment_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.investment_file_path(),
                line_num + 1,
                error
            ),
        };
        let investment_fields = investment_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        succ_rec += 1;
        let mut output_line = String::new();
        //Initialization of vector to store all the maturity date , put and call date fields
        let mut mat_put_call_vec: Vec<NaiveDate> = Vec::new();
        for (index, investment_val) in investment_fields.iter().enumerate() {
            if date_fields.contains(&((index + 1).to_string().as_str())) {
                let current_date = investment_val;
                let curr_date_field = NaiveDate::parse_from_str(current_date, "%d-%b-%Y")
                    .unwrap_or(*config_params.as_on_date());
                // store the maturity date
                if index == 10 && !investment_val.is_empty() {
                    mat_put_call_vec.push(curr_date_field);
                }
                // store the put date
                if index == 52 && !investment_val.is_empty() {
                    mat_put_call_vec.push(curr_date_field);
                }
                // store the put date
                if index == 53 && !investment_val.is_empty() {
                    mat_put_call_vec.push(curr_date_field);
                }
                output_line.push_str(&curr_date_field.format("%d-%m-%Y").to_string());
            } else {
                output_line.push_str(investment_val.trim());
            }
            output_line.push_str(config_params.delimeter_type());
        }
        //For Computed Maturity Date
        mat_put_call_vec.sort();
        let mut computed_mat_date = config_params
            .def_comp_mat_date()
            .format("%d-%m-%Y")
            .to_string();
        if mat_put_call_vec.len() > 0 {
            computed_mat_date = mat_put_call_vec[0].format("%d-%m-%Y").to_string();
        }
        output_line.push_str(&computed_mat_date);
        output_line.push_str(config_params.delimeter_type());
        let cgl = bgl_cgl_map
            .get(&investment_fields[120][0..10].to_string())
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
        output_line.push_str(config_params.currency());

        let mat_dt_flag = if NaiveDate::parse_from_str(&computed_mat_date, "%d-%m-%Y")
            .unwrap_or(*config_params.def_comp_mat_date())
            < *config_params.as_on_date()
        {
            "Y"
        } else {
            "N"
        };
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(mat_dt_flag);

        let concat_deal_id = format!("{}-{}",investment_fields[4].trim(),investment_fields[0].trim());
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&concat_deal_id);
        let concat_inst_id = format!("{}-{}",investment_fields[2].trim(),investment_fields[0].trim());
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&concat_inst_id);
        let concat_deal_slr_id = format!("{}-{}",investment_fields[4].trim(),investment_fields[5].trim());
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&concat_deal_slr_id);
        let ftp_coupon_rt = investment_fields[11].trim().to_string().parse::<f64>().unwrap_or(investment_fields[12].trim().to_string().parse::<f64>().unwrap_or(0.0)).to_string();
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&ftp_coupon_rt);
        output_line.push_str(config_params.delimeter_type());
        //derived_coupon_rate:
        let instrument_type_data = config_params.instrument_type_data();
        if instrument_type_data.contains(&investment_fields[3].to_string()) {
            output_line.push_str(investment_fields[21]);
        }
        else {
            output_line.push_str(investment_fields[11]);
        }
        writeln!(investment_writer, "{}", output_line)
            .expect("pp invetment lending Output Line can not be written");
    }
    log_debug!(log, "Investment File Reading Completed");
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
    health_report.gen_health_rpt(config_params.investment_output_file_path());
}
