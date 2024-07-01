use self::derive_fields::get_op_line;
use self::reconcilation::ReconKey;
use self::split_pos::get_split_pos;
use calamine::{open_workbook, Reader, Xlsx};
use configuration_parameters::ConfigurationParameters;
use macros;
use sdb_io::{buf_file_wrtr, new_buf_rdr};
use slog::Logger;
use statics::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::time::{Duration, SystemTime};

mod derive_fields;
mod reconcilation;
mod split_pos;

pub fn process(config_param: ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let start_read_timer = SystemTime::now();
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
    if let Some(Ok(reader)) = ref_excel3.worksheet_range(config_param.alm_master_sheet_name()) {
        for row in reader.rows() {
            alm_line.insert(row[0].to_string(), row[6].to_string());
        }
    }

    let mut ref_excel4: Xlsx<_> = open_workbook(config_param.ref_file_path_4())
        .expect("Error while opening `FWCostCenter_OD.xlsx` file.");
    let mut cost_center: HashMap<String, String> = HashMap::new();
    if let Some(Ok(reader)) = ref_excel4.worksheet_range("OD") {
        for row in reader.rows() {
            cost_center.insert(row[0].to_string(), row[1].to_string());
        }
    }

    let mut npa_flg: Vec<String> = Vec::new();
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
                line_num,
                error
            ),
        };
        npa_flg.push(line.to_string());
    }
    let end_read_timer = SystemTime::now();
    let total_duration = end_read_timer
        .duration_since(start_read_timer)
        .expect("Could not calculate total duration.");
    debug!(
        diag_log,
        "Reading Reference Files Total Duration: {:?}", total_duration
    );

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

    let mut output_line = String::new();
    let mut recon: HashMap<ReconKey, f64> = HashMap::new();
    let mut tot_acc_encntrd = DEFAULT_INT;
    for (line_num, lines) in reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num,
                error
            ),
        };
        let split_val = get_split_pos();
        let mut fields: Vec<&str> = Vec::new();
        for val in split_val {
            let st_pos = val.st_pos;
            let end_pos = val.st_pos + val.length;
            let sub_string = line[st_pos..end_pos].trim();
            fields.push(sub_string);
        }
        output_line.push_str(&get_op_line(
            &mut fields,
            &mut div,
            &mut alm_line,
            &mut t_ora_prod,
            &mut t_ora_gl,
            &mut t_ora_cat,
            &mut cost_center,
            &mut npa_flg,
            *config_param.as_on_date(),
            &log,
        ));

        let ccy = match fields[3].parse::<i64>().unwrap_or(DEFAULT_INT) {
            1 => "INR",
            2 => "USD",
            _ => "FCY",
        };
        let recon_key = ReconKey::new(ccy.to_string(), "FWOD".to_string(), fields[16].to_string());
        let amt: f64 = fields[5].parse().unwrap_or(DEFAULT_FLOAT);
        recon
            .entry(recon_key)
            .and_modify(|val| *val += amt)
            .or_insert(amt);

        tot_acc_encntrd += 1;
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

    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total write process duration.");
    debug!(diag_log, "Write Process Total Duration: {:?}.", duration);

    log_info!(log, "Accounts encountered: {}", tot_acc_encntrd);
    println!("Accounts encountered: {}", tot_acc_encntrd);
}
