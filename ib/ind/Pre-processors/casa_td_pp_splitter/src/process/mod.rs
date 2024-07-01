use crate::configuration_parameters::ConfigurationParameters;
use crate::macros;
use base64::{decode, encode};
use calamine::{open_workbook_auto, Reader};
use core::str;
use hashbrown::HashMap;
use health_report::HealthReport;
use rbdate::datevalue_to_naive_date;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::SystemTime;
pub fn process(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_process_timer = SystemTime::now();
    let mut tot_rec = 0;
    let mut td_processed = 0;
    let mut sa_processed = 0;
    let mut ca_processed = 0;

    //Mapping master File reading started
    log_debug!(log, "Mapping master File reading started");
    println!("Mapping master File reading started");
    let mut map_master_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut map_master_excel = open_workbook_auto(config_params.map_master_file_path())
        .expect("Unable to open Mapping Master File.");
    if let Some(Ok(reader)) = map_master_excel.worksheet_range(config_params.sheet_name()) {
        for row in reader.rows().skip(0) {
            let gl_acc_no = row[0].to_string();
            let clsn = row[2].to_string();
            let group = row[3].to_string();
            let llg = row[4].to_string();
            map_master_map.insert(gl_acc_no, [clsn, group, llg].to_vec());
        }
    }
    log_debug!(log, "Mapping Master File Reading Completed");
    println!("Mapping master File reading Completed");
    //Mapping master File reading is Completed
    //TD Identifier File reading is started
    log_debug!(log, "CASATD Identifier File reading started");
    println!("CASATD Identifier File reading started");
    let casatd_identifier_file = match new_buf_rdr(config_params.casatd_identifier_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found casatd_identifier_file: `{}` on location `{}` : {}.",
            config_params.casatd_identifier_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut casatd_identifier_map: HashMap<String, Vec<String>> = HashMap::new();
    for (line_num, lines) in casatd_identifier_file.lines().enumerate() {
        let casatd_identifier_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.casatd_identifier_file_path(),
                line_num + 1,
                error
            ),
        };
        let casatd_identifier_fields = casatd_identifier_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        let group = casatd_identifier_fields[0]
            .trim()
            .to_lowercase()
            .to_string();
        let source = casatd_identifier_fields[1]
            .trim()
            .to_uppercase()
            .to_string();
        casatd_identifier_map
            .entry(source)
            .and_modify(|data| data.push(group.to_string()))
            .or_insert(vec![group.to_string()]);
    }
    log_debug!(log, "CASATD Identifier File Reading is Completed");

    //TD Identifier File reading is Completed
    println!("TD Identifier File reading Completed");
    let date_fields = config_params
        .date_fields()
        .split(config_params.delimeter_type())
        .collect::<Vec<&str>>();
    let header_rows = config_params
        .header_rows()
        .split(config_params.delimeter_type())
        .collect::<Vec<&str>>();
    let base_input_file = match new_buf_rdr(config_params.base_input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found base_input_file: `{}` on location `{}` : {}.",
            config_params.base_input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    // Read the Sb Intrate master file
    log_debug!(log, "Sb intrate File Reading has started");
    println!("Sb intrate File Reading started");
    let sb_intrate_file = match new_buf_rdr(config_params.sb_intrate_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found sb_intrate_file: `{}` on location `{}` : {}.",
            config_params.sb_intrate_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut sb_intrate_map: HashMap<String, Vec<String>> = HashMap::new();
    for (line_num, lines) in sb_intrate_file.lines().enumerate() {
        let sb_intrate_input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.sb_intrate_file_path(),
                line_num + 1,
                error
            ),
        };
        let sb_intrate_inp_fields = sb_intrate_input_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        let mut sb_intrate_map_val: Vec<String> = Vec::new();
        let cgl = sb_intrate_inp_fields[0].to_string();
        for (index, _) in sb_intrate_inp_fields.iter().enumerate() {
            if index == 0 {
                continue;
            } else {
                sb_intrate_map_val.push(sb_intrate_inp_fields[index].to_string());
            }
        }
        sb_intrate_map.insert(cgl, sb_intrate_map_val);
    }
    log_debug!(log, "Sb intrate File Reading has Completed");
    println!("Sb intrate File Reading Completed");
    // Sb intrate Master file reading is Completed
    // Read the Customer master file
    log_debug!(log, "Customer File Reading has started");
    println!("Customer File Reading started");
    let cust_master_file = match new_buf_rdr(config_params.cust_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found cust_master_file: `{}` on location `{}` : {}.",
            config_params.cust_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut cust_master_map: HashMap<i64, String> = HashMap::new();
    for (line_num, lines) in cust_master_file.lines().enumerate() {
        let cust_input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.cust_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let cust_input_fields = cust_input_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        let mut cust_map_val: String = "".to_string();
        let cust_acc_no: i64 = cust_input_fields[1].parse().unwrap_or(0);
        for (index, _) in cust_input_fields.iter().enumerate().take(8) {
            if index == 1 {
                continue;
            } else {
                cust_map_val.push_str(cust_input_fields[index]);
                if index < 7 {
                    cust_map_val.push_str(config_params.delimeter_type());
                }
            }
        }
        let compressed_cust_val = encode(cust_map_val);
        cust_master_map.insert(cust_acc_no, compressed_cust_val);
    }
    log_debug!(log, "Customer File Reading has Completed");
    println!("Customer File Reading Completed");
    // Customer Master file reading is Completed
    //Initialize the writer for TD
    let mut td_writer = match buf_file_wrtr(config_params.td_output_file_path(), None) {
        Ok(td_output_file) => td_output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.td_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    //Initialize the writer for CA
    let mut ca_writer = match buf_file_wrtr(config_params.ca_output_file_path(), None) {
        Ok(ca_output_file) => ca_output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.ca_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    //Initialize the writer for SA
    let mut sa_writer = match buf_file_wrtr(config_params.sa_output_file_path(), None) {
        Ok(sa_output_file) => sa_output_file,
        Err(error) => panic!(
            "Unable to create output file: `{}` on location `{}` : {}",
            config_params.sa_output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error,
        ),
    };
    //Read the Base Input file
    log_debug!(log, "Base Input File reading started");
    println!("Base Input File Reading started");
    for (line_num, lines) in base_input_file.lines().enumerate() {
        tot_rec += 1;
        if header_rows.contains(&((line_num + 1).to_string().as_str())) {
            log_debug!(log, "skipped record for header KEY_1:{}", line_num + 1);
            continue;
        }
        let base_input_line = match lines {
            Ok(input_line) => input_line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.base_input_file_path(),
                line_num + 1,
                error
            ),
        };
        let base_input_fields = base_input_line
            .split(config_params.delimeter_type())
            .collect::<Vec<&str>>();
        let gl_class_code = base_input_fields[25];
        let mut gl_code = "NA".to_string();
        if gl_class_code.len() < 18 {
            log_debug!(
                log,
                "The Account {} has GL Class Code length less than 18",
                base_input_fields[0]
            );
        } else {
            gl_code = gl_class_code[8..18].to_string();
        }
        //Logic for TD and CASA
        let cust_no = base_input_fields[7].parse().unwrap_or(0);
        let mut map_master_val = ["L".to_string(), "NA".to_string(), "NA".to_string()].to_vec();
        let mut group = "na".to_string();
        if map_master_map.contains_key(&gl_code) {
            map_master_val = map_master_map
                .get(&gl_code)
                .expect("GL Code is not found in Mapping Master file")
                .to_vec();
            group = map_master_val[1].trim().to_string().to_lowercase();
        }
        //For write in TD File
        let is_td_account = casatd_identifier_map
            .get("TD")
            .expect("Could not get group for TD source!")
            .contains(&group);
        let is_sa_account = casatd_identifier_map
            .get("SA")
            .expect("Could not get group for SA source!")
            .contains(&group);
        let mut output_line = "".to_string();
        for index in 0..34 {
            if !is_td_account && (index == 12 || index == 20 || index == 21 || index == 33) {
                continue;
            }
            if !is_td_account && index == 24 {
                output_line.push_str(base_input_fields[20]);
                output_line.push_str(config_params.delimeter_type());
                continue;
            }
            if date_fields.contains(&((index + 1).to_string().as_str())) {
                let mut current_date: i64 =
                    base_input_fields[index].parse::<i64>().unwrap_or(0) + 1;
                if !(0..=99999).contains(&current_date) {
                    current_date = 366;
                }
                let date = datevalue_to_naive_date(&current_date.to_string())
                    .unwrap_or(*config_params.as_on_date())
                    .format("%d-%m-%Y");
                output_line.push_str(&date.to_string());
            } else {
                output_line.push_str(base_input_fields[index]);
            }
            output_line.push_str(config_params.delimeter_type());
        }

        if cust_master_map.contains_key(&cust_no) {
            let comp_cust_val: &String = cust_master_map
                .get(&cust_no)
                .expect("cust_no is not found in cust master file");
            let decomp_val = decode(comp_cust_val).expect("Error in decoding!");
            let cust_val_str = std::str::from_utf8(&decomp_val).expect("Error in utf8 conversion!");
            let cust_val = cust_val_str
                .split(config_params.delimeter_type())
                .collect::<Vec<&str>>();
            for (index, _) in cust_val.iter().enumerate().take(7) {
                //to Push customer no. into the output
                if index == 1 {
                    output_line.push_str(&cust_no.to_string());
                    output_line.push_str(config_params.delimeter_type());
                    output_line.push_str(cust_val[index]);
                } else {
                    output_line.push_str(cust_val[index]);
                }
                output_line.push_str(config_params.delimeter_type());
            }
        } else {
            log_debug!(
                log,
                "The Customer Number {} is not present in customer master file For Account no: {}",
                cust_no.to_string(),
                base_input_fields[0]
            );
            for _index in 0..8 {
                output_line.push_str("NA");
                output_line.push_str(config_params.delimeter_type());
            }
        }
        output_line.push_str(&map_master_val[1]);
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&map_master_val[2]);
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&map_master_val[0][0..1]);
        output_line.push_str(config_params.delimeter_type());
        let product_code = format!("{}{}", base_input_fields[3], base_input_fields[4],);
        output_line.push_str(&product_code);
        output_line.push_str(config_params.delimeter_type());
        for index in 37..base_input_fields.len() {
            if is_td_account {
                if index == 37 {
                    output_line.push_str(
                        &(base_input_fields[19].parse::<i64>().unwrap_or(0)
                            - base_input_fields[13].parse::<i64>().unwrap_or(0))
                        .to_string(),
                    );
                    output_line.push_str(config_params.delimeter_type());
                    continue;
                }
                if index == 38 {
                    output_line.push_str(
                        &(base_input_fields[19].parse::<i64>().unwrap_or(0)
                            - base_input_fields[21].parse::<i64>().unwrap_or(0))
                        .to_string(),
                    );
                    output_line.push_str(config_params.delimeter_type());
                    continue;
                }
            }
            output_line.push_str(base_input_fields[index]);
            if index < base_input_fields.len() - 1 {
                output_line.push_str(config_params.delimeter_type());
            }
        }
        //For GLCODE
        output_line.push_str(config_params.delimeter_type());
        if gl_class_code.len() < 18 {
            log_debug!(log,"Error (GL_class_code {} len is less then 18) while getting GL Code for Account - {}",gl_class_code,base_input_fields[0]);
            output_line.push_str("NA");
        } else {
            output_line.push_str(&gl_code);
        }
        //For int_rate
        output_line.push_str(config_params.delimeter_type());
        if is_td_account {
            if !base_input_fields[23].is_empty() {
                //24->CR_STORE_Rate
                output_line.push_str(base_input_fields[23]);
            } else if !base_input_fields[17].is_empty() {
                //18->VAR_INT_RATE
                output_line.push_str(base_input_fields[17]);
            } else {
                output_line.push_str(base_input_fields[24]);
            }
        } else {
            let sb_intrate_dft_val = [
                "CR_ID_DET".to_string(),
                "0.00".to_string(),
                "0.00".to_string(),
                "0.00".to_string(),
                "0.00".to_string(),
            ]
            .to_vec();
            let sb_intrate_val = sb_intrate_map.get(&gl_code).unwrap_or(&sb_intrate_dft_val);
            let amount = base_input_fields[9].parse().unwrap_or(0.00);
            if (0.0..=1000000.00).contains(&amount) {
                output_line.push_str(&sb_intrate_val[2]);
            } else if amount > 1000000.00 && amount <= 1999999999.99 {
                output_line.push_str(&sb_intrate_val[3]);
            } else if amount >= 2000000000.00 {
                output_line.push_str(&sb_intrate_val[4]);
            }
        }
        //For CURR_BAL_LCY
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str("");
        //For As_on_Date
        output_line.push_str(config_params.delimeter_type());
        output_line.push_str(&(config_params.as_on_date().format("%d-%m-%Y").to_string()));
        if is_td_account {
            td_processed += 1;
            writeln!(td_writer, "{}", output_line).expect("TD Output Line can not be written");
        } else if is_sa_account {
            sa_processed += 1;
            writeln!(sa_writer, "{}", output_line).expect("SA Output Line can not be written");
        } else {
            ca_processed += 1;
            writeln!(ca_writer, "{}", output_line).expect("CA Output Line can not be written");
        }
    }

    log_debug!(log, "Base Input File Reading Completed");
    println!("Base Input File Reading Completed");
    let end_process_timer = SystemTime::now();
    let duration = end_process_timer
        .duration_since(start_process_timer)
        .expect("Could not calculate total duration for the process.");
    log_debug!(
        diag_log,
        "Total Duration for Reading and Writing Records: {:?}.",
        duration
    );
    let health_report_td =
        HealthReport::new(tot_rec, td_processed, tot_rec - td_processed, 0.0, 0.0, 0);
    health_report_td.gen_health_rpt(config_params.td_output_file_path());
    let health_report_sa =
        HealthReport::new(tot_rec, sa_processed, tot_rec - sa_processed, 0.0, 0.0, 0);
    health_report_sa.gen_health_rpt(config_params.sa_output_file_path());
    let health_report_ca =
        HealthReport::new(tot_rec, ca_processed, tot_rec - ca_processed, 0.0, 0.0, 0);
    health_report_ca.gen_health_rpt(config_params.ca_output_file_path());
}
